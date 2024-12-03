use color_eyre::Result;
use aoc2024::read_input;

pub(crate) fn solve() -> Result<()> {
    let input = read_input("input/02.txt")?;

    let mut total_safe = 0;
    for line in input.lines() {
        let split: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_save(split) {
            total_safe += 1;
        }
    }

    println!("Total safe: {}", total_safe);

    let mut total_safe = 0;
    for line in input.lines() {
        let split: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        for ignored_i in 0..split.len() {
            let extracted = split.iter().enumerate().filter(|(i, _)| *i != ignored_i).map(|(_, x)| *x).collect();
            if is_save(extracted) {
                total_safe += 1;
                break
            }
        }
    }

    println!("Total safe with dampener: {}", total_safe);

    Ok(())
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing
}

fn is_save(split: Vec<i32>) -> bool {
    let mut last_val = split[0];
    let mut global_direction: Option<Direction> = None;
    for i in 1..split.len() {
        let val = split[i];

        let dif = (val - last_val).abs();
        if dif > 3 || dif == 0 {
            return false;
        }

        let this_dir = if val > last_val { Direction::Increasing } else { Direction::Decreasing };

        match &global_direction {
            None => {
                global_direction = Some(this_dir);
            }
            Some(direction) => {
                if *direction != this_dir {
                    return false;
                }
            }
        };
        last_val = val;
    }
    true
}
