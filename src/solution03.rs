use color_eyre::Result;
use aoc2024::read_input;
use regex::Regex;

pub(crate) fn solve() -> Result<()> {
    let input = read_input("input/03.txt")?;
    let input = input.replace(char::is_whitespace, "");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut sum = 0;
    for mat in re.captures_iter(&input) {
        let num_a: usize = mat.get(1).unwrap().as_str().parse()?;
        let num_b: usize = mat.get(2).unwrap().as_str().parse()?;
        sum += num_a * num_b;
    };

    println!("Sum: {sum}");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut sum = 0;
    for mat in re.captures_iter(&input) {
        let start_index = mat.get(0).unwrap().start();
        let substr = &input[..start_index];
        if substr.rfind("do").get_or_insert(2) > substr.rfind("don't").get_or_insert(1) {
            let num_a: usize = mat.get(1).unwrap().as_str().parse()?;
            let num_b: usize = mat.get(2).unwrap().as_str().parse()?;
            sum += num_a * num_b;
        }
    };

    println!("Sum with Do/Don't: {sum}");

    Ok(())
}
