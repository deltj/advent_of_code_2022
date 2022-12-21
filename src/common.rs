pub struct Grid {
    grid: Box<Vec<u32>>,
    pub rows: usize,
    pub cols: usize
}

impl Grid {
    /// Create a new grid with the specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Box::new(Vec::new());

        let grid_size = rows * cols;
        for _i in 0..grid_size {
            grid.push(0u32);
        }

        Self { grid, rows, cols }
    }

    /// Resize the grid
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.grid.resize(rows * cols, 0u32);
        self.rows = rows;
        self.cols = cols;
    }

    /// Return the value at the specified grid location
    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.grid[self.rows * row + col]
    }

    /// Assign a new value to the specified grid location
    pub fn set(&mut self, row: usize, col: usize, value: u32) {
        self.grid[self.rows * row + col] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_new() {
        let g: Grid = Grid::new(1, 1);
        assert_eq!(0, g.get(0, 0));
    }

    #[test]
    fn grid_set_get() {
        let mut g: Grid = Grid::new(2, 2);
        g.set(0, 0, 1);
        g.set(1, 1, 2);
        assert_eq!(1, g.get(0, 0));
        assert_eq!(0, g.get(0, 1));
        assert_eq!(0, g.get(1, 0));
        assert_eq!(2, g.get(1, 1));
    }
}