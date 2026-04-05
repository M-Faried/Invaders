use crate::{NUM_COLS, NUM_ROWS};

// pub type Frame = Vec<Vec<&'static str>>;

// pub fn new_frame() -> Frame {
//     let mut cols = Vec::with_capacity(NUM_COLS);
//     for _ in 0..NUM_COLS {
//         let mut col = Vec::with_capacity(NUM_ROWS);
//         for _ in 0..NUM_ROWS {
//             col.push(" ");
//         }
//         cols.push(col);
//     }
//     cols
// }

pub struct Frame {
    data: Vec<Vec<&'static str>>,
}

impl Frame {
    pub fn new() -> Self {
        let mut cols = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_COLS {
            let mut col = Vec::with_capacity(NUM_ROWS);
            for _ in 0..NUM_ROWS {
                col.push(" ");
            }
            cols.push(col);
        }
        Self { data: cols }
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: &'static str) {
        self.data[x][y] = val;
    }

    pub fn get_at(&self, x: usize, y: usize) -> &str {
        self.data[x][y]
    }

    pub fn iter(&self) -> FrameIterator {
        FrameIterator {
            frame: &self,
            col: 0,
            row: 0,
        }
    }
}

pub struct FrameIterator<'a> {
    frame: &'a Frame,
    col: usize,
    row: usize,
}

impl<'a> Iterator for FrameIterator<'a> {
    type Item = (usize, usize, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        // If col is out of bounds, we're done
        if self.col >= NUM_COLS {
            return None;
        }

        let item = (self.col, self.row, self.frame.data[self.col][self.row]);

        // Advance position: row-first within each column
        self.row += 1;
        if self.row >= NUM_ROWS {
            self.row = 0;
            self.col += 1; // move to next column
        }

        Some(item)
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
