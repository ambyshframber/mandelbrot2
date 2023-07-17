use rayon::prelude::*;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize
}
impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, init: T) -> Grid<T> {
        let mut v = Vec::new();
        v.resize(width * height, init);
        Grid { data: v, width }
    }
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[x + (y * self.width)]
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks_exact(self.width)
    }
    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        self.data
            .chunks_exact(self.width)
            .enumerate()
            .map(|(y, row)| 
                row.iter().enumerate().zip(std::iter::repeat(y))
            )
            .flatten()
            .map(|((x, v), y)| (x, y, v))
    }
    pub fn iter_coords_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> + '_ {
        self.data
            .chunks_exact_mut(self.width)
            .enumerate()
            .map(|(y, row)| 
                row.iter_mut().enumerate().zip(std::iter::repeat(y))
            )
            .flatten()
            .map(|((x, v), y)| (x, y, v))
    }
    /// iterate left-right then top-bottom
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.data.iter().copied()
    }
}
impl<T: Send> Grid<T> {
    pub fn par_iter_rows_mut(&mut self) -> impl ParallelIterator<Item = (usize, &mut [T])> + '_ {
        self.data.par_chunks_exact_mut(self.width).enumerate()
    }
}
