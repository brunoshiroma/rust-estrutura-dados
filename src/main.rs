use crate::lista_encadeada::Lista;

mod lista_encadeada;

fn main() {
    let mut lista_encadeada = lista_encadeada::ListaEncadeada::<i128>::new();

    lista_encadeada.insere(128);
    lista_encadeada.insere(64);

    print!("valor {}", lista_encadeada.obtem(0).unwrap());
}
