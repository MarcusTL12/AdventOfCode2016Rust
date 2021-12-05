use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn render_screen(screen: &[Vec<bool>]) {
    for row in screen {
        for &cell in row {
            print!("{}", if cell { '█' } else { ' ' });
        }
        println!();
    }
}

fn get_screen() -> Vec<Vec<bool>> {
    const ROWS: usize = 6;
    const COLS: usize = 50;
    //
    let mut screen = vec![vec![false; COLS]; ROWS];
    //
    for l in BufReader::new(File::open("inputfiles/day8/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        match l.split(' ').collect::<ArrayVec<_, 5>>().as_slice() {
            ["rect", x] => {
                if let [Ok(a), Ok(b)] = x
                    .split('x')
                    .map(|s| s.parse())
                    .collect::<ArrayVec<_, 2>>()
                    .as_slice()
                {
                    for i in 0..*b {
                        for j in 0..*a {
                            screen[i][j] = true;
                        }
                    }
                }
            }
            ["rotate", dir, a, _, b] => match *dir {
                "column" => {
                    let colnr: usize =
                        a.split('=').last().unwrap().parse().unwrap();
                    let amt = b.parse().unwrap();
                    //
                    for (i, v) in (0..ROWS)
                        .cycle()
                        .skip(amt)
                        .zip(0..ROWS)
                        .map(|(i, j)| (i, screen[j][colnr]))
                        .collect::<ArrayVec<_, ROWS>>()
                    {
                        screen[i][colnr] = v;
                    }
                }
                "row" => {
                    let rownr: usize =
                        a.split('=').last().unwrap().parse().unwrap();
                    let amt = b.parse().unwrap();
                    //
                    for (i, v) in (0..COLS)
                        .cycle()
                        .skip(amt)
                        .zip(0..COLS)
                        .map(|(i, j)| (i, screen[rownr][j]))
                        .collect::<ArrayVec<_, COLS>>()
                    {
                        screen[rownr][i] = v;
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
    screen
}

fn part1() {
    let ans = get_screen()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    render_screen(&get_screen());
}
