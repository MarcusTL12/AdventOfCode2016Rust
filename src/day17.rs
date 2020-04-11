use std::{collections::VecDeque, io::Write, iter};

use arrayvec::ArrayVec;

use md5;

use num::Complex;

pub const PARTS: [fn(); 2] = [part1, part2];

const DIRS: [Complex<i32>; 4] = [
    Complex { re: 0, im: -1 },
    Complex { re: 0, im: 1 },
    Complex { re: -1, im: 0 },
    Complex { re: 1, im: 0 },
];

const DIRCODES: [u8; 4] = [b'U', b'D', b'L', b'R'];

fn doors<T: AsRef<[u8]>>(data: T) -> impl Iterator<Item = bool> {
    let mut a = ArrayVec::<[_; 4]>::new();
    //
    write!(&mut a, "{:x}", md5::compute(data)).unwrap_err();
    //
    a.into_iter().map(|x| matches!(x, b'b'..=b'f'))
}

fn walls(p: Complex<i32>) -> [bool; 4] {
    [p.im > 0, p.im < 3, p.re > 0, p.re < 3]
}

fn open(p: Complex<i32>, data: &[u8]) -> [bool; 4] {
    walls(p)
        .iter()
        .zip(doors(data))
        .map(|(&a, b)| a && b)
        .collect::<ArrayVec<[_; 4]>>()
        .into_inner()
        .unwrap()
}

fn part1() {
    const PASSCODE: &[u8] = b"bwnlcvfs";
    const TARGET: Complex<i32> = Complex { re: 3, im: 3 };
    //
    let mut queue = VecDeque::new();
    queue.push_back((
        Complex { re: 0, im: 0 },
        PASSCODE.iter().cloned().collect::<Vec<_>>(),
    ));
    //
    let mut path = None;
    //
    'outer: while let Some((p, data)) = queue.pop_front() {
        for (d, c) in DIRS
            .iter()
            .zip(DIRCODES.iter())
            .zip(open(p, &data).iter())
            .filter_map(|(d, &x)| if x { Some(d) } else { None })
        {
            if p + d == TARGET {
                path = Some(
                    data.iter()
                        .skip(PASSCODE.len())
                        .chain(iter::once(c))
                        .map(|&x| x as char)
                        .collect::<String>(),
                );
                break 'outer;
            }
            queue.push_back((
                p + d,
                data.iter().chain(iter::once(c)).cloned().collect(),
            ))
        }
    }
    //
    if let Some(path) = path {
        println!("{}", path);
    } else {
        panic!("No path found!");
    }
}

fn part2() {
    const PASSCODE: &[u8] = b"bwnlcvfs";
    const TARGET: Complex<i32> = Complex { re: 3, im: 3 };
    //
    fn rec(pos: Complex<i32>, stack: &mut Vec<u8>) -> Option<usize> {
        if pos == TARGET {
            Some(stack.len() - PASSCODE.len())
        } else {
            DIRS.iter()
                .zip(DIRCODES.iter())
                .zip(open(pos, stack).iter())
                .filter_map(|(d, &x)| if x { Some(d) } else { None })
                .filter_map(|(d, &ndir)| {
                    stack.push(ndir);
                    let ans = rec(pos + d, stack);
                    stack.pop();
                    ans
                })
                .max()
        }
    }
    //
    let ans = rec(Complex { re: 0, im: 0 }, &mut Vec::from(PASSCODE))
        .expect("Did not find any path!");
    //
    println!("{}", ans);
}
