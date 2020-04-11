use std::iter;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: &[u8] = b"01111010110010011";

fn dragon<'a, T: DoubleEndedIterator<Item = bool> + Clone + 'a>(
    it: T,
) -> impl Iterator<Item = bool> {
    it.clone()
        .chain(iter::once(false))
        .chain(it.map(|x| !x).rev())
}

fn part1() {
    const DISK: usize = 272;
    //
    let mut a = INPUT.iter().map(|&x| x != b'0').collect::<Vec<_>>();
    let mut b = Vec::new();
    //
    while a.len() < DISK {
        b.clear();
        b.extend(dragon(a.iter().cloned()));
        //
        let temp = a;
        a = b;
        b = temp;
    }
    //
    a.truncate(DISK);
    //
    while a.len() % 2 == 0 {
        b.clear();
        b.extend(a.iter().tuples().map(|(x, y)| x == y));
        //
        let temp = a;
        a = b;
        b = temp;
    }
    //
    println!(
        "{}",
        a.iter()
            .map(|&x| if x { '1' } else { '0' })
            .collect::<String>()
    );
}

fn part2() {
    const DISK: usize = 35651584;
    //
    let mut a = INPUT.iter().map(|&x| x != b'0').collect::<Vec<_>>();
    let mut b = Vec::new();
    //
    while a.len() < DISK {
        b.clear();
        b.extend(dragon(a.iter().cloned()));
        //
        let temp = a;
        a = b;
        b = temp;
    }
    //
    a.truncate(DISK);
    //
    while a.len() % 2 == 0 {
        b.clear();
        b.extend(a.iter().tuples().map(|(x, y)| x == y));
        //
        let temp = a;
        a = b;
        b = temp;
    }
    //
    println!(
        "{}",
        a.iter()
            .map(|&x| if x { '1' } else { '0' })
            .collect::<String>()
    );
}
