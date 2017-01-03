#[derive(Clone)]
pub struct PatriciaTree {
    prefix: String,
    is_leaf: bool,
    children: Vec<PatriciaTree>,
}

impl PatriciaTree {
    pub fn new() -> PatriciaTree {
        PatriciaTree {
            prefix: "".to_owned(),
            is_leaf: false,
            children: vec![],
        }
    }

    fn new_with(prefix: &str, is_leaf: bool, children: Vec<PatriciaTree>) -> PatriciaTree {
        PatriciaTree {
            prefix: prefix.to_owned(),
            is_leaf: is_leaf,
            children: children,
        }
    }

    pub fn exist(&self, s: &str) -> bool {
        let l = self.same_len(s);
        if l == self.prefix.len() {
            if l == s.len() && self.is_leaf {
                true
            } else {
                let substr = &s[l..];
                self.children.iter().any(|c| c.exist(substr))
            }
        } else {
            false
        }
    }

    pub fn add(&mut self, s: &str) {
        let l = self.same_len(s);
        if l == self.prefix.len() {
            if l == s.len() {
                self.is_leaf = true;
            } else {
                let substr = &s[l..];
                for c in &mut self.children {
                    if c.match_first_char(substr) {
                        c.add(substr);
                        return;
                    }
                }

                self.children.push(PatriciaTree::new_with(substr, true, vec![]));
            }
        } else {
            let new_child =
                PatriciaTree::new_with(&self.prefix[l..], self.is_leaf, self.children.clone());

            self.children.clear();
            self.children.push(new_child);
            self.children.push(PatriciaTree::new_with(&s[l..], true, vec![]));

            self.prefix.truncate(l);
            self.is_leaf = false;
        }
    }

    pub fn remove(&mut self, s: &str) {
        let l = self.same_len(s);
        if l == self.prefix.len() {
            let substr = &s[l..];
            let mut idx: usize = 0;
            loop {
                if idx >= self.children.len() {
                    break;
                }

                if self.children[idx].prefix == substr {
                    match self.children[idx].children.len() {
                        0 => {
                            self.children.remove(idx);
                            if self.prefix != "" && self.children.len() == 1 {
                                self.prefix += &self.children[0].prefix;
                                self.is_leaf = self.children[0].is_leaf;
                                self.children = self.children[0].children.clone();

                            }
                        }
                        1 => {
                            let new_prefix = format!("{}{}",
                                                     self.children[idx].prefix,
                                                     self.children[idx].children[0].prefix);
                            self.children[idx].children[0].prefix = new_prefix;

                            self.children[idx] = self.children[idx].children[0].clone();
                        }
                        _ => {
                            self.children[idx].is_leaf = false;
                        }
                    }
                    return;
                }
                idx += 1;
            }

            for c in &mut self.children {
                if c.match_first_char(substr) {
                    c.remove(substr);
                }
            }
        }
    }

    fn same_len(&self, s: &str) -> usize {
        let mut idx: usize = 0;
        let min_len = std::cmp::min(self.prefix.len(), s.len());

        while idx < min_len && self.prefix.chars().nth(idx) == s.chars().nth(idx) {
            idx += 1;
        }

        idx
    }

    fn match_first_char(&self, s: &str) -> bool {
        self.prefix.chars().nth(0) == s.chars().nth(0)
    }

    #[allow(dead_code)]
    fn children_prefixes(&self) -> Vec<String> {
        self.children
            .iter()
            .map(|c| self.prefix.clone() + &c.prefix)
            .collect()
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
    fn mismatch_pos_test() {
        let abc = PatriciaTree::new_with("abc", true, vec![]);
        assert_eq!(abc.same_len("ade"), 1);
        assert_eq!(abc.same_len("abe"), 2);
        assert_eq!(abc.same_len("abc"), 3);
        assert_eq!(abc.same_len("abcd"), 3);
        assert_eq!(abc.same_len("ab"), 2);
        assert_eq!(abc.same_len("bcd"), 0);

        let empty = PatriciaTree::new_with("", true, vec![]);
        assert_eq!(empty.same_len("abc"), 0);
        assert_eq!(empty.same_len(""), 0);
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

        root.remove("roast");
        assert!(!root.exist("roast"));
        root.remove("root");
        assert!(!root.exist("root"));
        root.remove("test");
        assert!(!root.exist("test"));
        root.remove("teapot");
        assert!(!root.exist("teapot"));
        root.remove("rooter");
        assert!(!root.exist("rooter"));
        root.remove("tea");
        assert!(!root.exist("tea"));
    }
}
