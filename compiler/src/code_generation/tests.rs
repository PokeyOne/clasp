use super::*;

#[test]
fn test_data_segment_with_one_element() {
    let mut builder = DataSegmentBuilder::new();

    builder.add_data("hello".to_string(), "world".to_string());

    let result = builder.build_source();
    let expected = ".section data\n  .hello \"world\"\n";

    assert_eq!(result, expected);
}