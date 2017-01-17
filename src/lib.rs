#[derive(Debug)]
pub struct PatriciaTree {
    prefix: String,
    is_leaf: bool,
    children: Vec<Box<PatriciaTree>>,
}

fn slice_to_string(s: &[char]) -> String {
    s.to_vec().into_iter().collect()
}

fn split_charvec_str(s: &Vec<char>, at: usize) -> (String, String) {
    let (p, s) = s.split_at(at);
    (slice_to_string(p), slice_to_string(s))
}

impl PatriciaTree {
    pub fn new() -> PatriciaTree {
        PatriciaTree {
            prefix: String::new(),
            is_leaf: false,
            children: vec![],
        }
    }

    fn box_with(prefix: &str,
                is_leaf: bool,
                children: Vec<Box<PatriciaTree>>)
                -> Box<PatriciaTree> {
        assert!(prefix != "");
        Box::new(PatriciaTree {
            prefix: prefix.to_owned(),
            is_leaf: is_leaf,
            children: children,
        })
    }

    fn add_child(&mut self, child: Box<PatriciaTree>) {
        match self.children.binary_search_by(|c| c.cmp_first_char(&child.prefix)) {
            Err(p) => self.children.insert(p, child),
            _ => unreachable!(),
        }
    }

    fn cmp_first_char(&self, s: &str) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self.prefix.chars().next(), s.chars().next()) {
            (Some(c1), Some(c2)) => c1.cmp(&c2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }

    pub fn exist(&self, s: &str) -> bool {
        let mut prefix = self.prefix.chars();
        let mut s = s.chars();

        loop {
            enum IteratingState {
                Continue,
                Result(bool),
            }

            let st = match (prefix.next(), s.next()) {
                (Some(p), Some(c)) => {
                    if p == c {
                        IteratingState::Continue
                    } else {
                        IteratingState::Result(false)
                    }
                }
                (Some(_), None) => IteratingState::Result(false),
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    let recursive_result = match self.children
                        .binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                        Ok(child_idx) => self.children[child_idx].exist(&s_suffix),
                        Err(_) => false,
                    };
                    IteratingState::Result(recursive_result)
                }
                (None, None) => IteratingState::Result(self.is_leaf),
            };

            match st {
                IteratingState::Continue => {}
                IteratingState::Result(b) => {
                    return b;
                }
            }
        }
    }

    pub fn add(&mut self, s: &str) {
        let mut c_idx: usize = 0;
        let prefix: Vec<char> = self.prefix.chars().collect();
        let mut s = s.chars();

        loop {
            enum IteratingState {
                Continue,
                Finished,
            }

            let st = match (prefix.get(c_idx), s.next()) {
                (Some(&p), Some(c)) if p == c => IteratingState::Continue,
                (Some(_), Some(c)) /* p != c */ => {
                    let (p_prefix, p_suffix) = split_charvec_str(&prefix, c_idx);
                    let s_suffix = format!("{}{}", c, s.as_str());

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suffix, self.is_leaf, child_children);
                    self.add_child(child);
                    self.add_child(PatriciaTree::box_with(&s_suffix, true, vec![]));

                    self.prefix = p_prefix;
                    self.is_leaf = false;

                    IteratingState::Finished
                }
                (Some(_), None) => {
                    let (p_prefix, p_suffix) = split_charvec_str(&prefix, c_idx);

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suffix, self.is_leaf, child_children);
                    self.add_child(child);

                    self.prefix = p_prefix;
                    self.is_leaf = true;

                    IteratingState::Finished
                }
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                        Ok(child_idx) => {
                            self.children[child_idx].add(&s_suffix);
                        }
                        Err(_) => {
                            self.add_child(PatriciaTree::box_with(&s_suffix, true, vec![]));
                        }
                    }

                    IteratingState::Finished
                }
                (None, None) => {
                    self.is_leaf = true;
                    IteratingState::Finished
                }
            };

            match st {
                IteratingState::Continue => {
                    c_idx += 1;
                    assert!(c_idx <= self.prefix.len());
                }
                IteratingState::Finished => {
                    return;
                }
            }
        }
    }

    pub fn remove(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }

        let mut c_idx: usize = 0;
        let prefix: Vec<char> = self.prefix.chars().collect();
        let mut s = s.chars();

        loop {
            enum IteratingState {
                Continue,
                Finished,
            }

            let st = match (prefix.get(c_idx), s.next()) {
                (Some(&p), Some(c)) if p != c => IteratingState::Finished,
                (Some(_), None) => IteratingState::Finished,
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                        Ok(child_idx) => {
                            self.children[child_idx].remove(&s_suffix);
                        }
                        Err(_) => {}
                    }

                    IteratingState::Finished
                }
                (None, None) => {
                    match self.children.len() {
                        0 => {
                            // assert!(self.is_leaf); FIXME
                            self.is_leaf = false;
                        }
                        1 => {
                            assert!(self.is_leaf);
                            // assert!(self.children[0].is_leaf); FIXME
                            self.prefix.push_str(&self.children[0].prefix);
                            self.is_leaf = self.children[0].is_leaf; // FIXME
                            self.children.clear();
                        }
                        _ => {
                            self.is_leaf = false;
                        }
                    }

                    IteratingState::Finished
                }
                _ => IteratingState::Continue,
            };

            match st {
                IteratingState::Continue => {
                    c_idx += 1;
                    assert!(c_idx <= self.prefix.len());
                }
                IteratingState::Finished => {
                    return;
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        let leaf_cnt: usize = if self.is_leaf { 1 } else { 0 };
        let children_cnt: usize = self.children.iter().map(|c| c.size()).sum();
        leaf_cnt + children_cnt
    }
}

use std::fmt::Display;

impl Display for PatriciaTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const INDENT_WIDTH: usize = 2;

        fn fmt_r(f: &mut std::fmt::Formatter, tree: &PatriciaTree, indent: usize) {
            for _ in 0..indent {
                write!(f, " ").unwrap();
            }
            writeln!(f,
                     "- {} {}",
                     tree.prefix,
                     if tree.is_leaf { "[leaf]" } else { "" })
                .unwrap();
            for c in &tree.children {
                fmt_r(f, c, indent + INDENT_WIDTH);
            }
        };

        writeln!(f, "- (root)").unwrap();
        for c in &self.children {
            fmt_r(f, c, INDENT_WIDTH);
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_size_test() {
        let mut root = PatriciaTree::new();
        let mut expected_size = 0;
        assert_eq!(root.size(), expected_size);

        macro_rules! test {
            ($s: expr) => {{
                println!("\nAdding \"{}\"...", $s);
                root.add($s);
                expected_size += 1;
                println!("{:#?}", root);
                assert_eq!(root.size(), expected_size);
            }}
        };

        test!("test");
        test!("root");
        test!("tea");
        test!("rooter");
        test!("roast");
        test!("teapot");
    }

    #[test]
    fn add_exist_test() {
        let mut root = PatriciaTree::new();
        assert!(!root.exist("test"));

        macro_rules! test {
            ($s: expr) => {{
                println!("\nAdding \"{}\"...", $s);
                root.add($s);
                println!("{:#?}", root);
                assert!(root.exist($s));
            }}
        };

        test!("root");
        test!("tea");
        test!("roast");
        test!("rooter");
        test!("test");
        test!("teapot");

        assert!(!root.exist("te"));
        assert!(!root.exist("ro"));
    }

    #[test]
    fn remove_test() {
        let mut root = PatriciaTree::new();
        root.add("test");
        root.add("tea");
        root.add("teapot");
        root.add("root");
        root.add("rooter");
        root.add("roast");

        println!("{:#?}", root);

        macro_rules! test {
            ($s: expr) => {{
                assert!(root.exist($s));
                println!("\nRemoving \"{}\"...", $s);
                root.remove($s);
                println!("{:#?}", root);
                assert!(!root.exist($s));
            }}
        };

        test!("teapot");
        test!("roast");
        test!("root");
        test!("test");
        test!("tea");
        test!("rooter");
    }
}
