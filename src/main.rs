mod produto;
mod busca;

use produto::Produto;
use busca::SistemaBusca;

fn main() {
    let mut sistema = SistemaBusca::novo();

    sistema.adicionar(
        "notebook".to_string(),
        Produto {
            nome: "Notebook Dell".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    sistema.adicionar(
        "tv".to_string(),
        Produto {
            nome: "Smart TV".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
    );

    let resultado = sistema.buscar("tv");

    match resultado {
        Some(produto) => println!("Encontrado: {}", produto.nome),
        None => println!("Produto não encontrado"),
    }
}