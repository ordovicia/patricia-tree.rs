pub struct PatriciaTree {
    prefix: String,
    is_leaf: bool,
    children: Vec<Box<PatriciaTree>>,
}

fn slice_to_string(s: &[char]) -> String {
    s.to_vec().into_iter().collect()
}

fn split_charvec_str1(s: &Vec<char>, at: usize) -> String {
    slice_to_string(s.split_at(at).1)
}

fn split_charvec_str(s: &Vec<char>, at: usize) -> (String, String) {
    let (p, s) = s.split_at(at);
    (slice_to_string(p), slice_to_string(s))
}

fn cmp_first_char(s1: &str, s2: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match (s1.chars().next(), s2.chars().next()) {
        (Some(c1), Some(c2)) => c1.cmp(&c2),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
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
        match self.children.binary_search_by(|c| cmp_first_char(&c.prefix, &child.prefix)) {
            Err(p) => self.children.insert(p, child),
            _ => {}
        }
    }

    pub fn exist(&self, s: &str) -> bool {
        let mut c_idx: usize = 0;
        let mut prefix = self.prefix.chars();
        let s: Vec<char> = s.chars().collect();

        loop {
            enum IteratingState {
                Continue,
                Result(bool),
            }

            let st = match (prefix.next(), s.get(c_idx)) {
                (Some(p), Some(&c)) => {
                    if p == c {
                        IteratingState::Continue
                    } else {
                        IteratingState::Result(false)
                    }
                }
                (Some(_), None) => IteratingState::Result(false),
                (None, Some(_)) => {
                    let s_suf = split_charvec_str1(&s, c_idx);
                    let result = self.children.iter().any(|c| c.exist(&s_suf));
                    IteratingState::Result(result)
                }
                (None, None) => IteratingState::Result(self.is_leaf),
            };

            match st {
                IteratingState::Continue => {
                    c_idx += 1;
                }
                IteratingState::Result(b) => {
                    return b;
                }
            }
        }
    }

    pub fn add(&mut self, s: &str) {
        let mut c_idx: usize = 0;
        let prefix: Vec<char> = self.prefix.chars().collect();
        let s: Vec<char> = s.chars().collect();

        loop {
            enum IteratingState {
                Continue,
                Finished,
            }

            let st = match (prefix.get(c_idx), s.get(c_idx)) {
                (Some(p), Some(c)) if p == c => IteratingState::Continue,
                (Some(_), Some(_)) /* p != c */ => {
                    let (p_pre, p_suf) = split_charvec_str(&prefix, c_idx);
                    let s_suf = split_charvec_str1(&s, c_idx);

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suf, self.is_leaf, child_children);
                    self.add_child(child);
                    self.add_child(PatriciaTree::box_with(&s_suf, true, vec![]));

                    self.prefix = p_pre;
                    self.is_leaf = false;

                    IteratingState::Finished
                }
                (Some(_), None) => {
                    let (p_pre, p_suf) = split_charvec_str(&prefix, c_idx);

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suf, self.is_leaf, child_children);
                    self.add_child(child);

                    self.prefix = p_pre;
                    self.is_leaf = true;

                    IteratingState::Finished
                }
                (None, Some(_)) => {
                    let s_suf = split_charvec_str1(&s, c_idx);
                    match self.children.binary_search_by(|c| cmp_first_char(&c.prefix, &s_suf)) {
                        Ok(child_pos) => {
                            self.children[child_pos].add(&s_suf);
                        }
                        Err(_) => {
                            self.add_child(PatriciaTree::box_with(&s_suf, true, vec![]));
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
                    assert!(c_idx <= s.len());
                }
                IteratingState::Finished => {
                    return;
                }
            }
        }
    }

    pub fn remove(&mut self, s: &str) {
        let mut c_idx: usize = 0;
        let prefix: Vec<char> = self.prefix.chars().collect();
        let s: Vec<char> = s.chars().collect();

        loop {
            enum IteratingState {
                Continue,
                Finished,
            }

            let st = match (prefix.get(c_idx), s.get(c_idx)) {
                (Some(p), Some(c)) if p != c => IteratingState::Finished,
                (Some(_), None) => IteratingState::Finished,
                (None, Some(_)) => {
                    let s_suf = split_charvec_str1(&s, c_idx);
                    for c in &mut self.children {
                        c.remove(&s_suf);
                    }
                    IteratingState::Finished
                }
                (None, None) => {
                    match self.children.len() {
                        0 => {
                            // assert!(self.is_leaf); TODO
                            self.is_leaf = false;
                        }
                        1 => {
                            // assert!(self.is_leaf); TODO
                            // assert!(self.children[0].is_leaf); TODO
                            self.prefix.push_str(&self.children[0].prefix);
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
                    assert!(c_idx <= s.len());
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

#[cfg(test)]
mod tests {
    use super::*;

    fn print(root: &PatriciaTree) {
        println!("\n==> PatriciaTree <==");
        print_r(root, 0);
    }

    fn print_r(tree: &PatriciaTree, indent: i32) {
        for _ in 0..indent {
            print!(" ");
        }

        print!("|- \"{}\" {}",
               tree.prefix,
               if tree.is_leaf { "[leaf]" } else { "" });
        for c in &tree.children {
            print_r(c, indent + 2);
        }
    }

    #[test]
    fn add_size_test() {
        let mut root = PatriciaTree::new();
        let mut expected_size = 0;
        assert_eq!(root.size(), expected_size);

        {
            let mut test = |s| {
                root.add(s);
                expected_size += 1;
                assert_eq!(root.size(), expected_size);
            };

            test("test");
            test("root");
            test("tea");
            test("rooter");
            test("roast");
            test("teapot");
        }

        print(&root);
    }

    #[test]
    fn add_exist_test() {
        let mut root = PatriciaTree::new();
        assert!(!root.exist("test"));

        {
            let mut test = |s| {
                root.add(s);
                assert!(root.exist(s));
            };

            test("root");
            test("tea");
            test("roast");
            test("rooter");
            test("test");
            test("teapot");
        }

        assert!(!root.exist("te"));
        assert!(!root.exist("ro"));

        print(&root);
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

        print(&root);

        {
            let mut test = |s| {
                assert!(root.exist(s));
                root.remove(s);
                print(&root);
                assert!(!root.exist(s));
            };

            test("teapot");
            test("roast");
            test("root");
            test("test");
            test("tea");
            test("rooter");
        }

        print(&root);
    }
}
