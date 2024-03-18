fn parse_nums(s: &str) -> Vec<u32> {
    s.trim().split_whitespace().map(|v| v.parse().unwrap()).collect()
}

pub fn count_winners(line: &str) -> Option<u32> {
    let mut split1 = line.split(":");
    let _card_label = split1.next()?;

    let card_sides = split1.next()?;
    let mut split2 = card_sides.split("|");
    let winning_nums: Vec<u32> = parse_nums(split2.next()?);
    let my_nums: Vec<u32> = parse_nums(split2.next()?);

    let mut counter: u32 = 0;
    for my_num in &my_nums {
        if winning_nums.iter().any(|v| *v == *my_num) {
            counter += 1;
        }
    }

    Some(counter)
}