use std::collections::HashSet;

use utils::{self, fs::DataType};

pub fn solve(r#type: DataType) {
    let contents = utils::fs::read_data(r#type);

    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        matrix.push(
            line.chars()
                .filter_map(|x| x.to_digit(10))
                .collect(),
        );
    }

    let mut trees = Trees::new(matrix);
    trees.find_all_visible_trees();

    println!("{}", trees.visible.len());
}

#[derive(Debug)]
struct Trees {
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
}
