use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct Maze {
    width: usize,
    height: usize,
    walls_v: Vec<Vec<bool>>,
    walls_h: Vec<Vec<bool>>,
}

impl Maze {
    fn get_neighbours(
        &self,
        visited: &Vec<Vec<bool>>,
        row: usize,
        col: usize,
    ) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);
        if col + 1 < self.width && !visited[row][col + 1] {
            neighbors.push((row, col + 1));
        }
        if col > 0 && !visited[row][col - 1] {
            neighbors.push((row, col - 1));
        }
        if row + 1 < self.height && !visited[row + 1][col] {
            neighbors.push((row + 1, col));
        }
        if row > 0 && !visited[row - 1][col] {
            neighbors.push((row - 1, col));
        }
        neighbors
    }

    fn remove_wall(&mut self, start_row: usize, start_col: usize, end_row: usize, end_col: usize) {
        if start_row == end_row {
            if start_col > end_col {
                self.walls_v[start_row][start_col] = false;
            } else {
                self.walls_v[start_row][end_col] = false;
            }
        } else {
            if start_row > end_row {
                self.walls_h[start_row][start_col] = false;
            } else {
                self.walls_h[end_row][start_col] = false;
            }
        }
    }

    fn set_entry_and_exit(&mut self, rng: &mut ChaCha8Rng) {
        if *[true, false].choose(rng).unwrap() {
            self.walls_h[0][rng.gen_range(0..self.width)] = false;
        } else {
            self.walls_v[rng.gen_range(0..self.height)][self.width - 1] = false;
        }
        if *[true, false].choose(rng).unwrap() {
            self.walls_h[self.height + 1][rng.gen_range(0..self.width)] = false;
        } else {
            self.walls_v[rng.gen_range(0..self.height)][0] = false;
        }
    }
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            width,
            height,
            walls_v: vec![vec![true; width + 1]; height],
            walls_h: vec![vec![true; width]; height + 1],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn generate(&mut self, seed: u64) {
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut path: Vec<(usize, usize)> = Vec::with_capacity(self.width * self.height);
        let mut current_row = rng.gen_range(0..self.height);
        let mut current_col = rng.gen_range(0..self.width);
        path.push((current_row, current_col));
        self.set_entry_and_exit(&mut rng);
        loop {
            if path.is_empty() {
                break;
            }
            (current_row, current_col) = path[path.len() - 1];
            visited[current_row][current_col] = true;
            let neighbors = self.get_neighbours(&visited, current_row, current_col);

            match neighbors.choose(&mut rng) {
                Some((row, col)) => {
                    self.remove_wall(current_row, current_col, *row, *col);
                    path.push((*row, *col));
                }
                None => {
                    path.pop();
                    continue;
                }
            }
        }
    }

    pub fn has_north_wall(&self, row: usize, column: usize) -> bool {
        self.walls_h[row][column]
    }

    pub fn has_south_wall(&self, row: usize, column: usize) -> bool {
        self.walls_h[row + 1][column]
    }

    pub fn has_west_wall(&self, row: usize, column: usize) -> bool {
        self.walls_v[row][column]
    }

    pub fn has_east_wall(&self, row: usize, column: usize) -> bool {
        self.walls_v[row][column + 1]
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for column in 0..self.width {
                if self.has_north_wall(row, column) {
                    write!(f, "+---")?;
                } else {
                    write!(f, "+   ")?;
                }
            }
            write!(f, "+\n")?;
            for column in 0..self.width {
                if self.has_west_wall(row, column) {
                    write!(f, "|   ")?;
                } else {
                    write!(f, "    ")?;
                }
            }
            if self.has_east_wall(row, self.width - 1) {
                write!(f, "|")?;
            }
            write!(f, "\n")?;
        }
        for column in 0..self.width {
            if self.has_south_wall(self.height - 1, column) {
                write!(f, "+---")?;
            }
        }
        write!(f, "+")?;
        Ok(())
    }
}
