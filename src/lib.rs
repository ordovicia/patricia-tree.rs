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
        Box::new(PatriciaTree {
            prefix: prefix.to_owned(),
            is_leaf: is_leaf,
            children: children,
        })
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

    fn push_child(&mut self, child: Box<PatriciaTree>) {
        match self.children.binary_search_by(|c| cmp_first_char(&c.prefix, &child.prefix)) {
            Err(p) => self.children.insert(p, child),
            _ => {}
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
                (Some(_), Some(_)) => {
                    let (p_pre, p_suf) = split_charvec_str(&prefix, c_idx);
                    let s_suf = split_charvec_str1(&s, c_idx);

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suf, self.is_leaf, child_children);
                    self.push_child(child);
                    self.push_child(PatriciaTree::box_with(&s_suf, true, vec![]));

                    self.prefix = p_pre;
                    self.is_leaf = false;

                    IteratingState::Finished
                }
                (Some(_), None) => {
                    let (p_pre, p_suf) = split_charvec_str(&prefix, c_idx);

                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suf, self.is_leaf, child_children);
                    self.push_child(child);
                    self.prefix = p_pre;
                    self.is_leaf = true;

                    IteratingState::Finished
                }
                (None, Some(_)) => {
                    let s_suf = split_charvec_str1(&s, c_idx);
                    match self.children
                        .binary_search_by(|c| cmp_first_char(&c.prefix, &s_suf)) {
                        Ok(child_pos) => {
                            self.children[child_pos].add(&s_suf);
                        }
                        Err(_) => {
                            self.push_child(PatriciaTree::box_with(&s_suf, true, vec![]));
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
                        0 => {} // TODO
                        1 => {}
                        2 => {}
                        _ => {}
                    }
                    IteratingState::Finished
                }
                _ => IteratingState::Continue,
            };

            match st {
                IteratingState::Continue => {
                    c_idx += 1;
                }
                IteratingState::Finished => {
                    return;
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        let s1: usize = if self.is_leaf { 1 } else { 0 };
        let s2: usize = self.children.iter().map(|c| c.size()).sum();
        s1 + s2
    }
}

#[cfg(test)]
mod tests {
    use super::PatriciaTree;

    #[allow(dead_code)]
    fn print(root: &PatriciaTree) {
        println!("==> PatriciaTree <==");
        print_r(root, 0);
    }

    #[allow(dead_code)]
    fn print_r(tree: &PatriciaTree, indent: i32) {
        for _ in 0..indent {
            print!(" ");
        }

        print!("|- \"{}\"", tree.prefix);
        if tree.is_leaf {
            println!(" [leaf]");
        } else {
            println!("");
        }
        for c in &tree.children {
            print_r(c, indent + 2);
        }
    }

    #[test]
    fn cmp_first_char_test() {
        use std::cmp::Ordering;
        use super::cmp_first_char;

        assert_eq!(cmp_first_char("", ""), Ordering::Equal);
        assert_eq!(cmp_first_char("a", ""), Ordering::Greater);
        assert_eq!(cmp_first_char("", "a"), Ordering::Less);
        assert_eq!(cmp_first_char("a", "a"), Ordering::Equal);
        assert_eq!(cmp_first_char("a", "b"), Ordering::Less);
        assert_eq!(cmp_first_char("b", "a"), Ordering::Greater);
        assert_eq!(cmp_first_char("a", "ab"), Ordering::Equal);
        assert_eq!(cmp_first_char("bc", "b"), Ordering::Equal);
        assert_eq!(cmp_first_char("a", "bc"), Ordering::Less);
        assert_eq!(cmp_first_char("bc", "cd"), Ordering::Less);
        assert_eq!(cmp_first_char("b", "ac"), Ordering::Greater);
        assert_eq!(cmp_first_char("ca", "b"), Ordering::Greater);
    }

    #[test]
    fn add_size_test() {
        let mut root = PatriciaTree::new();
        assert_eq!(root.size(), 0);

        root.add("test");
        assert_eq!(root.size(), 1);

        root.add("tea");
        assert_eq!(root.size(), 2);

        root.add("teapot");
        assert_eq!(root.size(), 3);

        root.add("root");
        assert_eq!(root.size(), 4);

        root.add("rooter");
        assert_eq!(root.size(), 5);

        root.add("roast");
        assert_eq!(root.size(), 6);
    }

    #[test]
    fn add_exist_test() {
        let mut root = PatriciaTree::new();
        assert!(!root.exist("test"));

        root.add("test");
        assert!(root.exist("test"));

        root.add("tea");
        assert!(root.exist("tea"));
        assert!(!root.exist("te"));

        root.add("teapot");
        assert!(root.exist("teapot"));
        assert!(root.exist("tea"));

        root.add("root");
        root.add("rooter");
        root.add("roast");
        assert!(root.exist("root"));
        assert!(root.exist("rooter"));
        assert!(root.exist("roast"));
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

        // root.remove("roast");
        // assert!(!root.exist("roast"));
        // root.remove("root");
        // assert!(!root.exist("root"));
        // root.remove("test");
        // assert!(!root.exist("test"));
        // root.remove("teapot");
        // assert!(!root.exist("teapot"));
        // root.remove("rooter");
        // assert!(!root.exist("rooter"));
        // root.remove("tea");
        // assert!(!root.exist("tea"));
        //
    }
}
