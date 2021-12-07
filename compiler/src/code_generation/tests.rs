use super::*;

#[test]
fn test_data_segment_with_one_element() {
    let mut builder = DataSegmentBuilder::new();

    builder.add_data("hello".to_string(), "world".to_string());

    let result = builder.build_source();
    let expected = ".section data\n  .hello \"world\"\n";

    assert_eq!(result, expected);
}

#[test]
fn test_data_segment_with_no_elements() {
    let mut builder = DataSegmentBuilder::new();

    let result = builder.build_source();
    let expected = ""; // no data segment

    assert_eq!(result, expected);
}

#[test]
fn test_data_segment_with_multiple_elements() {
    let mut builder = DataSegmentBuilder::new();

    builder.add_data("hello".to_string(), "world".to_string());
    builder.add_data("foo".to_string(), "bar".to_string());

    let result = builder.build_source();
    let expected = ".section data\n  .hello \"world\"\n  .foo \"bar\"\n";

    assert_eq!(result, expected);
}
