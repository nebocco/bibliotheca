// verification failed at half_rot_killer: https://judge.yosupo.jp/submission/64242


use std::clone::Clone;
use std::rc::Rc;

pub struct PersistentStack<T>(Option<Rc<Node<T>>>);

struct Node<T> {
    value: T,
    next: PersistentStack<T>,
}

impl<T: Clone> PersistentStack<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn push(&self, value: T) -> Self {
        Self(Some(Rc::new(Node {
            value,
            next: self.clone(),
        })))
    }

    pub fn pop(&self) -> Option<(T, Self)> {
        self.0.as_ref().map(|node| (node.value.clone(), node.next.clone()))
    }

    pub fn top(&self) -> Option<&T> {
        self.0.as_ref().map(|x| &x.value)
    }

    pub fn append(&self, rhs: &Self) -> Self {
        if let Some((value, next)) = rhs.pop() {
            Self(Some(Rc::new(Node {
                value,
                next: self.append(&next),
            })))
        } else {
            self.clone()
        }
    }

    pub fn reverse(&self) -> Self {
        let mut ret = Self::new();
        let mut stack = self.clone();
        while let Some((value, next)) = stack.pop() {
            ret = ret.push(value);
            stack = next;
        }
        ret
    }
}

impl<T> Clone for PersistentStack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Persistent queue implemented by Banker's Queue
/// push, pop: amortized O(1)
#[derive(Clone)]
pub struct PersistentQueue<T>{
    front_size: usize,
    rear_size: usize,
    front: PersistentStack<T>,
    rear: PersistentStack<T>
}

impl<T: Clone> PersistentQueue<T> {
    pub fn new() -> Self {
        Self{
            front_size: 0,
            rear_size: 0,
            front: PersistentStack::new(),
            rear: PersistentStack::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front_size == 0
    }

    pub fn top(&self) -> Option<&T> {
        self.front.top()
    }

    pub fn push(&self, value: T) -> Self {
        let res = Self {
            front_size: self.front_size,
            rear_size: self.rear_size + 1,
            front: self.front.clone(),
            rear: self.rear.push(value),
        };
        res.normalize()
    }

    pub fn pop(&self) -> Option<(T, Self)> {
        if let Some((value, front)) = self.front.pop() {
            let res = Self {
                front_size: self.front_size - 1,
                rear_size: self.rear_size,
                front,
                rear: self.rear.clone(),
            };
            Some((value, res.normalize()))
        } else {
            None
        }
    }

    fn normalize(&self) -> Self {
        if self.front_size < self.rear_size {
            Self {
                front_size: self.front_size + self.rear_size,
                rear_size: 0,
                front: self.rear.reverse().append(&self.front),
                rear: PersistentStack::new(),
            }
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_queue_hand() {
        let mut history = Vec::new();
        let s = PersistentQueue::new();
        history.push(s.push(6));
        history.push(history[0].push(7));
        let (value, next) = history[0].pop().unwrap();
        assert_eq!(value, 6);
        history.push(next);
        history.push(s.push(8));
        let (value, next) = history[3].pop().unwrap();
        assert_eq!(value, 8);
        history.push(next);
        let (value, next) = history[1].pop().unwrap();
        assert_eq!(value, 6);
        history.push(next);
    }
}
