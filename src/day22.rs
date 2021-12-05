use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use arrayvec::ArrayVec;

use lazy_static::*;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<Vec<[usize; 3]>> {
    lazy_static! {
        static ref REG: Regex = Regex::new(concat!(
            r"/dev/grid/node-x(\d+)-y(\d+)\s+",
            r"(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%"
        ))
        .unwrap();
    }
    //
    let mut xlen = 0;
    let mut ylen = 0;
    //
    let inp: HashMap<_, _> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .skip(2)
        .map(|l| l.unwrap())
        .map(|l| {
            if let Some(c) = REG.captures(&l) {
                let nums = (1..=5)
                    .map(|i| c[i].parse::<usize>().unwrap())
                    .collect::<ArrayVec<_, 5>>();
                //
                let ind = (nums[0], nums[1]);
                let data = [nums[2], nums[3], nums[4]];
                //
                xlen = max(xlen, ind.0);
                ylen = max(ylen, ind.1);
                //
                (ind, data)
            } else {
                unreachable!("{}", l)
            }
        })
        .collect();
    //
    (0..ylen + 1)
        .map(|i| (0..xlen + 1).map(|j| inp[&(j, i)]).collect())
        .collect()
}

fn part1() {
    let inp = load_input("inputfiles/day22/input.txt");
    //
    let ans = inp
        .iter()
        .flat_map(|row| row.iter())
        .tuple_combinations()
        .filter(|(a, b)| {
            (a[1] != 0 && a[1] <= b[2]) || (b[1] != 0 && b[1] <= a[2])
        })
        .count();
    //
    println!("{}", ans);
}

fn _show_grid(grid: &Vec<Vec<[usize; 3]>>) {
    let mincap = grid
        .iter()
        .flat_map(|row| row.iter())
        .min_by(|a, b| a[0].cmp(&b[0]))
        .unwrap()[0];
    //
    println!("{}", mincap);
    //
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!(
                "{}",
                match grid[i][j] {
                    [_, 0, _] => '_',
                    [_, x, _] =>
                        if x > mincap {
                            '#'
                        } else {
                            '.'
                        },
                }
            )
        }
        println!();
    }
}

fn part2() {
    let inp = load_input("inputfiles/day22/input.txt");
    //
    _show_grid(&inp);
    // Then solved by hand
    println!("I appearently solved this by hand at this point");
}
