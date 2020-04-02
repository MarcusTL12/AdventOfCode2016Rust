use std::{io::Write, iter};

use md5;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: &[u8] = b"ffykfhsq";
// const INPUT: &[u8] = b"abc";

fn get_char(
    num: usize,
    buffer1: &mut Vec<u8>,
    buffer2: &mut Vec<u8>,
    amt_zeros: usize,
) -> Option<char> {
    buffer1.clear();
    buffer1.extend(INPUT.iter());
    write!(buffer1, "{}", num).unwrap();
    buffer2.clear();
    write!(buffer2, "{:x}", md5::compute(buffer1)).unwrap();
    //
    if buffer2
        .iter()
        .map(|&x| x as char)
        .take(amt_zeros)
        .eq(iter::repeat('0').take(amt_zeros))
    {
        Some(buffer2[amt_zeros] as char)
    } else {
        None
    }
}

fn get_charpos(
    num: usize,
    buffer1: &mut Vec<u8>,
    buffer2: &mut Vec<u8>,
    amt_zeros: usize,
) -> Option<(char, usize)> {
    buffer1.clear();
    buffer1.extend(INPUT.iter());
    write!(buffer1, "{}", num).unwrap();
    buffer2.clear();
    write!(buffer2, "{:x}", md5::compute(buffer1)).unwrap();
    //
    if buffer2
        .iter()
        .map(|&x| x as char)
        .take(amt_zeros)
        .eq(iter::repeat('0').take(amt_zeros))
    {
        if let Some(n) = (buffer2[amt_zeros] as char).to_digit(10) {
            Some((buffer2[amt_zeros + 1] as char, n as usize))
        } else {
            None
        }
    } else {
        None
    }
}

fn part1() {
    let ans: String = (0..)
        .scan((Vec::new(), Vec::new()), |(buf1, buf2), i| {
            Some(get_char(i, buf1, buf2, 5))
        })
        .filter_map(|x| x)
        .take(8)
        .collect();
    //
    println!("{}", ans);
}

fn part2() {
    let ans: String = (0..)
        .scan((Vec::new(), Vec::new()), |(buf1, buf2), i| {
            Some(get_charpos(i, buf1, buf2, 5))
        })
        .filter_map(|x| x)
        .scan(([None; 8], 0), |(pass, j), (c, i)| {
            if *j < 8 {
                if i < 8 && pass[i].is_none() {
                    pass[i] = Some(c);
                    *j += 1;
                }
                Some(*pass)
            } else {
                None
            }
        })
        .last()
        .unwrap()
        .iter()
        .filter_map(|&x| x)
        .collect();
    //
    println!("{}", ans);
}
