use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    divider: u64,
    if_true: usize,
    if_false: usize,
    nb_inspect: u64,
}

impl Monkey {
    fn do_round(monkeys: &mut Vec<Monkey>) {
        for i in 0..monkeys.len() {
            while let Some(old) = monkeys[i].items.pop_front() {
                monkeys[i].nb_inspect += 1;
                let mut new = (monkeys[i].op)(old);
                // new /= 3;
                new %= 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
                let receiver = if (new % monkeys[i].divider) == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[receiver].items.push_back(new);
            }
        }
    }
}

fn main() {
    let monkey_0 = Monkey {
        items: VecDeque::from([75, 63]),
        op: Box::new(|old| old * 3),
        divider: 11,
        if_true: 7,
        if_false: 2,
        nb_inspect: 0,
    };
    let monkey_1 = Monkey {
        items: VecDeque::from([65, 79, 98, 77, 56, 54, 83, 94]),
        op: Box::new(|old| old + 3),
        divider: 2,
        if_true: 2,
        if_false: 0,
        nb_inspect: 0,
    };
    let monkey_2 = Monkey {
        items: VecDeque::from([66]),
        op: Box::new(|old| old + 5),
        divider: 5,
        if_true: 7,
        if_false: 5,
        nb_inspect: 0,
    };
    let monkey_3 = Monkey {
        items: VecDeque::from([51, 89, 90]),
        op: Box::new(|old| old * 19),
        divider: 7,
        if_true: 6,
        if_false: 4,
        nb_inspect: 0,
    };
    let monkey_4 = Monkey {
        items: VecDeque::from([75, 94, 66, 90, 77, 82, 61]),
        op: Box::new(|old| old + 1),
        divider: 17,
        if_true: 6,
        if_false: 1,
        nb_inspect: 0,
    };
    let monkey_5 = Monkey {
        items: VecDeque::from([53, 76, 59, 92, 95]),
        op: Box::new(|old| old + 2),
        divider: 19,
        if_true: 4,
        if_false: 3,
        nb_inspect: 0,
    };
    let monkey_6 = Monkey {
        items: VecDeque::from([81, 61, 75, 89, 70, 92]),
        op: Box::new(|old| old * old),
        divider: 3,
        if_true: 0,
        if_false: 1,
        nb_inspect: 0,
    };
    let monkey_7 = Monkey {
        items: VecDeque::from([81, 86, 62, 87]),
        op: Box::new(|old| old + 8),
        divider: 13,
        if_true: 3,
        if_false: 5,
        nb_inspect: 0,
    };
    let mut monkeys = vec![
        monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7,
    ];

    // for _ in 0..20 {
    for _ in 0..10_000 {
        Monkey::do_round(&mut monkeys);
    }
    println!("{}", {
        let mut nbs_inspect = monkeys.iter().map(|m| m.nb_inspect).collect::<Vec<_>>();
        nbs_inspect.sort();
        nbs_inspect.reverse();
        nbs_inspect[0] * nbs_inspect[1]
    });
}
