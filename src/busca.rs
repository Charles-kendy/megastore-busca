use std::collections::HashMap;
use crate::produto::Produto;

pub struct SistemaBusca {
    pub produtos: HashMap<String, Produto>,
}

impl SistemaBusca {
    pub fn novo() -> Self {
        SistemaBusca {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, chave: String, produto: Produto) {
        self.produtos.insert(chave, produto);
    }

    pub fn buscar(&self, chave: &str) -> Option<&Produto> {
        self.produtos.get(chave)
    }
}