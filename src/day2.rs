use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use num::Complex;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

fn pos_to_num(pos: Complex<i32>) -> Option<i32> {
    if pos.re >= 0 && pos.re < 3 && pos.im >= 0 && pos.im < 3 {
        Some(pos.im * 3 + pos.re + 1)
    } else {
        None
    }
}

lazy_static! {
    static ref KEYPAD_2: HashMap<Complex<i32>, char> = [
        (Complex { re: 2, im: 0 }, '1'),
        (Complex { re: 1, im: 1 }, '2'),
        (Complex { re: 2, im: 1 }, '3'),
        (Complex { re: 3, im: 1 }, '4'),
        (Complex { re: 0, im: 2 }, '5'),
        (Complex { re: 1, im: 2 }, '6'),
        (Complex { re: 2, im: 2 }, '7'),
        (Complex { re: 3, im: 2 }, '8'),
        (Complex { re: 4, im: 2 }, '9'),
        (Complex { re: 1, im: 3 }, 'A'),
        (Complex { re: 2, im: 3 }, 'B'),
        (Complex { re: 3, im: 3 }, 'C'),
        (Complex { re: 2, im: 4 }, 'D'),
    ]
    .iter()
    .cloned()
    .collect();
}

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .scan(Complex { re: 1, im: 1 }, |start, l| {
            pos_to_num(
                l.chars()
                    .map(|c| match c {
                        'R' => Complex { re: 1, im: 0 },
                        'L' => Complex { re: -1, im: 0 },
                        'D' => Complex { re: 0, im: 1 },
                        'U' => Complex { re: 0, im: -1 },
                        _ => panic!(),
                    })
                    .fold(*start, |pos, d| {
                        if pos_to_num(pos + d).is_some() {
                            *start += d;
                            pos + d
                        } else {
                            pos
                        }
                    }),
            )
        })
        .fold(0, |s, x| 10 * s + x);
    //
    println!("{}", ans);
}

fn part2() {
    let ans: String =
        BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .scan(Complex { re: 0, im: 2 }, |start, l| {
                KEYPAD_2.get(
                    &l.chars()
                        .map(|c| match c {
                            'R' => Complex { re: 1, im: 0 },
                            'L' => Complex { re: -1, im: 0 },
                            'D' => Complex { re: 0, im: 1 },
                            'U' => Complex { re: 0, im: -1 },
                            _ => panic!(),
                        })
                        .fold(*start, |pos, d| {
                            if KEYPAD_2.get(&(pos + d)).is_some() {
                                *start += d;
                                pos + d
                            } else {
                                pos
                            }
                        }),
                )
            })
            .collect();
    //
    println!("{}", ans);
}
