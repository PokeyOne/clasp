use crate::label::*;
use std::cmp::Ordering;

#[test]
fn create_and_use_label() {
    let l: Label = Label::new("test_name".to_string(), 24);

    assert_eq!(l.name, "test_name");
    assert_eq!(l.location, 24);
}

#[test]
fn label_comparisons() {
    let a: Label = Label::new("test_name".to_string(), 24);
    let b: Label = Label::new("test_name".to_string(), 48);
    let c: Label = Label::new("other_test_name".to_string(), 48);

    assert_eq!(true, a.eq(&b));
    assert_eq!(true, b.eq(&a));
    assert_eq!(false, a.eq(&c));
    assert_eq!(false, b.eq(&c));

    assert_eq!(
        true,
        match a.cmp(&b) {
            Ordering::Equal => true,
            _ => false
        }
    );

    assert_eq!(
        true,
        match a.cmp(&c) {
            Ordering::Greater => true,
            _ => false
        }
    );
}

#[test]
fn create_empty_collection() {
    let c: LabelCollection = LabelCollection::new();

    assert_eq!(true, c.is_empty());
    assert_eq!(0, c.size());
}

#[test]
fn insert_one_element_to_collection() {
    let mut c = LabelCollection::new();

    c.insert("blah".to_string(), 24);

    assert_eq!(false, c.is_empty());
    assert_eq!(1, c.size());

    c.insert("blah".to_string(), 48);

    assert_eq!(false, c.is_empty());
    assert_eq!(1, c.size());
}

#[test]
fn insert_several_elements_to_label_collection() {
    let mut c = LabelCollection::new();

    c.insert("blah".to_string(), 24);
    c.insert("apples".to_string(), 48);

    assert_eq!(false, c.is_empty());
    assert_eq!(2, c.size());

    c.insert("bananas".to_string(), 32);
    c.insert("baa".to_string(), 64);

    assert_eq!(4, c.size());
}

#[test]
fn retrieve_from_empty_label_collection() {
    let c = LabelCollection::new();

    assert_eq!(true, c.retrieve("some".to_string()).is_none());
}

#[test]
fn retrieve_from_single_celled_label_collection() {
    let mut c = LabelCollection::new();

    c.insert("_main".to_string(), 540);

    assert_eq!(Some(540u64), c.retrieve("_main".to_string()));
    assert_eq!(true, c.retrieve("_other".to_string()).is_none());
}

#[test]
fn full_test_of_label_collection() {
    let mut c = LabelCollection::new();

    assert_eq!(true, c.is_empty());
    assert_eq!(0, c.size());

    c.insert("banana".to_string(), 64);
    c.insert("apple".to_string(), 8);
    c.insert("pear".to_string(), 48);
    assert_eq!(3, c.size());
    c.insert("starfruit".to_string(), 40);
    c.insert("carrot".to_string(), 80);

    assert_eq!(5, c.size());

    assert_eq!(true, c.retrieve("non-existent".to_string()).is_none());
    assert_eq!(Some(64), c.retrieve("banana".to_string()));
    assert_eq!(Some(8), c.retrieve("apple".to_string()));
    assert_eq!(Some(48), c.retrieve("pear".to_string()));
    assert_eq!(Some(40), c.retrieve("starfruit".to_string()));
    assert_eq!(Some(80), c.retrieve("carrot".to_string()));

    c.insert("starfruit".to_string(), 4000);

    assert_eq!(5, c.size());
    assert_eq!(Some(4000), c.retrieve("starfruit".to_string()));
}
