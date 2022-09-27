pub fn basic_types() -> Vec<String> {
    let types = vec![
        "int", "ll", "ull", "string", "bool", "char", "float", "double",
    ];

    types.iter().map(|t| t.to_string()).collect()
}

pub fn outer_types() -> Vec<String> {
    let types = vec!["vec", "set"];

    types.iter().map(|t| t.to_string()).collect()
}
