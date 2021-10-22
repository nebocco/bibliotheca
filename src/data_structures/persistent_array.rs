use std::clone::Clone;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct PersistentArray<T>(Option<Rc<Node<T>>>);

struct Node<T> {
    value: Option<T>,
    children: Box<[PersistentArray<T>]>,
}

impl<T: Clone> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: None,
            children: vec![PersistentArray(None); 20].into_boxed_slice(),
        }
    }
}

impl<T: Clone> PersistentArray<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn set(&self, idx: usize, value: T) -> Self {
        let mut res = if let Some(node) = self.0.as_ref() {
            node.as_ref().clone()
        } else {
            Node::default()
        };
        if idx == 0 {
            res.value = Some(value);
        } else {
            res.children[idx % 20] = res.children[idx % 20].set(idx / 20, value);
        }
        Self(Some(Rc::new(res)))
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if let Some(node) = self.0.as_ref() {
            if idx == 0 {
                node.value.as_ref()
            } else {
                node.children[idx % 20].get(idx / 20)
            }
        } else {
            None
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone(),
        }
    }
}
