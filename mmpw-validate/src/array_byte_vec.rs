use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayByteVec<const N: usize> {
    array: [u8; N],
    len: usize,
}

impl<const N: usize> ArrayByteVec<N> {
    pub fn zeroed_with_len(len: usize) -> Self {
        Self { array: [0; N], len }
    }
    pub fn from_raw(raw: [u8; N], len: usize) -> Self {
        Self { array: raw, len }
    }
    pub fn insert(&mut self, index: usize, value: u8) {
        assert!(self.len < N);
        // Shift all items by 1
        for i in (index..N).rev() {
            let v = if i > 0 { self.array[i - 1] } else { 0 };
            self.array[i] = v;
        }
        self.array[index] = value;
        self.len += 1;
    }
}

impl<const N: usize> Index<u8> for ArrayByteVec<N> {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        &self.array[index as usize]
    }
}

impl<const N: usize> IndexMut<u8> for ArrayByteVec<N> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.array[index as usize]
    }
}

impl<const N: usize> Deref for ArrayByteVec<N> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.array[..self.len]
    }
}

impl<const N: usize> DerefMut for ArrayByteVec<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array[..self.len]
    }
}
