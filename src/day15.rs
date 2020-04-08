use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn part1() {
    let reg = Regex::new(
        r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).",
    )
    .unwrap();
    //
    let (r, m): (Vec<i64>, Vec<i64>) =
        BufReader::new(File::open("inputfiles/day15/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let c = reg.captures(&l).unwrap();
                (c[2].parse::<i64>().unwrap(), c[1].parse().unwrap())
            })
            .enumerate()
            .fold((Vec::new(), Vec::new()), |(mut r, mut m), (i, (a, b))| {
                r.push((-(a + i as i64 + 1) % b) + b);
                m.push(b);
                (r, m)
            });
    //
    let ans = chinese_remainder(&r, &m).unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let reg = Regex::new(
        r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).",
    )
    .unwrap();
    //
    let (r, m): (Vec<i64>, Vec<i64>) =
        BufReader::new(File::open("inputfiles/day15/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let c = reg.captures(&l).unwrap();
                (c[2].parse::<i64>().unwrap(), c[1].parse().unwrap())
            })
            .chain(iter::once((0, 11)))
            .enumerate()
            .fold((Vec::new(), Vec::new()), |(mut r, mut m), (i, (a, b))| {
                r.push((-(a + i as i64 + 1) % b) + b);
                m.push(b);
                (r, m)
            });
    //
    let ans = chinese_remainder(&r, &m).unwrap();
    //
    println!("{}", ans);
}
