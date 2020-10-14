pub struct SlicePermutations<'a, T, const SLOTS: usize> {
    slice: &'a [T],
    indices: [usize; SLOTS],
    first: bool,
}

impl<'a, T, const SLOTS: usize> SlicePermutations<'a, T, SLOTS> {
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            slice,
            indices: [0; SLOTS],
            first: true,
        }
    }
}

impl<'a, T, const SLOTS: usize> Iterator for SlicePermutations<'a, T, SLOTS> {
    type Item = [&'a T; SLOTS];
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let mut arr = [&self.slice[0]; SLOTS];
        if self.first {
            self.first = false;
            return Some(arr);
        }
        let mut i = SLOTS - 1;
        loop {
            if self.indices[i] < self.slice.len() - 1 {
                self.indices[i] += 1;
                for (j, &indice) in self.indices.iter().enumerate() {
                    arr[j] = &self.slice[indice];
                }
                return Some(arr);
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    return None;
                }
                i -= 1;
            }
        }
    }
}
