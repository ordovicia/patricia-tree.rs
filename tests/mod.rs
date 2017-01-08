extern crate patricia_tree;
use patricia_tree::PatriciaTree;

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

    {
        let mut test = |s| {
            assert!(root.exist(s));
            root.remove(s);
            assert!(!root.exist(s));
        };

        test("teapot");
        test("roast");
        test("root");
        test("test");
        test("tea");
        test("rooter");
    }
}
