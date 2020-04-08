use std::collections::{HashSet, VecDeque};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: i32 = 1350;

fn iswall(pos: Complex<i32>) -> bool {
    let x = pos.re;
    let y = pos.im;
    //
    if x < 0 || y < 0 {
        return true;
    }
    //
    let n = x * x + 3 * x + 2 * x * y + y + y * y + INPUT;
    //
    (0..)
        .scan(n, |n, _| {
            if *n > 0 {
                let bit = *n % 2;
                //
                *n >>= 1;
                //
                Some(bit)
            } else {
                None
            }
        })
        .sum::<i32>()
        % 2
        != 0
}

fn bfs1(pos: Complex<i32>, target: Complex<i32>) -> usize {
    const DIRS: [Complex<i32>; 4] = [
        Complex { re: 1, im: 0 },
        Complex { re: -1, im: 0 },
        Complex { re: 0, im: 1 },
        Complex { re: 0, im: -1 },
    ];
    //
    let mut queue = VecDeque::new();
    queue.push_back((pos, 0));
    //
    let mut visited = HashSet::new();
    visited.insert(pos);
    //
    while let Some((pos, len)) = queue.pop_front() {
        for npos in DIRS.iter().map(|d| pos + d) {
            if !visited.contains(&npos) && !iswall(npos) {
                if npos == target {
                    return len + 1;
                }
                queue.push_back((npos, len + 1));
                visited.insert(npos);
            }
        }
    }
    //
    panic!("Did not find path!");
}

fn part1() {
    let ans = bfs1(Complex { re: 1, im: 1 }, Complex { re: 31, im: 39 });
    //
    println!("{}", ans);
}

fn bfs2(pos: Complex<i32>, maxlen: usize) -> usize {
    const DIRS: [Complex<i32>; 4] = [
        Complex { re: 1, im: 0 },
        Complex { re: -1, im: 0 },
        Complex { re: 0, im: 1 },
        Complex { re: 0, im: -1 },
    ];
    //
    let mut queue = VecDeque::new();
    queue.push_back((pos, 0));
    //
    let mut visited = HashSet::new();
    visited.insert(pos);
    //
    while let Some((pos, len)) = queue.pop_front() {
        for npos in DIRS.iter().map(|d| pos + d) {
            if len < maxlen && !visited.contains(&npos) && !iswall(npos) {
                queue.push_back((npos, len + 1));
                visited.insert(npos);
            }
        }
    }
    //
    visited.len()
}

fn part2() {
    let ans = bfs2(Complex { re: 1, im: 1 }, 50);
    //
    println!("{}", ans);
}
