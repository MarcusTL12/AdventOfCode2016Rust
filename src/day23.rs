use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone, Copy)]
enum Arg {
    Value(i32),
    Register(usize),
}

impl FromStr for Arg {
    type Err = &'static str;
    //
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse() {
            Ok(Arg::Value(n))
        } else {
            match s {
                "a" => Ok(0),
                "b" => Ok(1),
                "c" => Ok(2),
                "d" => Ok(3),
                _ => Err("Invalid register!"),
            }
            .and_then(|x| Ok(Arg::Register(x)))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg),
}

fn load_input(filename: &str) -> Vec<Instruction> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(
            |l| match l.split(' ').collect::<ArrayVec<_, 3>>().as_slice() {
                ["cpy", x, y] => {
                    Instruction::Cpy(x.parse().unwrap(), y.parse().unwrap())
                }
                ["inc", x] => Instruction::Inc(x.parse().unwrap()),
                ["dec", x] => Instruction::Dec(x.parse().unwrap()),
                ["jnz", x, y] => {
                    Instruction::Jnz(x.parse().unwrap(), y.parse().unwrap())
                }
                ["tgl", x] => Instruction::Tgl(x.parse().unwrap()),
                a => unreachable!("{:?}", a),
            },
        )
        .collect()
}

fn run_program(registers: &mut [i32], program: &mut [Instruction]) {
    let mut i = 0;
    while i < program.len() {
        match program[i] {
            Instruction::Cpy(Arg::Value(x), Arg::Register(y)) => {
                registers[y] = x
            }
            Instruction::Cpy(Arg::Register(x), Arg::Register(y)) => {
                registers[y] = registers[x]
            }
            Instruction::Inc(Arg::Register(x)) => registers[x] += 1,
            Instruction::Dec(Arg::Register(x)) => registers[x] -= 1,
            Instruction::Jnz(x, y) => {
                let x = match x {
                    Arg::Value(x) => x,
                    Arg::Register(x) => registers[x],
                };
                //
                let y = match y {
                    Arg::Value(x) => x,
                    Arg::Register(x) => registers[x],
                };
                //
                if x != 0 {
                    i = (i as i32 + y) as usize - 1;
                }
            }
            Instruction::Tgl(arg) => {
                let x = match arg {
                    Arg::Value(x) => x,
                    Arg::Register(x) => registers[x],
                } + i as i32;
                if x >= 0 && x < program.len() as i32 {
                    let nins = match program[x as usize] {
                        Instruction::Inc(x) => Instruction::Dec(x),
                        Instruction::Dec(x) | Instruction::Tgl(x) => {
                            Instruction::Inc(x)
                        }
                        Instruction::Jnz(x, y) => Instruction::Cpy(x, y),
                        Instruction::Cpy(x, y) => Instruction::Jnz(x, y),
                    };
                    program[x as usize] = nins;
                }
            }
            a => unreachable!("{:?}", a),
        }
        i += 1;
    }
}

fn part1() {
    let mut program = load_input("inputfiles/day23/input.txt");
    //
    let mut registers = vec![0; 4];
    //
    registers[0] = 7;
    //
    run_program(&mut registers, &mut program);
    //
    println!("{}", registers[0]);
}

fn part2() {
    let mut program = load_input("inputfiles/day23/input.txt");
    //
    let mut registers = vec![0; 4];
    //
    registers[0] = 12;
    //
    run_program(&mut registers, &mut program);
    //
    println!("{}", registers[0]);
}
