use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day3/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            if let Ok(v) = l
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<ArrayVec<[_; 3]>>()
                .into_inner()
            {
                let s: u32 = v.iter().sum();
                v.iter().all(|&x| 2 * x < s)
            } else {
                panic!()
            }
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day3/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .map(|l| {
            let lines: [[u32; 3]; 3] = l
                .map(|l| {
                    l.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<ArrayVec<_>>()
                        .into_inner()
                        .unwrap()
                })
                .collect::<ArrayVec<_>>()
                .into_inner()
                .unwrap();
            //
            (0..3).map(move |i| -> [u32; 3] {
                (0..3)
                    .map(|j| lines[j][i])
                    .collect::<ArrayVec<_>>()
                    .into_inner()
                    .unwrap()
            })
        })
        .flatten()
        .filter(|v| {
            let s: u32 = v.iter().sum();
            v.iter().all(|&x| 2 * x < s)
        })
        .count();
    //
    println!("{}", ans);
}
