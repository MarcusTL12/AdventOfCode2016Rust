use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let reg = Regex::new(r"([a-z-]+)-(\d+)\[(\w+)\]").unwrap();
    //
    let ans: u32 =
        BufReader::new(File::open("inputfiles/day4/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .filter_map(|l| {
                if let Some(c) = reg.captures(&l) {
                    let id: u32 = c[2].parse().unwrap();
                    //
                    if c[1]
                        .split('-')
                        .flat_map(|s| s.chars())
                        .fold(HashMap::new(), |mut map, c| {
                            if let Some(x) = map.get_mut(&c) {
                                *x += 1;
                            } else {
                                map.insert(c, 1);
                            }
                            map
                        })
                        .iter()
                        .sorted_by(|(a, _), (b, _)| a.cmp(b))
                        .sorted_by(|(_, a), (_, b)| b.cmp(a))
                        .map(|(x, _)| x)
                        .take(5)
                        .collect::<String>()
                        == c[3]
                    {
                        Some(id)
                    } else {
                        None
                    }
                } else {
                    panic!()
                }
            })
            .sum();
    //
    println!("{:?}", ans);
}

fn part2() {
    let reg = Regex::new(r"([a-z-]+)-(\d+)\[(\w+)\]").unwrap();
    //
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    //
    let ans = BufReader::new(File::open("inputfiles/day4/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|l| {
            if let Some(c) = reg.captures(&l) {
                let id: u32 = c[2].parse().unwrap();
                //
                if c[1]
                    .split('-')
                    .flat_map(|s| s.chars())
                    .fold(HashMap::new(), |mut map, c| {
                        if let Some(x) = map.get_mut(&c) {
                            *x += 1;
                        } else {
                            map.insert(c, 1);
                        }
                        map
                    })
                    .iter()
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .sorted_by(|(_, a), (_, b)| b.cmp(a))
                    .map(|(x, _)| x)
                    .take(5)
                    .collect::<String>()
                    == c[3]
                {
                    Some((c[1].to_owned(), id))
                } else {
                    None
                }
            } else {
                panic!()
            }
        })
        .filter_map(|(s, id)| {
            if s.chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        alphabet[(((c as u8 - b'a') as u32 + id) as usize)
                            % alphabet.len()]
                    }
                })
                .take(5)
                .eq("north".chars())
            {
                Some(id)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    //
    println!("{}", ans);
}
