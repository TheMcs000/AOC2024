use color_eyre::Result;
use aoc2024::read_input;

pub(crate) fn solve() -> Result<()> {
    let input = read_input("input/01.txt")?;

    let mut sorted_a: Vec<i32> = Vec::new();
    let mut sorted_b: Vec<i32> = Vec::new();
    for line in input.lines() {
        let split: Vec<i32> = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        assert_eq!(split.len(), 2);

        sorted_a.push(split[0]);
        sorted_b.push(split[1]);
    }
    sorted_a.sort();
    sorted_b.sort();

    let mut total_distance = 0;
    for (a, b) in sorted_a.iter().zip(sorted_b.iter()) {
        total_distance += (a - b).abs();
    }

    println!("Total distance: {}", total_distance);

    let mut i_a = 0;
    let mut i_b = 0;
    let mut total_similarity = 0;
    loop {
        if i_a >= sorted_a.len() {
            break;
        }
        let val_a = sorted_a[i_a];
        let mut count_same = 0;

        loop {
            if i_b >= sorted_b.len() {
                break;
            }
            let val_b = sorted_b[i_b];
            if val_a == val_b {
                i_b += 1;
                count_same += 1;
            } else if val_b < val_a {
                i_b += 1;
            } else {
                // val_b > val_a: We are at the next A value
                break;
            }
        };

        total_similarity += val_a * count_same;
        i_a += 1;
    };

    println!("Total similarity: {}", total_similarity);

    return Ok(());
}
