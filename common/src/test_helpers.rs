#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    }
}

#[test]
fn string_vec_macro_rule() {
    let result: Vec<String> = string_vec!["this", "is", "a", "test"];

    assert_eq!(4, result.len());
    assert_eq!("this", result[0]);
}
