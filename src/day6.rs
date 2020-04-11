use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: String =
        BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .fold(Vec::new(), |mut counter, l| {
                if counter.len() == 0 {
                    counter = vec![HashMap::new(); l.len()]
                }
                for (c, m) in l.chars().zip(counter.iter_mut()) {
                    if let Some(x) = m.get_mut(&c) {
                        *x += 1;
                    } else {
                        m.insert(c, 1);
                    }
                }
                counter
            })
            .into_iter()
            .map(|m| m.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap())
            .map(|(x, _)| x)
            .collect();
    //
    println!("{}", ans);
}

fn part2() {
    let ans: String =
        BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .fold(Vec::new(), |mut counter, l| {
                if counter.len() == 0 {
                    counter = vec![HashMap::new(); l.len()]
                }
                for (c, m) in l.chars().zip(counter.iter_mut()) {
                    if let Some(x) = m.get_mut(&c) {
                        *x += 1;
                    } else {
                        m.insert(c, 1);
                    }
                }
                counter
            })
            .into_iter()
            .map(|m| m.into_iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap())
            .map(|(x, _)| x)
            .collect();
    //
    println!("{}", ans);
}
