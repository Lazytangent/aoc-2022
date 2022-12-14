use std::collections::HashMap;

pub struct Grid {
    pub grid: HashMap<(usize, usize), char>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    pub ymax: usize,
    pub floor: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            xmin: 500,
            xmax: 500,
            ymin: 0,
            ymax: 0,
            floor: 0,
        }
    }

    pub fn put(&mut self, x: usize, y: usize, c: char) {
        if x > self.xmax {
            self.xmax = x
        }
        if x < self.xmin {
            self.xmin = x
        }

        if y > self.ymax {
            self.ymax = y;
        }
        if y < self.ymin {
            self.ymin = y;
        }

        self.grid.insert((x, y), c);
    }

    pub fn show(&mut self) {
        for y in self.ymin..self.ymax + 1 {
            let mut line = String::new();

            for x in self.xmin..self.xmax + 1 {
                if !self.grid.contains_key(&(x, y)) {
                    let c = self.choose_char(y);
                    self.put(x, y, c);
                }

                line.push(self.grid[&(x, y)]);
            }

            println!("{line}");
        }
    }

    pub fn drop(&mut self, x: usize, y: usize, cap: usize) -> bool {
        let (mut x, mut y) = (x, y);

        while y < cap {
            if !self.grid.contains_key(&(x - 1, y + 1)) {
                self.put(x - 1, y + 1, self.choose_char(y + 1));
            }
            if !self.grid.contains_key(&(x, y + 1)) {
                self.put(x, y + 1, self.choose_char(y + 1));
            }
            if !self.grid.contains_key(&(x + 1, y + 1)) {
                self.put(x + 1, y + 1, self.choose_char(y + 1));
            }

            if self.grid[&(x, y + 1)] == '.' {
                y += 1;
            } else {
                if self.grid[&(x - 1, y + 1)] == '.' {
                    x -= 1;
                    y += 1;
                } else if self.grid[&(x + 1, y + 1)] == '.' {
                    x += 1;
                    y += 1;
                } else {
                    self.put(x, y, 'o');
                    return true;
                }
            }
        }

        false
    }

    fn choose_char(&self, y: usize) -> char {
        if y >= self.floor {
            '#'
        } else {
            '.'
        }
    }

    pub fn read(&mut self, file: &str) {
        for line in file.lines() {
            let pairs = line.trim_end().split(" -> ");
            let mut first = true;
            let (mut px, mut py) = (0, 0);

            for pair in pairs {
                let coords: Vec<usize> = pair
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let (x, y) = (coords[0], coords[1]);

                if first {
                    (px, py) = (x, y);
                    first = false;
                    continue;
                }

                if px == x {
                    if py < y {
                        for yy in py..y+1 {
                            self.put(x, yy, '#');
                        }
                    } else {
                        for yy in y..py+1 {
                            self.put(x, yy, '#');
                        }
                    }
                } else if py == y {
                    if px < x {
                        for xx in px..x+1 {
                            self.put(xx, y, '#');
                        }
                    } else {
                        for xx in x..px+1 {
                            self.put(xx, y, '#');
                        }
                    }
                }

                (px, py) = (x, y);
            }
        }

        self.floor = self.ymax + 2;
    }
}
