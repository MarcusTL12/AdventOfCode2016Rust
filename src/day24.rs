use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use num::Complex;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> (Vec<Vec<bool>>, Vec<Complex<i16>>) {
    let mut nodes = Vec::new();
    //
    let maze = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => false,
                    '#' => true,
                    '0'..='9' => {
                        nodes.push((
                            c as u8 - b'0',
                            Complex {
                                re: j as i16,
                                im: i as i16,
                            },
                        ));
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    //
    let nodes = nodes
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, x)| *x)
        .collect::<Vec<_>>();
    //
    (maze, nodes)
}

fn _render_maze(maze: &Vec<Vec<bool>>) {
    for row in maze {
        for cell in row {
            print!(
                "{}",
                match cell {
                    false => '.',
                    true => '#',
                }
            )
        }
        println!();
    }
}

fn get_cell(maze: &Vec<Vec<bool>>, pos: Complex<i16>) -> bool {
    maze[pos.im as usize][pos.re as usize]
}

fn bfs(
    maze: &Vec<Vec<bool>>,
    from: Complex<i16>,
    targets: &[Complex<i16>],
) -> Vec<usize> {
    let mut lengths = vec![0; targets.len()];
    let mut amt_lens = targets.len();
    let m: HashMap<_, _> =
        targets.iter().enumerate().map(|(a, b)| (b, a)).collect();
    //
    const DIRS: [Complex<i16>; 4] = [
        Complex { re: 1, im: 0 },
        Complex { re: -1, im: 0 },
        Complex { re: 0, im: 1 },
        Complex { re: 0, im: -1 },
    ];
    //
    if let Some(x) = m.get(&from) {
        lengths[*x] = 0;
        amt_lens -= 1;
    }
    //
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    //
    let mut visited = HashSet::new();
    visited.insert(from);
    //
    while let Some((pos, len)) = queue.pop_front() {
        for npos in DIRS.iter().map(|d| d + pos) {
            if !get_cell(maze, npos) && !visited.contains(&npos) {
                if let Some(x) = m.get(&npos) {
                    lengths[*x] = len + 1;
                    amt_lens -= 1;
                    if amt_lens == 0 {
                        return lengths;
                    }
                }
                queue.push_back((npos, len + 1));
                visited.insert(npos);
            }
        }
    }
    //
    panic!("Did not find all paths from {}", from)
}

fn allpaths(
    maze: &Vec<Vec<bool>>,
    nodes: &Vec<Complex<i16>>,
) -> Vec<Vec<usize>> {
    nodes.iter().map(|&x| bfs(maze, x, nodes)).collect()
}

fn part1() {
    let (maze, nodes) = load_input("inputfiles/day24/input.txt");
    //
    let paths = allpaths(&maze, &nodes);
    //
    let ans = (1..nodes.len())
        .permutations(nodes.len() - 1)
        .map(|p| {
            p.into_iter().fold((0, 0), |(prev, sum), next| {
                (next, sum + paths[prev][next])
            })
        })
        .map(|(_, x)| x)
        .min()
        .unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let (maze, nodes) = load_input("inputfiles/day24/input.txt");
    //
    let paths = allpaths(&maze, &nodes);
    //
    let ans = (1..nodes.len())
        .permutations(nodes.len() - 1)
        .map(|p| {
            p.into_iter().fold((0, 0), |(prev, sum), next| {
                (next, sum + paths[prev][next])
            })
        })
        .map(|(last, len)| len + paths[last][0])
        .min()
        .unwrap();
    //
    println!("{}", ans);
}
