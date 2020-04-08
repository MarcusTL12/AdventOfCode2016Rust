use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use regex::Regex;

// use itertools::Itertools;

use arrayvec::ArrayVec;

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
    println!("{:?}", gens_floors);
    println!("{:?}", chips_floors);
    //
    !gens_floors
        .iter()
        .zip(chips_floors.iter())
        .any(|(&a, &b)| a && b)
}

fn shortest_path(
    conf: &(usize, Vec<usize>, Vec<usize>),
    memo: &mut HashMap<(usize, Vec<usize>, Vec<usize>), usize>,
    in_progress: &mut HashSet<(usize, Vec<usize>, Vec<usize>)>,
) -> usize {
    if let Some(x) = memo.get(conf) {
        *x
    } else {
        let floor_items: Vec<_> = (0..conf.1.len())
            .flat_map(|i| {
                iter::once((Entity::Gen(i), conf.1[i]))
                    .chain(iter::once((Entity::Chip(i), conf.2[i])))
            })
            .filter(|(_, floor)| *floor == conf.0)
            .map(|(x, _)| x)
            .collect();
        //
        println!("{:?}", floor_items);
        //
        let mut nconf = conf.clone();
        //
        let ans = (0..floor_items.len())
            .map(|i| iter::once(i).collect::<ArrayVec<[_; 2]>>())
            .chain((0..floor_items.len()).flat_map(|i| {
                (i + 1..floor_items.len())
                    .map(move |j| iter::once(i).chain(iter::once(j)).collect())
            }))
            .flat_map(|c| {
                (0..4).filter(|&i| i != conf.0).map(move |i| (i, c.clone()))
            })
            .filter_map(|(floor, comb)| {
                nconf.0 = floor;
                for (a, b) in conf.1.iter().zip(nconf.1.iter_mut()) {
                    *b = *a;
                }
                for (a, b) in conf.2.iter().zip(nconf.2.iter_mut()) {
                    *b = *a;
                }
                //
                for c in comb {
                    match floor_items[c] {
                        Entity::Gen(i) => nconf.1[i] = floor,
                        Entity::Chip(i) => nconf.2[i] = floor,
                    }
                }
                //
                if valid_conf(&nconf) && !in_progress.contains(&nconf) {
                    in_progress.insert(nconf.clone());
                    //
                    let ans = shortest_path(&nconf, memo, in_progress);
                    //
                    in_progress.remove(&nconf);
                    Some(ans)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
            + 1;
        //
        memo.insert(conf.clone(), ans);
        //
        ans
    }
}

fn part1() {
    let inp = load_input("inputfiles/day11/example1.txt");
    //
    let conf = (0, inp.0, inp.1);
    //
    let temp = shortest_path(&conf, &mut HashMap::new(), &mut HashSet::new());
    //
    println!("{}", temp);
}

fn part2() {}
