#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<usize>,
    pub prev: Option<usize>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}
