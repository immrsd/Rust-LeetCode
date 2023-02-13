use std::collections::VecDeque;

/*
An image is represented by an m x n integer grid image where 
image[i][j] represents the pixel value of the image.

You are also given three integers sr, sc, and color. You should perform 
a flood fill on the image starting from the pixel image[sr][sc].

To perform a flood fill, consider the starting pixel, plus any pixels connected 
4-directionally to the starting pixel of the same color as the starting pixel, 
plus any pixels connected 4-directionally to those pixels (also with the same color), 
and so on. Replace the color of all of the aforementioned pixels with color.

Return the modified image after performing the flood fill.
*/

struct Cell {
    row: usize, 
    col: usize,
}

trait Table {
    fn value_at(self: &Self, row: usize, col: usize) -> i32;
    fn set_value(self: &mut Self, val: i32, row: usize, col: usize);
    fn get_dimensions(self: &Self) -> (usize, usize);
}

impl Table for Vec<Vec<i32>> {

    fn value_at(self: &Self, row: usize, col: usize) -> i32 {
        self[row][col]
    }

    fn set_value(self: &mut Self, val: i32, row: usize, col: usize) {
        self[row][col] = val;
    }

    fn get_dimensions(self: &Self) -> (usize, usize) {
        let height = self.len();
        if height == 0 {
            (0, 0)
        } else {
            let width = self[0].len();
            (height, width)
        }
    }
}

impl Solution {

    pub fn flood_fill(
        image: Vec<Vec<i32>>, 
        sr: i32, 
        sc: i32, 
        color: i32
    ) -> Vec<Vec<i32>> {
        let init_cell = Cell {
            row: sr as usize, 
            col: sc as usize
        };
        let init_color = image.value_at(init_cell.row, init_cell.col);
        if init_color == color {
            return image;
        }
        let (height, width) = image.get_dimensions();
        let mut image = image;
        let mut queue = VecDeque::<Cell>::new();
        queue.push_back(init_cell);

        while let Some(Cell { row, col }) = queue.pop_front() {
            if image.value_at(row, col) != init_color { continue; }
            image.set_value(color, row, col);
            Solution::enqueue_adjacent_cells(&mut queue, row, col, height, width);
        }

        image
    }

    fn enqueue_adjacent_cells(
        queue: &mut VecDeque<Cell>,
        row: usize,
        col: usize, 
        height: usize, 
        width: usize
    ) {
        if row > 0 {
            queue.push_back(Cell { row: row - 1, col });
        }
        if row + 1 < height {
            queue.push_back(Cell { row: row + 1, col });
        }
        if col > 0 {
            queue.push_back(Cell { row, col: col - 1 });
        }
        if col + 1 < width {
            queue.push_back(Cell { row, col: col + 1 });
        }
    }
}
