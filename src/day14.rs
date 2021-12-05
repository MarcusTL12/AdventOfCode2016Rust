use std::{collections::VecDeque, io::Write};

use md5;

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

const SALT: &[u8] = b"yjdafjpo";

fn ninarow(data: &[u8], n: usize) -> Option<u8> {
    let mut record = 0;
    let mut curlen = 0;
    let mut curchar = 0;
    //
    for &c in data {
        if c == curchar {
            curlen += 1;
        } else {
            if curlen > record {
                record = curlen;
            }
            if record >= n {
                return Some(curchar);
            }
            curlen = 1;
            curchar = c;
        }
    }
    //
    if curlen >= n {
        Some(curchar)
    } else {
        None
    }
}

fn ninarow2(data: &[u8], n: usize, spec: u8) -> bool {
    data.iter()
        .fold((0, 0), |(mut record, mut curlen), &c| {
            if c == spec {
                curlen += 1;
            } else {
                if curlen > record {
                    record = curlen;
                }
                curlen = 0;
            }
            //
            (record, curlen)
        })
        .0
        >= n
}

fn part1() {
    let mut buf1 = Vec::new();
    let mut buf2 = ArrayVec::<_, 32>::new();
    //
    let mut hashes = VecDeque::with_capacity(1001);
    //
    for i in 0..1001 {
        buf1.clear();
        buf1.extend(SALT.iter());
        write!(buf1, "{}", i).unwrap();
        //
        buf2.clear();
        write!(buf2, "{:x}", md5::compute(&buf1)).unwrap();
        //
        hashes.push_back((i, buf2.clone()));
    }
    //
    let mut keys = Vec::with_capacity(64);
    //
    while keys.len() < 64 {
        if let Some((i, curhash)) = hashes.pop_front() {
            if let Some(c) = ninarow(&curhash, 3) {
                for (_, h) in hashes.iter() {
                    if ninarow2(h, 5, c) {
                        keys.push(i);
                    }
                }
            }
            //
            let i = hashes[hashes.len() - 1].0 + 1;
            buf1.clear();
            buf1.extend(SALT.iter());
            write!(buf1, "{}", i).unwrap();
            //
            buf2.clear();
            write!(buf2, "{:x}", md5::compute(&buf1)).unwrap();
            //
            hashes.push_back((i, buf2.clone()));
        }
    }
    //
    println!("{}", keys[keys.len() - 1]);
}

fn hash_stretch(
    data: usize,
    buf: &mut Vec<u8>,
    target: &mut ArrayVec<u8, 32>,
) {
    buf.clear();
    buf.extend(SALT.iter());
    write!(buf, "{}", data).unwrap();
    for _ in 0..2017 {
        target.clear();
        write!(target, "{:x}", md5::compute(&buf)).unwrap();
        buf.clear();
        buf.extend(target.iter());
    }
}

fn part2() {
    let mut buf1 = Vec::new();
    let mut buf2 = ArrayVec::<_, 32>::new();
    //
    let mut hashes = VecDeque::with_capacity(1001);
    //
    for i in 0..1001 {
        hash_stretch(i, &mut buf1, &mut buf2);
        //
        hashes.push_back((i, buf2.clone()));
    }
    //
    let mut keys = Vec::with_capacity(64);
    //
    while keys.len() < 64 {
        if let Some((i, curhash)) = hashes.pop_front() {
            if let Some(c) = ninarow(&curhash, 3) {
                for (_, h) in hashes.iter() {
                    if ninarow2(h, 5, c) {
                        keys.push(i);
                    }
                }
            }
            //
            let i = hashes[hashes.len() - 1].0 + 1;
            hash_stretch(i, &mut buf1, &mut buf2);
            //
            hashes.push_back((i, buf2.clone()));
        }
    }
    //
    println!("{}", keys[keys.len() - 1]);
}
