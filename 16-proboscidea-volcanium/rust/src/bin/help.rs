use std::{collections::HashMap, env};

use ndarray::Array3;

const SMALL: &str = include_str!("../../../data/small.txt");
const FULL: &str = include_str!("../../../data/full.txt");

fn main() {
    let use_real_data = match env::args().nth(1) {
        Some(v) => {
            println!("{v}");
            v == "-r"
        }
        None => false,
    };
    let input = match use_real_data {
        true => FULL,
        false => SMALL,
    };

    let mut valves: Vec<(&str, u16, Vec<&str>)> = vec![];
    for line in input.trim().split('\n') {
        let (valve, flow, _, tunnels) = sscanf::sscanf!(
            line,
            "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}"
        )
        .unwrap();

        let tunnels: Vec<_> = tunnels.split(", ").collect();
        valves.push((valve, flow, tunnels));
    }

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let lab_2_idx: HashMap<_, _> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj: Vec<Vec<usize>> = vec![vec![0; 0]; n];
    let mut flow: Vec<u16> = vec![0; n];

    for v in valves.iter() {
        let i = lab_2_idx[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(lab_2_idx[w]);
        }
    }

    let aa = lab_2_idx["AA"];
    let mm = 1 << m;

    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    let res = opt[(29, aa, mm - 1)];
    println!("Part one: {res}");

    let mut best = 0;
    for x in 0..mm {
        for y in 0..x {
            if (x & y) == 0 {
                best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
            }
        }
    }
    println!("Part two: {best}");
}
