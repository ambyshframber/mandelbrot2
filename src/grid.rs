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
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }
}
