pub struct Solution;

impl Solution {
    /// Counts the number of islands in a given 2D grid.
    ///
    /// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
    /// The function modifies the input grid in-place to mark visited land cells.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D vector of characters where '1' represents land and '0' represents water.
    ///
    /// # Returns
    ///
    /// * `i32` - The number of islands found in the grid.
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut grid = grid; // make grid mutable

        let height = grid.len();
        let width = grid.first().unwrap().len();

        let mut islands = 0;

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == '1' {
                    islands += 1;
                    Self::dfs(&mut grid, x, y, width, height);
                }
            }
        }

        islands
    }

    /// Performs a depth-first search (DFS) on the grid to mark all connected land ('1') cells as visited.
    ///
    /// # Arguments
    ///
    /// * `grid` - A mutable reference to a 2D vector representing the grid of land ('1') and water ('0').
    /// * `x` - The current x-coordinate (column index) in the grid.
    /// * `y` - The current y-coordinate (row index) in the grid.
    /// * `width` - The total number of columns in the grid.
    /// * `height` - The total number of rows in the grid.
    ///
    /// This function modifies the grid in-place, setting visited land cells to '0' to avoid revisiting.
    pub fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize, width: usize, height: usize) {
        if x >= width || y >= height || grid[y][x] == '0' {
            return;
        }

        grid[y][x] = '0';

        if x > 0 {
            Self::dfs(grid, x - 1, y, width, height);
        }
        Self::dfs(grid, x + 1, y, width, height);
        if y > 0 {
            Self::dfs(grid, x, y - 1, width, height);
        }
        Self::dfs(grid, x, y + 1, width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_single_island() {
        let grid = vec![vec!['1']];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_no_islands() {
        let grid = vec![vec!['0', '0', '0'], vec!['0', '0', '0']];
        assert_eq!(Solution::num_islands(grid), 0);
    }
}
