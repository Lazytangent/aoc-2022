use std::collections::HashSet;

const P: [&[(i32, i32)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)]
];

pub fn part_two(contents: &str) -> usize {
    unimplemented!()
}

fn solve(count: usize) -> usize {
    unimplemented!()
}

fn free(t: &HashSet<(i32, i32)>, x: i32, y: i32) -> bool {
    x >= 0 && x < 7 && y > 0 && !t.contains(&(x, y))
}

fn can_move(t: &HashSet<(i32, i32)>, piece: usize, x: i32, y: i32) -> bool {
    for (dx, dy) in P[piece] {
        if !free(t, x + dx, y + dy) {
            return false;
        }
    }

    true
}

fn place(input: &[char], t: &mut HashSet<(i32, i32)>, jet: usize, piece: usize, max_y: i32) -> (usize, usize, i32) {
    let (mut x, mut y) = (2, max_y + 5);
    let mut jet = jet;

    while can_move(t, piece, x, y - 1) {
        y -= 1;
        if input[jet] == '<' && can_move(t, piece, x - 1, y) {
            x -= 1;
        }
        if input[jet] == '>' && can_move(t, piece, x + 1, y) {
            x += 1;
        }
        jet = (jet + 1) % input.len();
    }

    let mut new_cells = vec![];
    for (dx, dy) in P[piece] {
        new_cells.push((x + dx, y + dy));
    }
    t.extend(&new_cells);

    let mut new_max_y = max_y;

    for (_, y) in new_cells {
        new_max_y = new_max_y.max(y);
    }

    (jet, (piece + 1) % P.len(), new_max_y)
}
