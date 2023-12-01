static TEST_INPUT: &str = include_str!("test_input");
static INPUT: &str = include_str!("input");

fn main() {
    let lines = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .map(str::trim);
    println!("{lines:#?}")
}
