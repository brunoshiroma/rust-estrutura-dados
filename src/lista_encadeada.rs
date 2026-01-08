pub trait Lista<T> {
    fn insere(&mut self, t: T);

    fn obtem(&mut self, indice: i32) -> Option<T>;
}

struct No<T> {
    valor: T,
    proximo: Option<Box<No<T>>>,
}

pub struct ListaEncadeada<T> {
    topo: Option<Box<No<T>>>,
}

impl<T: Clone> Lista<T> for ListaEncadeada<T> {
    fn insere(&mut self, t: T) {
        match &mut self.topo {
            None => {
                self.topo = Option::Some(Box::from(No {
                    valor: t,
                    proximo: None,
                }));
            }
            Some(no) => {
                no.proximo = Option::Some(Box::from(No {
                    valor: t,
                    proximo: None,
                }));
            }
        }
    }

    fn obtem(&mut self, indice: i32) -> Option<T> {
        if indice < 0 {
            return None;
        }
        let mut corrente = &mut self.topo;
        for _ in 0..indice {
            match corrente {
                Some(no) => corrente = &mut no.proximo,
                None => return None,
            }
        }
        corrente.as_mut().map(|no| no.valor.clone())
    }
}

impl<T> ListaEncadeada<T> {
    pub fn new() -> Self {
        ListaEncadeada { topo: None }
    }
}
