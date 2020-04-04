use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let inp = BufReader::new(File::open("inputfiles/day9/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    //
    let ans: usize = inp
        .chars()
        .scan(
            (false, false, String::new(), Vec::new(), (0, 0)),
            |(tag, data, tag_buf, data_buf, tag_data), c| {
                if *tag {
                    if *data {
                        if tag_data.0 > 1 {
                            data_buf.push(c);
                            tag_data.0 -= 1;
                            Some(0)
                        } else {
                            *tag = false;
                            *data = false;
                            data_buf.push(c);
                            //
                            Some(data_buf.len() * tag_data.1)
                        }
                    } else {
                        if c == ')' {
                            *data = true;
                            if let [Ok(a), Ok(b)] = tag_buf
                                .split('x')
                                .map(|n| n.parse())
                                .collect::<ArrayVec<[_; 2]>>()
                                .as_slice()
                            {
                                *tag_data = (*a, *b);
                            } else {
                                panic!()
                            }
                        } else {
                            tag_buf.push(c);
                        }
                        Some(0)
                    }
                } else {
                    if c == '(' {
                        *tag = true;
                        tag_buf.clear();
                        data_buf.clear();
                        Some(0)
                    } else {
                        Some(1)
                    }
                }
            },
        )
        .sum();
    //
    println!("{}", ans);
}

fn multi_pass_len<T: Iterator<Item = char>>(s: T) -> usize {
    s.scan(
        (false, false, String::new(), Vec::new(), (0, 0)),
        |(tag, data, tag_buf, data_buf, tag_data), c| {
            if *tag {
                if *data {
                    if tag_data.0 > 1 {
                        data_buf.push(c);
                        tag_data.0 -= 1;
                        Some(0)
                    } else {
                        *tag = false;
                        *data = false;
                        data_buf.push(c);
                        //
                        Some(
                            multi_pass_len(data_buf.iter().cloned())
                                * tag_data.1,
                        )
                    }
                } else {
                    if c == ')' {
                        *data = true;
                        if let [Ok(a), Ok(b)] = tag_buf
                            .split('x')
                            .map(|n| n.parse())
                            .collect::<ArrayVec<[_; 2]>>()
                            .as_slice()
                        {
                            *tag_data = (*a, *b);
                        } else {
                            panic!()
                        }
                    } else {
                        tag_buf.push(c);
                    }
                    Some(0)
                }
            } else {
                if c == '(' {
                    *tag = true;
                    tag_buf.clear();
                    data_buf.clear();
                    Some(0)
                } else {
                    Some(1)
                }
            }
        },
    )
    .sum()
}

fn part2() {
    let inp = BufReader::new(File::open("inputfiles/day9/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    //
    let ans = multi_pass_len(inp.chars());
    //
    println!("{}", ans);
}
