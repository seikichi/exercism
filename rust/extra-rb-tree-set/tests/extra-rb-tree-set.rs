use rb_tree_set::*;

#[test]
fn test_new_set_is_empty() {
    let set: TreeSet<i32> = TreeSet::new();
    assert_eq!(set.len(), 0, "set's length must be 0");
}

#[test]
fn test_insert_increments_length() {
    let mut set = TreeSet::new();
    set.insert(42);
    assert_eq!(set.len(), 1, "set's length must be 1");
    set.insert(0);
    assert_eq!(set.len(), 2, "set's length must be 2");
    set.insert(0);
    assert_eq!(set.len(), 2, "set's length must be 2");
}

#[test]
fn test_get_returns_optional_results() {
    let mut set = TreeSet::new();
    set.insert(42);
    assert_eq!(set.get(&42), Some(&42), "set must contains 42");
    assert_eq!(set.get(&0), None, "set should not contain 0");
    set.insert(0);
    assert_eq!(set.get(&0), Some(&0), "set must contains 0");
}
