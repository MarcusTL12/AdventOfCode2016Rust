use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum EntityId {
    Bot(u32),
    Output(u32),
}

#[derive(Debug, Clone, Copy)]
enum EntityVal {
    Value(u32),
    Bot(EntityId, bool),
}

#[derive(Debug, Clone, Copy)]
enum EntityMap {
    Bot(EntityVal, Option<EntityVal>),
    Output(EntityVal),
}

fn load_input(filename: &str) -> HashMap<EntityId, EntityMap> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold(HashMap::new(), |mut map, l| {
            match l.split(' ').collect::<ArrayVec<[_; 12]>>().as_slice() {
                ["value", v, _, _, t, n] => match *t {
                    "bot" => {
                        let bot = EntityId::Bot(n.parse().unwrap());
                        let val = EntityVal::Value(v.parse().unwrap());
                        if let Some(EntityMap::Bot(_, x)) = map.get_mut(&bot) {
                            *x = Some(val)
                        } else {
                            let emap = EntityMap::Bot(val, None);
                            map.insert(bot, emap);
                        }
                    }
                    "output" => {
                        let out = EntityId::Output(n.parse().unwrap());
                        let emap = EntityMap::Output(EntityVal::Value(
                            v.parse().unwrap(),
                        ));
                        map.insert(out, emap);
                    }
                    _ => panic!(),
                },
                ["bot", n, _, _, _, t1, n1, _, _, _, t2, n2] => {
                    let inbot = EntityId::Bot(n.parse().unwrap());
                    //
                    for (&t, n, hl) in [(t1, n1, false), (t2, n2, true)].iter()
                    {
                        match t {
                            "bot" => {
                                let out = EntityId::Bot(n.parse().unwrap());
                                let val = EntityVal::Bot(inbot, *hl);
                                if let Some(EntityMap::Bot(_, x)) =
                                    map.get_mut(&out)
                                {
                                    *x = Some(val);
                                } else {
                                    let emap = EntityMap::Bot(val, None);
                                    map.insert(out, emap);
                                }
                            }
                            "output" => {
                                let out = EntityId::Output(n.parse().unwrap());
                                let val = EntityVal::Bot(inbot, *hl);
                                let emap = EntityMap::Output(val);
                                map.insert(out, emap);
                            }
                            _ => panic!(),
                        }
                    }
                }
                _ => panic!(),
            }
            map
        })
}

fn get_val(
    map: &HashMap<EntityId, EntityMap>,
    memo: &mut HashMap<EntityId, ArrayVec<[u32; 2]>>,
    id: EntityId,
) -> ArrayVec<[u32; 2]> {
    fn valmatch(
        map: &HashMap<EntityId, EntityMap>,
        memo: &mut HashMap<EntityId, ArrayVec<[u32; 2]>>,
        v: EntityVal,
    ) -> u32 {
        match v {
            EntityVal::Value(v) => v,
            EntityVal::Bot(id, hl) => {
                if let [a, b] = get_val(map, memo, id).as_slice() {
                    let &v = if hl { max(a, b) } else { min(a, b) };
                    v
                } else {
                    panic!()
                }
            }
        }
    }
    //
    if let Some(x) = memo.get(&id) {
        x.clone()
    } else {
        let ans = match map[&id] {
            EntityMap::Output(v) => {
                iter::once(valmatch(map, memo, v)).collect()
            }
            EntityMap::Bot(v1, Some(v2)) => {
                let v1 = valmatch(map, memo, v1);
                let v2 = valmatch(map, memo, v2);
                ArrayVec::from([v1, v2])
            }
            _ => panic!(),
        };
        //
        memo.insert(id, ans.clone());
        //
        ans
    }
}

fn part1() {
    const CHECK: (u32, u32) = (17, 61);
    //
    let inp = load_input("inputfiles/day10/input.txt");
    let mut memo = HashMap::new();
    //
    let ans = inp
        .keys()
        .filter(|x| matches!(x, EntityId::Bot(_)))
        .filter(|&&id| {
            let v = get_val(&inp, &mut memo, id);
            v.as_slice() == &[CHECK.0, CHECK.1]
                || v.as_slice() == &[CHECK.1, CHECK.0]
        })
        .next()
        .unwrap();
    //
    println!("{:?}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day10/input.txt");
    let mut memo = HashMap::new();
    //
    let ans = [0, 1, 2]
        .iter()
        .map(|&id| EntityId::Output(id))
        .map(|id| get_val(&inp, &mut memo, id)[0])
        .fold(1, |a, b| a * b);
    //
    println!("{}", ans);
}
