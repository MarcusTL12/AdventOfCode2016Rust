use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn search_ranges(ranges: &[(u32, u32)], n: u32) -> Result<usize, usize> {
    ranges.binary_search_by(|(a, b)| {
        if *a > n {
            Ordering::Greater
        } else if *b < n {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
}

fn union_into(
    ranges: &[(u32, u32)],
    nrange: (u32, u32),
    target: &mut Vec<(u32, u32)>,
) {
    let a = search_ranges(ranges, nrange.0);
    let b = search_ranges(ranges, nrange.1);
    //
    let mut mid = (
        match a {
            Err(_) => nrange.0,
            Ok(i) => ranges[i].0,
        },
        match b {
            Err(_) => nrange.1,
            Ok(i) => ranges[i].1,
        },
    );
    //
    let start = {
        let mut stopind = match a {
            Ok(a) => a,
            Err(a) => a,
        };
        if stopind > 0 && ranges[stopind - 1].1 + 1 == nrange.0 {
            stopind -= 1;
            mid.0 = ranges[stopind].0;
        }
        ranges[0..stopind].iter().cloned()
    };
    //
    let end = {
        let mut startind = match b {
            Ok(b) => b + 1,
            Err(b) => b,
        };
        if startind < ranges.len() && ranges[startind].0 == nrange.1 + 1 {
            mid.1 = ranges[startind].1;
            startind += 1;
        }
        ranges[startind..].iter().cloned()
    };
    //
    target.clear();
    target.extend(start.chain(iter::once(mid)).chain(end));
}

fn part1() {
    let (buf, _) =
        BufReader::new(File::open("inputfiles/day20/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Ok([a, b]) = l
                    .split('-')
                    .map(|n| n.parse().unwrap())
                    .collect::<ArrayVec<_, 2>>()
                    .into_inner()
                {
                    (a, b)
                } else {
                    panic!()
                }
            })
            .fold((Vec::new(), Vec::new()), |(buf1, mut buf2), x| {
                union_into(&buf1, x, &mut buf2);
                (buf2, buf1)
            });
    //
    println!("{}", buf[0].1 + 1);
}

fn part2() {
    let ans = u32::max_value()
        - BufReader::new(File::open("inputfiles/day20/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Ok([a, b]) = l
                    .split('-')
                    .map(|n| n.parse().unwrap())
                    .collect::<ArrayVec<_, 2>>()
                    .into_inner()
                {
                    (a, b)
                } else {
                    panic!()
                }
            })
            .fold((Vec::new(), Vec::new()), |(buf1, mut buf2), x| {
                union_into(&buf1, x, &mut buf2);
                (buf2, buf1)
            })
            .0
            .into_iter()
            .map(|(a, b)| b - a + 1)
            .sum::<u32>()
        + 1;
    //
    println!("{}", ans);
}
