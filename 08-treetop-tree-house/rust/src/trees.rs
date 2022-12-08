use std::collections::HashSet;

#[derive(Debug)]
pub struct Trees {
    matrix: Vec<Vec<u32>>,
    pub visible: HashSet<(usize, usize)>,
}

impl Trees {
    fn visible_top(&mut self) {
        let mut set = self.visible.clone();
        for col in 0..self.matrix[0].len() {
            let mut tallest = 0;
            for row in 0..self.matrix.len() {
                let h = self.matrix[row][col];
                if h > tallest || row == 0 {
                    tallest = h;
                    set.insert((row, col));
                }
            }
        }

        self.visible = set;
    }

    fn visible_bottom(&mut self) {
        let mut set = self.visible.clone();
        let matrix_height = self.matrix.len();

        for col in 0..self.matrix[0].len() {
            let mut tallest = 0;
            for row in (0..self.matrix.len()).rev() {
                let h = self.matrix[row][col];
                if h > tallest || row == matrix_height - 1 {
                    tallest = h;
                    set.insert((row, col));
                }
            }
        }

        self.visible = set;
    }

    fn visible_right(&mut self) {
        let mut set = self.visible.clone();
        let matrix_width = self.matrix[0].len();

        for row in 0..self.matrix.len() {
            let mut tallest = 0;
            for col in (0..matrix_width).rev() {
                let h = self.matrix[row][col];
                if h > tallest || col == matrix_width - 1 {
                    tallest = h;
                    set.insert((row, col));
                }
            }
        }

        self.visible = set;
    }

    fn visible_left(&mut self) {
        let mut set = self.visible.clone();
        let matrix_width = self.matrix[0].len();

        for row in 0..self.matrix.len() {
            let mut tallest = 0;
            for col in 0..matrix_width {
                let h = self.matrix[row][col];
                if h > tallest || col == 0 {
                    tallest = h;
                    set.insert((row, col));
                }
            }
        }

        self.visible = set;
    }

    pub fn find_all_visible_trees(&mut self) {
        self.visible_top();
        self.visible_bottom();
        self.visible_right();
        self.visible_left();
    }

    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self {
            visible: HashSet::new(),
            matrix,
        }
    }

    fn see_up(&self, pos: (usize, usize)) -> usize {
        let mut score = 0;
        let val = self.matrix[pos.0][pos.1];
        for row in (0..pos.0).rev() {
            score += 1;
            if val <= self.matrix[row][pos.1] {
                break;
            }
        }

        score
    }

    fn see_down(&self, pos: (usize, usize)) -> usize {
        let mut score = 0;
        let val = self.matrix[pos.0][pos.1];
        let matrix_height = self.matrix.len();

        for row in pos.0 + 1..matrix_height {
            score += 1;
            if val <= self.matrix[row][pos.1] {
                break;
            }
        }

        score
    }

    fn see_left(&self, pos: (usize, usize)) -> usize {
        let mut score = 0;
        let val = self.matrix[pos.0][pos.1];

        for col in (0..pos.1).rev() {
            score += 1;
            if val <= self.matrix[pos.0][col] {
                break;
            }
        }

        score
    }

    fn see_right(&self, pos: (usize, usize)) -> usize {
        let mut score = 0;
        let val = self.matrix[pos.0][pos.1];
        let matrix_width = self.matrix[0].len();

        for col in (pos.1 + 1)..matrix_width {
            score += 1;
            if val <= self.matrix[pos.0][col] {
                break;
            }
        }

        score
    }

    fn get_scenic_score(&self, pos: (usize, usize)) -> usize {
        let top = self.see_up(pos);
        let bottom = self.see_down(pos);
        let left = self.see_left(pos);
        let right = self.see_right(pos);

        top * bottom * left * right
    }

    pub fn find_best_scenic_score(&self) -> usize {
        let mut max_score = 0;
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[0].len() {
                let rating = self.get_scenic_score((row, col));
                if rating > max_score {
                    max_score = rating;
                }
            }
        }

        max_score
    }
}
