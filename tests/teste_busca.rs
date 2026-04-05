use std::collections::HashMap;

#[test]
fn teste_simples() {
    let mut produtos = HashMap::new();
    produtos.insert("tv", "Smart TV");

    assert_eq!(produtos.get("tv"), Some(&"Smart TV"));
}