use std::{mem};

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct ChunkList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> ChunkList<T> {
    pub fn new() -> Self {
        ChunkList {
            head: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_head);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            self.len -= 1;
            old_head.elem
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Erases all elements in the list.
    pub fn clear(&mut self)
    where T: PartialEq {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


impl<T> Drop for ChunkList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}



/// --------------------
/// INTO ITER
/// --------------------

pub struct IntoIter<T>(ChunkList<T>);

impl<T> ChunkList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop_front()
    }
}

/// --------------------
/// ITER
/// --------------------

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> ChunkList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

/// --------------------
/// ITER MUT
/// --------------------

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> ChunkList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}




#[cfg(test)]
mod test {
    use super::ChunkList;
    
    #[test]
    fn push_pop_front() {
        let mut list = ChunkList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = ChunkList::new();
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_front_mut(), None);
        // assert_eq!(list.peek_back(), None);
        // assert_eq!(list.peek_back_mut(), None);
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_front_mut(), Some(&mut 1));
        // assert_eq!(list.peek_back(), Some(&3));
        // assert_eq!(list.peek_back_mut(), Some(&mut 3));
    }

    #[test]
    fn into_iter() {
        let mut list = ChunkList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = ChunkList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = ChunkList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn erase() {
        let mut list = ChunkList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.clear();
        assert_eq!(list.peek_front(), None);
    }
}


fn main() {

}