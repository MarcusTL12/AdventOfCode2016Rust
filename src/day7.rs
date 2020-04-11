use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn is_tls(s: &str) -> bool {
    let (_, a, b) = s.chars().tuple_windows().fold(
        (false, false, false),
        |(mut inside, mut abba, mut abba_inside), (a, b, c, d)| {
            if a == '[' {
                inside = true;
            } else if a == ']' {
                inside = false;
            } else if a == d && b == c && a != b {
                if inside {
                    abba_inside = true;
                } else {
                    abba = true;
                }
            }
            (inside, abba, abba_inside)
        },
    );
    a && !b
}

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day7/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| is_tls(&l))
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let temp =
        BufReader::new(File::open("inputfiles/day7/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| {
                let (aba, bab, _) = l.chars().tuple_windows().fold(
                    (HashSet::new(), HashSet::new(), false),
                    |(mut aba, mut bab, mut inside), (a, b, c)| {
                        if a == '[' {
                            inside = true;
                        } else if a == ']' {
                            inside = false;
                        } else if a == c && a != b {
                            if inside {
                                bab.insert((a, b));
                            } else {
                                aba.insert((a, b));
                            }
                        }
                        (aba, bab, inside)
                    },
                );
                aba.into_iter().any(|(a, b)| bab.contains(&(b, a)))
            })
            .count();
    //
    println!("{}", temp);
}
