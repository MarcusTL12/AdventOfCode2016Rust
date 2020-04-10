use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut v: VecDeque<_> = "abcdefgh".chars().collect();
    //
    for l in BufReader::new(File::open("inputfiles/day21/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        match l.split(' ').collect::<ArrayVec<[_; 7]>>().as_slice() {
            ["swap", "position", x, "with", "position", y] => {
                v.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            ["swap", "letter", x, "with", "letter", y] => {
                let x = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == x.chars().next().unwrap())
                    .unwrap()
                    .0;
                let y = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == y.chars().next().unwrap())
                    .unwrap()
                    .0;
                v.swap(x, y);
            }
            ["rotate", lr, x, _] => match *lr {
                "left" => {
                    for _ in 0..x.parse().unwrap() {
                        let c = v.pop_front().unwrap();
                        v.push_back(c);
                    }
                }
                "right" => {
                    for _ in 0..x.parse().unwrap() {
                        let c = v.pop_back().unwrap();
                        v.push_front(c);
                    }
                }
                _ => unreachable!(),
            },
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                let x = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == x.chars().next().unwrap())
                    .unwrap()
                    .0;
                //
                let x = x + 1 + if x >= 4 { 1 } else { 0 };
                //
                for _ in 0..x {
                    let c = v.pop_back().unwrap();
                    v.push_front(c);
                }
            }
            ["reverse", "positions", x, "through", y] => {
                let mut x = x.parse().unwrap();
                let mut y = y.parse().unwrap();
                //
                while x < y {
                    v.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            }
            ["move", "position", x, "to", "position", y] => {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                //
                let c = v.remove(x).unwrap();
                v.insert(y, c);
            }
            _ => unreachable!("{}", l),
        }
    }
    //
    let ans: String = v.into_iter().collect();
    //
    println!("{}", ans);
}

fn part2() {
    let mut v: VecDeque<_> = "fbgdceah".chars().collect();
    //
    let invrotate: Vec<_> = (0..v.len())
        .map(|x| ((2 * x + 1 + if x >= 4 { 1 } else { 0 }) % v.len(), x))
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, x)| x)
        .collect();
    //
    for l in BufReader::new(File::open("inputfiles/day21/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        match l.split(' ').collect::<ArrayVec<[_; 7]>>().as_slice() {
            ["swap", "position", x, "with", "position", y] => {
                v.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            ["swap", "letter", x, "with", "letter", y] => {
                let x = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == x.chars().next().unwrap())
                    .unwrap()
                    .0;
                let y = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == y.chars().next().unwrap())
                    .unwrap()
                    .0;
                v.swap(x, y);
            }
            ["rotate", lr, x, _] => match *lr {
                "right" => {
                    for _ in 0..x.parse().unwrap() {
                        let c = v.pop_front().unwrap();
                        v.push_back(c);
                    }
                }
                "left" => {
                    for _ in 0..x.parse().unwrap() {
                        let c = v.pop_back().unwrap();
                        v.push_front(c);
                    }
                }
                _ => unreachable!(),
            },
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                let x = v
                    .iter()
                    .enumerate()
                    .find(|(_, &v)| v == x.chars().next().unwrap())
                    .unwrap()
                    .0;
                //
                let x = invrotate[x];
                let x = x + 1 + if x >= 4 { 1 } else { 0 };
                //
                for _ in 0..x {
                    let c = v.pop_front().unwrap();
                    v.push_back(c);
                }
            }
            ["reverse", "positions", x, "through", y] => {
                let mut x = x.parse().unwrap();
                let mut y = y.parse().unwrap();
                //
                while x < y {
                    v.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            }
            ["move", "position", x, "to", "position", y] => {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                //
                let c = v.remove(y).unwrap();
                v.insert(x, c);
            }
            _ => unreachable!("{}", l),
        }
    }
    //
    let ans: String = v.into_iter().collect();
    //
    println!("{}", ans);
}
