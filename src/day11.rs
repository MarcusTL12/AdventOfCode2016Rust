use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    // iter,
};

use regex::Regex;

use itertools::Itertools;

// use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Entity {
    Gen(usize),
    Chip(usize),
}

fn load_input(filename: &str) -> (Vec<usize>, Vec<usize>) {
    let reg = Regex::new(r"a (\w+)(?:-compatible)? (\w+)").unwrap();
    //
    let floors: Vec<Vec<_>> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            reg.captures_iter(&l)
                .map(|c| {
                    (
                        c[1].to_owned(),
                        match &c[2] {
                            "generator" => false,
                            "microchip" => true,
                            _ => panic!(),
                        },
                    )
                })
                .collect()
        })
        .collect();
    //
    let compounds = floors
        .iter()
        .flat_map(|floor| floor.iter())
        .map(|(x, _)| x)
        .collect::<HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();
    //
    let floors: HashMap<_, _> = floors
        .iter()
        .enumerate()
        .flat_map(|(i, floor)| {
            floor
                .into_iter()
                .map(move |(a, b)| (i, a, b))
                .map(|(i, c, m)| {
                    (
                        match m {
                            false => Entity::Gen(compounds[c]),
                            true => Entity::Chip(compounds[c]),
                        },
                        i,
                    )
                })
        })
        .collect();
    //
    let (gens, chips) = floors.iter().fold(
        (vec![0; floors.len() / 2], vec![0; floors.len() / 2]),
        |(mut gens, mut chips), (k, v)| {
            match k {
                Entity::Gen(id) => gens[*id] = *v,
                Entity::Chip(id) => chips[*id] = *v,
            }
            (gens, chips)
        },
    );
    //
    (gens, chips)
}

fn valid_conf((_, gens, chips): &(usize, Vec<usize>, Vec<usize>)) -> bool {
    let mut gens_floors = [false; 4];
    for &i in gens.iter() {
        gens_floors[i] = true;
    }
    //
    let mut chips_floors = [false; 4];
    for (i, &j) in chips.iter().enumerate() {
        if gens[i] != j {
            chips_floors[j] = true;
        }
    }
    //
    !gens_floors
        .iter()
        .zip(chips_floors.iter())
        .any(|(&a, &b)| a && b)
}

fn all_at_top((_, gens, chips): &(usize, Vec<usize>, Vec<usize>)) -> bool {
    gens.iter().all(|&x| x == 3) && chips.iter().all(|&x| x == 3)
}

fn shortest_path(conf: (usize, Vec<usize>, Vec<usize>)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((conf.clone(), 0));
    //
    let mut visited = HashSet::new();
    visited.insert(conf);
    //
    while let Some((conf, len)) = queue.pop_front() {
        let floor_items: Vec<_> = conf
            .1
            .iter()
            .enumerate()
            .filter(|(_, &i)| i == conf.0)
            .map(|(i, _)| Entity::Gen(i))
            .chain(
                conf.2
                    .iter()
                    .enumerate()
                    .filter(|(_, &i)| i == conf.0)
                    .map(|(i, _)| Entity::Chip(i)),
            )
            .collect();
        //
        let confs = (0..4)
            .filter(|&i| (i as isize - conf.0 as isize).abs() == 1)
            .flat_map(|i| {
                let singly =
                    floor_items.iter().map(move |x| (i, x)).map(|(i, e)| {
                        let mut nconf = conf.clone();
                        nconf.0 = i;
                        match e {
                            Entity::Gen(j) => nconf.1[*j] = i,
                            Entity::Chip(j) => nconf.2[*j] = i,
                        }
                        nconf
                    });
                //
                let doubly = floor_items
                    .iter()
                    .tuple_combinations()
                    .map(move |x| (i, x))
                    .map(|(i, (e1, e2))| {
                        let mut nconf = conf.clone();
                        nconf.0 = i;
                        match e1 {
                            Entity::Gen(j) => nconf.1[*j] = i,
                            Entity::Chip(j) => nconf.2[*j] = i,
                        }
                        match e2 {
                            Entity::Gen(j) => nconf.1[*j] = i,
                            Entity::Chip(j) => nconf.2[*j] = i,
                        }
                        nconf
                    });
                //
                singly.chain(doubly)
            });
        //
        for nconf in confs {
            if valid_conf(&nconf) && !visited.contains(&nconf) {
                if all_at_top(&nconf) {
                    return len + 1;
                }
                //
                queue.push_back((nconf.clone(), len + 1));
                visited.insert(nconf);
            }
        }
    }
    //
    panic!("Did not find a solution!");
}

fn part1() {
    let inp = load_input("inputfiles/day11/input.txt");
    //
    let ans = shortest_path((0, inp.0, inp.1));
    //
    println!("{}", ans);
}

fn part2() {
    let mut inp = load_input("inputfiles/day11/input.txt");
    //
    inp.0.push(0);
    inp.1.push(0);
    inp.0.push(0);
    inp.1.push(0);
    //
    let ans = shortest_path((0, inp.0, inp.1));
    //
    println!("{}", ans);
}
