use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn nextrow(row: &[bool], target: &mut Vec<bool>) {
    target.clear();
    target.extend(
        iter::once(row[1])
            .chain(row.iter().tuple_windows().map(|(a, _, b)| a != b))
            .chain(iter::once(row[row.len() - 2])),
    );
}

fn _showrow<T: Iterator<Item = bool>>(row: T) {
    for b in row {
        print!("{}", if b { '^' } else { '.' });
    }
    println!();
}

fn part1() {
    let inp: Vec<_> =
        BufReader::new(File::open("inputfiles/day18/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => false,
                '^' => true,
                _ => panic!("Invalid tile: {}", c),
            })
            .collect();
    //
    let (_, _, ans) =
        (0..40).fold((inp, Vec::new(), 0), |(a, mut b, mut sum), _| {
            sum += a.iter().filter(|&x| !x).count();
            nextrow(&a, &mut b);
            (b, a, sum)
        });
    //
    println!("{}", ans);
}

fn part2() {
    let inp: Vec<_> =
        BufReader::new(File::open("inputfiles/day18/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => false,
                '^' => true,
                _ => panic!("Invalid tile: {}", c),
            })
            .collect();
    //
    let (_, _, ans) =
        (0..400_000).fold((inp, Vec::new(), 0), |(a, mut b, mut sum), _| {
            sum += a.iter().filter(|&x| !x).count();
            nextrow(&a, &mut b);
            (b, a, sum)
        });
    //
    println!("{}", ans);
}
