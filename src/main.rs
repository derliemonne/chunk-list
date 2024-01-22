use std::{mem, collections::{LinkedList, VecDeque}};

pub struct Chunk<T, const N: usize> {
    elements: Vec<T>,
}

impl<T, const N: usize> Default for Chunk<T, N> {
    fn default() -> Self {
        Chunk::new()
    }
}

impl<T, const N: usize> Chunk<T, N> {
    pub fn new() -> Self {
        Self {
            elements: Vec::<T>::with_capacity(N),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == N
    }

    /// Return false in case of chunk overflow.
    pub fn push_back(&mut self, value: T) -> bool {
        if self.is_full(){
            return false
        }
        self.elements.push(value);
        true
    }

    pub fn push_front(&mut self, value: T) -> bool {
        if self.is_full() {
            return false
        }
        self.elements.push(value);
        self.elements.rotate_right(1);
        true
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(0)
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= N {
            panic!()
        }
        self.elements.get(i)
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if i >= N {
            panic!()
        }
        if i >= self.elements.len() {
            return None
        }
        Some(self.elements.remove(i))
    }



}

pub struct ChunkList<T, const N: usize> {
    chunks: VecDeque<Chunk<T, N>>,
    elements_count: usize,
}

impl<T, const N: usize> ChunkList<T, N> {
    pub fn new() -> Self {
        if N < 1 {
            panic!()
        }
        ChunkList {
            chunks: VecDeque::new(),
            elements_count: 0,
        }
    }

    /// Constructs the container with count copies of elements with value.
    pub fn new_filled(count: usize, value: &T) -> Self
    where T: Clone {
        let mut chunk_list = Self::new();
        for i in 0..count {
            chunk_list.push_back(value.clone())
        }
        chunk_list
    }

    pub fn add_new_chunk_front(&mut self) -> &mut Chunk<T, N> {
        self.chunks.push_front(Chunk::new());
        self.chunks.front_mut().unwrap()
    }

    pub fn add_new_chunk_back(&mut self) -> &mut Chunk<T, N> {
        self.chunks.push_back(Chunk::new());
        self.chunks.back_mut().unwrap()
    }

    pub fn remove_chunk(&mut self, i: usize) -> Option<Chunk<T, N>> {
        self.chunks.remove(i)
    }

    pub fn push_back(&mut self, value: T) {
        let chunk = match self.chunks.back_mut() {
            Some(back) => back,
            None => self.add_new_chunk_back(),
        };
        if chunk.is_full() {
            self.add_new_chunk_back().push_back(value);
        }
        else {
            chunk.push_back(value);
        }
        self.elements_count += 1
    }

    pub fn push_front(&mut self, value: T) {
        let not_full_chunk = match self.chunks.front_mut() {
            None => self.add_new_chunk_front(),
            Some(chunk) => {
                if chunk.is_full() {
                    self.add_new_chunk_front()
                } else {
                    chunk
                }
            },
        };
        not_full_chunk.push_front(value);
        self.elements_count += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let chunk = self.chunks.back_mut()?;
        let value = chunk.pop_back().unwrap();
        if chunk.is_empty() {
            self.chunks.pop_back();
        }
        self.elements_count -= 1;
        Some(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(0)
    }

    pub fn chunks_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn elements_count(&self) -> usize {
        self.elements_count
    }

    /// Deletes all elements in the list.
    pub fn clear(&mut self)
    where T: PartialEq {
        self.chunks.clear();
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.iter().nth(i)
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let mut chunk_i = 0;
        let mut count = 0;
        while let Some(chunk) = self.chunks.get_mut(chunk_i) {
            if count + chunk.len() >= i {
                break;
            }
            count += chunk.len();
            chunk_i += 1;
        }
        let chunk = self.chunks.get_mut(chunk_i)?;
        let element_i = count - i;
        let value = chunk.remove(element_i);
        if chunk.is_empty() {
            self.remove_chunk(i);
        }
        self.elements_count -= 1;
        value
    }
}

// --------------------
// INTO ITER
// --------------------
pub struct IntoIter<T, const N: usize>(ChunkList<T, N>);

impl<T, const N: usize> ChunkList<T, N> {
    pub fn into_iter(self) -> IntoIter<T, N> {
        IntoIter(self)
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let front_chunk = self.0.chunks.front_mut()?;
        let value = front_chunk.pop_front();
        if front_chunk.is_empty() {
            self.0.chunks.pop_front();
        }
        value
    }
}

// impl<T, const N: usize> Iterator for ChunkList<T, N> {
//     type Item = T;
// }

// --------------------
// ITER
// --------------------
pub struct Iter<'a, T, const N: usize> {
    chunk_list: &'a ChunkList<T, N>,
    chunk_i: usize,
    element_i: usize,
}

impl<T, const N: usize> ChunkList<T, N> {
    pub fn iter(&self) -> Iter<T, N> {
        Iter { chunk_list: self, chunk_i: 0, element_i: 0} 
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunk_list.chunks.get(self.chunk_i)?;
        let value = match chunk.get(self.element_i) {
            None => {
                self.chunk_i += 1;
                self.element_i = 0;
                self.next()
            }
            Some(value) => Some(value),
        };
        self.element_i += 1;
        if self.element_i >= chunk.len() {
            self.element_i = 0;
            self.chunk_i += 1;
        }
        value
    }
}


#[cfg(test)]
mod test {
    use super::ChunkList;
    
    #[test]
    fn push_pop_front() {
        let mut list = ChunkList::<i32, 2>::new();
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
        let mut list = ChunkList::<i32, 2>::new();
        assert_eq!(list.get(0), None);
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.get(0), Some(&1));
    }

    #[test]
    fn into_iter() {
        let mut list = ChunkList::<i32, 2>::new();
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
        let mut list = ChunkList::<i32, 2>::new();
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
    fn erase() {
        let mut list = ChunkList::<i32, 2>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.clear();
        assert_eq!(list.get(0), None);
    }

    #[test]
    fn elements_count() {
        let mut list = ChunkList::<i32, 2>::new();
        assert_eq!(list.elements_count(), 0);
        list.push_front(1);
        assert_eq!(list.elements_count(), 1);
        list.push_front(2);
        assert_eq!(list.elements_count(), 2);
        list.pop_front();
        assert_eq!(list.elements_count(), 1);
        list.pop_back();
        assert_eq!(list.elements_count(), 0);
    }
}


fn main() {

}