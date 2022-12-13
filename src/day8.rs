use std::{io::BufRead};

pub struct Grid {
    grid: Box<Vec<u32>>,
    rows: usize,
    cols: usize
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

    /// Check whether the tree at the specified grid location is visible
    pub fn visible(&self, row: usize, col: usize) -> bool {
        // If this tree is at the edge of the grid, it is visible
        if row == 0 || row == self.rows || col == 0 || col == self.cols {
            return true;
        }

        let this_tree_height = self.get(row, col);

        let mut visible_from_left = true;
        for i in 0..col {
            if self.get(row, i) >= this_tree_height {
                //  This tree is not visible from the left
                visible_from_left = false;
            }
        }

        let mut visible_from_right = true;
        for i in col+1..self.cols {
            if self.get(row, i) >= this_tree_height {
                // This tree is not visible from the right
                visible_from_right = false;
            }
        }

        let mut visible_from_top = true;
        for i in 0..row {
            if self.get(i, col) >= this_tree_height {
                // This tree is not visible from the top
                visible_from_top = false;
            }
        }

        let mut visible_from_bottom = true;
        for i in row+1..self.rows {
            if self.get(i, col) >= this_tree_height {
                // This tree is not visible from the bottom
                visible_from_bottom = false;
            }
        }

        return visible_from_left || visible_from_right || visible_from_top || visible_from_bottom;
    }

    /// Calculate the scenic score for the specified grid location
    pub fn scenic_score(&self, row: usize, col: usize) -> u32 {
        let this_tree_height = self.get(row, col);

        let mut dist_left = 0;
        for i in (0..col).rev() {
            dist_left += 1;
            if self.get(row, i) >= this_tree_height {
                break;
            }
        }

        let mut dist_right = 0;
        for i in col+1..self.cols {
            dist_right += 1;
            if self.get(row, i) >= this_tree_height {
                break;
            }
        }

        let mut dist_up = 0;
        for i in (0..row).rev() {
            dist_up += 1;
            if self.get(i, col) >= this_tree_height {
                break;
            }
        }

        let mut dist_down = 0;
        for i in row+1..self.rows {
            dist_down += 1;
            if self.get(i, col) >= this_tree_height {
                break;
            }
        }

        return dist_left * dist_right * dist_up * dist_down;
    }
}

/// Read tree heights from the input into a Grid
pub fn read_tree_heights(reader: &mut dyn BufRead) -> Grid {
    let mut grid: Grid = Grid::new(10, 10);
    let mut row: usize = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();

        if row == 0 {
            grid.resize(trimmed_line.len(), trimmed_line.len());
        }
        
        let mut col: usize = 0;
        for c in trimmed_line.chars() {
            let value = c.to_digit(10).unwrap();
            grid.set(row, col, value);
            col += 1;
        }

        row += 1;
    }

    return grid;
}

/// Count the number of visible trees in the input Grid
pub fn count_visible_trees(grid: &Grid) -> u32 {
    let mut visible_trees = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.visible(row, col) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

/// Find the highest scenic score in the grid
pub fn highest_scenic_score(grid: &Grid) -> u32 {
    let mut highest_score = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let score = grid.scenic_score(row, col);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    return highest_score;
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

    #[test]
    fn visible_from_left() {
        let input = "111
                     011
                     111";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(trees.visible(1, 1));
    }

    #[test]
    fn visible_from_right() {
        let input = "111
                     110
                     111";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(trees.visible(1, 1));
    }

    #[test]
    fn visible_from_top() {
        let input = "101
                     111
                     111";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(trees.visible(1, 1));
    }

    #[test]
    fn visible_from_bottom() {
        let input = "111
                     111
                     101";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(trees.visible(1, 1));
    }

    #[test]
    fn not_visible() {
        let input = "111
                     111
                     111";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(!trees.visible(1, 1));
    }

    #[test]
    fn visible_edge() {
        let input = "111
                     111
                     111";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);
        assert!(trees.visible(0, 0));
        assert!(trees.visible(2, 2));
    }

    #[test]
    fn read_tree_heights_test() {
        let input = "30373
                     25512
                     65332
                     33549
                     35390";
        let mut buf = input.as_bytes();
        let trees = read_tree_heights(&mut buf);

        // tests from problem description
        assert!( trees.visible(1, 1));
        assert!( trees.visible(1, 2));
        assert!(!trees.visible(1, 3));
        assert!( trees.visible(2, 1));
        assert!(!trees.visible(2, 2));
        assert!(!trees.visible(3, 1));
        assert!( trees.visible(3, 2));
        assert!(!trees.visible(3, 3));

        let visible_trees = count_visible_trees(&trees);
        assert_eq!(21, visible_trees);

        assert_eq!(4, trees.scenic_score(1, 2));
        assert_eq!(8, trees.scenic_score(3, 2));

        let highest_score = highest_scenic_score(&trees);
        assert_eq!(8, highest_score);
    }
}