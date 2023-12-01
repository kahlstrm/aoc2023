static TEST_INPUT: &str = include_str!("test_input");
static INPUT: &str = include_str!("input");
static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn main() {
    let res: usize = INPUT
        .lines()
        .map(|line| {
            let line = line.trim();
            let first = (0..=line.len()).find_map(|i| {
                let sliec = &line[..i];
                if sliec.ends_with(char::is_numeric) {
                    return sliec.chars().last();
                }
                NUMBERS.iter().enumerate().find_map(|(num_i, num_str)| {
                    sliec
                        .find(num_str)
                        .and_then(|_| char::from_digit(num_i as u32 + 1, 10))
                })
            });
            let second = (0..=line.len()).find_map(|i| {
                if let Some(next) = &line[line.len() - i..].chars().next() {
                    if next.is_numeric() {
                        return Some(next.to_owned());
                    }
                }

                NUMBERS.iter().enumerate().find_map(|(num_i, num_str)| {
                    line[line.len() - i..]
                        .find(num_str)
                        .and_then(|_| char::from_digit(num_i as u32 + 1, 10))
                })
            });
            match (first, second) {
                (Some(f), Some(s)) => {
                    let to_parse = format!("{}{}", f.to_string(), s.to_string());
                    println!("{}", to_parse);
                    to_parse.parse::<usize>().unwrap()
                }
                a => panic!("wtf:{a:#?}"),
            }
        })
        .sum();
    println!("{res}")
}
