use std::usize;

static TEST_INPUT: &str = include_str!("test_input");
static INPUT: &str = include_str!("input");

fn main() {
    let lines = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|line| {
            let res = line.trim();
            (!res.is_empty()).then_some(res)
        });
    let res = lines.fold(0, |a, line| {
        let thing = line
            .split_once(':')
            .unwrap()
            .1
            .split(';')
            .fold((0, 0, 0), |b, game_str| {
                let res = game_str.split(',').fold((0, 0, 0), |c, throw| {
                    let (amount, color) = throw.trim().split_once(' ').unwrap();
                    let amount = amount.parse::<usize>().unwrap();
                    match color {
                        "red" => (c.0 + amount, c.1, c.2),
                        "green" => (c.0, c.1 + amount, c.2),
                        "blue" => (c.0, c.1, c.2 + amount),
                        _ => panic!("wtf: {color}"),
                    }
                });
                (res.0.max(b.0), res.1.max(b.1), res.2.max(b.2))
            });
        let power = thing.0 * thing.1 * thing.2;
        a + power
    });
    println!("{res}")
}
