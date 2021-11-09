use crate::data_constants as dc;

#[test]
fn get_register_address() {
    assert_eq!(dc::GA_LOC, dc::get_register_address("ga").unwrap());
    assert_eq!(dc::GF_LOC, dc::get_register_address("gf").unwrap());
    assert_eq!(dc::GX_LOC, dc::get_register_address("gx").unwrap());
    assert_eq!(None, dc::get_register_address("blah"));
}
