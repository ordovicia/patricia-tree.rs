#[derive(Debug)]
pub struct PatriciaTree<Item> {
    prefix: String,
    value: Option<Item>,
    children: Vec<Box<PatriciaTree<Item>>>,
}

fn slice_to_string(s: &[char]) -> String {
    s.to_vec().into_iter().collect()
}

fn split_charvec_str(s: &Vec<char>, at: usize) -> (String, String) {
    let (p, s) = s.split_at(at);
    (slice_to_string(p), slice_to_string(s))
}

impl<Item> PatriciaTree<Item> {
    pub fn new() -> PatriciaTree<Item> {
        PatriciaTree {
            prefix: String::new(),
            value: None,
            children: vec![],
        }
    }

    fn box_with(prefix: &str,
                value: Option<Item>,
                children: Vec<Box<PatriciaTree<Item>>>)
                -> Box<PatriciaTree<Item>> {
        assert!(prefix != "");
        Box::new(PatriciaTree::<Item> {
                     prefix: prefix.to_owned(),
                     value: value,
                     children: children,
                 })
    }

    fn add_child(&mut self, child: Box<PatriciaTree<Item>>) {
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
            match (prefix.next(), s.next()) {
                (Some(p), Some(c)) if p == c => { /* continue */ }
                (Some(_), Some(_)) => {
                    return false;
                }
                (Some(_), None) => {
                    return false;
                }
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    return match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                               Ok(child_idx) => self.children[child_idx].exist(&s_suffix),
                               Err(_) => false,
                           };
                }
                (None, None) => {
                    return self.value.is_some();
                }
            };
        }
    }

    pub fn add(&mut self, s: &str, value: Item) {
        let mut c_idx: usize = 0;
        let prefix: Vec<char> = self.prefix.chars().collect();
        let mut s = s.chars();

        loop {
            match (prefix.get(c_idx), s.next()) {
                (Some(&p), Some(c)) if p == c => {
                    c_idx += 1;
                    assert!(c_idx <= self.prefix.len());
                },
                (Some(_), Some(c)) /* p != c */ => {
                    /*
                     * tea + test -> te
                     *  |             |--|
                     *  |             a  st
                     *  |             |
                     * pot            pot
                     */

                    let (p_prefix, p_suffix) = split_charvec_str(&prefix, c_idx); // te, a
                    let s_suffix = format!("{}{}", c, s.as_str()); // st

                    let mut child_value = None;
                    std::mem::swap(&mut child_value, &mut self.value);
                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suffix, child_value, child_children); // a
                    self.add_child(child);

                    self.add_child(PatriciaTree::box_with(&s_suffix, Some(value), vec![])); // st

                    self.prefix = p_prefix;

                    break;
                }
                (Some(_), None) => {
                    /*
                     * teapot + tea -> tea
                     *                  |
                     *                 pot
                     */

                    let (p_prefix, p_suffix) = split_charvec_str(&prefix, c_idx); // tea, pot

                    let mut child_value = None;
                    std::mem::swap(&mut child_value, &mut self.value);
                    let mut child_children = vec![];
                    std::mem::swap(&mut child_children, &mut self.children);
                    let child = PatriciaTree::box_with(&p_suffix, child_value, child_children); // pot
                    self.add_child(child);

                    self.prefix = p_prefix;
                    self.value = Some(value);

                    break;
                }
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                        Ok(child_idx) => {
                            self.children[child_idx].add(&s_suffix, value);
                        }
                        Err(_) => {
                            self.add_child(PatriciaTree::box_with(&s_suffix, Some(value), vec![]));
                        }
                    }

                    break;
                }
                (None, None) => {
                    self.value = Some(value);
                    break;
                }
            }
        }
    }

    pub fn find(&self, s: &str) -> Option<&Item> {
        let mut prefix = self.prefix.chars();
        let mut s = s.chars();

        loop {
            match (prefix.next(), s.next()) {
                (Some(p), Some(c)) if p == c => { /* continue */ }
                (Some(_), Some(_)) => {
                    return None;
                }
                (Some(_), None) => {
                    return None;
                }
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    return match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                               Ok(child_idx) => self.children[child_idx].find(&s_suffix),
                               Err(_) => None,
                           };
                }
                (None, None) => {
                    return self.value.as_ref();
                }
            };
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
            match (prefix.get(c_idx), s.next()) {
                (Some(&p), Some(c)) if p == c => {
                    c_idx += 1;
                    assert!(c_idx <= self.prefix.len());
                }
                (Some(_), Some(_)) /* p != c */ | (Some(_), None) => { break; }
                (None, Some(c)) => {
                    let s_suffix = format!("{}{}", c, s.as_str());
                    match self.children.binary_search_by(|c| c.cmp_first_char(&s_suffix)) {
                        Ok(child_idx) => {
                            self.children[child_idx].remove(&s_suffix);
                        }
                        Err(_) => {}
                    }

                    break;
                }
                (None, None) => {
                    match self.children.len() {
                        0 => {
                            // assert!(self.value.is_some()); FIXME
                            self.value = None;
                        }
                        1 => {
                            assert!(self.value.is_some());
                            // assert!(self.children[0].value.is_some()); FIXME
                            self.prefix.push_str(&self.children[0].prefix);
                            std::mem::swap(&mut self.value, &mut self.children[0].value); // FIXME
                            // self.value = self.children[0].value; // FIXME
                            self.children.clear();
                        }
                        _ => {
                            self.value = None;
                        }
                    }

                    break;
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        let leaf_cnt: usize = if self.value.is_some() { 1 } else { 0 };
        let children_cnt: usize = self.children
            .iter()
            .map(|c| c.size())
            .sum();
        leaf_cnt + children_cnt
    }
}

use std::fmt::Display;

impl<Item> Display for PatriciaTree<Item>
    where Item: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const INDENT_WIDTH: usize = 2;

        fn fmt_r<Item>(f: &mut std::fmt::Formatter, tree: &PatriciaTree<Item>, indent: usize)
            where Item: Display
        {
            for _ in 0..indent {
                write!(f, " ").unwrap();
            }
            write!(f, "- {} ", tree.prefix).unwrap();
            if tree.value.is_some() {
                writeln!(f, "{}", tree.value.as_ref().unwrap()).unwrap();
            } else {
                writeln!(f, "(none)").unwrap();
            }

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
                root.add($s, $s);
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
                root.add($s, $s);
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
    fn find_test() {
        let mut root = PatriciaTree::new();
        root.add("test", 0);
        root.add("tea", 1);
        root.add("teapot", 2);
        root.add("root", 3);
        root.add("rooter", 4);
        root.add("roast", 5);

        println!("{:#?}", root);

        assert!(root.find("test") == Some(&0));
        assert!(root.find("tea") == Some(&1));
        assert!(root.find("teapot") == Some(&2));
        assert!(root.find("root") == Some(&3));
        assert!(root.find("rooter") == Some(&4));
        assert!(root.find("roast") == Some(&5));

        assert!(root.find("po") == None);
    }

    #[test]
    fn remove_test() {
        let mut root = PatriciaTree::new();
        root.add("test", 0);
        root.add("tea", 1);
        root.add("teapot", 2);
        root.add("root", 3);
        root.add("rooter", 4);
        root.add("roast", 5);

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
