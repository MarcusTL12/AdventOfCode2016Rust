use vec_linked_list::VecLinkedList;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    const AMT_ELVES: usize = 3014387;
    //
    let mut v: VecLinkedList<_> = (1..AMT_ELVES + 1).collect();
    //
    let mut curelf = v.head().unwrap();
    //
    while v.len() > 1 {
        v.remove(v.get_next_node(curelf));
        curelf = v.get_next_node(curelf);
    }
    //
    println!("{}", v.remove(curelf));
}

fn part2() {
    const AMT_ELVES: usize = 3014387;
    //
    let mut v: VecLinkedList<_> = (1..AMT_ELVES + 1).collect();
    //
    let mut curelf = v.head().unwrap();
    let mut oppelf = v.len() / 2;
    //
    while v.len() > 1 {
        let n_oppelf = v.offset(oppelf, (v.len() % 2 + 1) as isize);
        v.remove(oppelf);
        oppelf = n_oppelf;
        curelf = v.get_next_node(curelf);
    }
    //
    println!("{}", v.remove(curelf));
}
