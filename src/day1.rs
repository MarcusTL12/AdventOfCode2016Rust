use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: Complex<_> =
        BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(", ")
            .scan(Complex { re: 0, im: 1 }, |d, x| {
                *d *= match &x[0..1] {
                    "R" => Complex { re: 0, im: -1 },
                    "L" => Complex { re: 0, im: 1 },
                    _ => panic!(),
                };
                Some(*d * x[1..].parse::<i32>().unwrap())
            })
            .sum();
    //
    println!("{}", ans.re.abs() + ans.im.abs());
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .scan(Complex { re: 0, im: 1 }, |d, x| {
            *d *= match &x[0..1] {
                "R" => Complex { re: 0, im: -1 },
                "L" => Complex { re: 0, im: 1 },
                _ => panic!(),
            };
            Some((*d, x[1..].parse::<i32>().unwrap()))
        })
        .scan(Complex { re: 0, im: 0 }, |p, (d, n)| {
            let thispos = *p;
            let it = (0..n).map(move |i| thispos + d * i);
            *p += d * n;
            Some(it)
        })
        .flatten()
        .scan(HashSet::new(), |visited, x| {
            if visited.contains(&x) {
                Some(Some(x))
            } else {
                visited.insert(x);
                Some(None)
            }
        })
        .filter_map(|x| x)
        .next()
        .unwrap();
    //
    println!("{}", ans.re.abs() + ans.im.abs());
}
