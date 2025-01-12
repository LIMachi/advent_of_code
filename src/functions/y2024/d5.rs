use std::collections::HashSet;

fn parse(input: &str) -> Result<(HashSet<(u32, u32)>, Vec<Vec<u32>>), String> {
    use nom::Parser;
    use nom::character::complete::u32 as int;
    use nom::bytes::complete::tag;
    use nom::sequence::{terminated, separated_pair};
    use nom::multi::{many0, separated_list0, separated_list1};

    let l = many0(terminated(separated_pair(int::<&str, ()>, tag("|"), int), tag("\n"))).map(|r| {
        let mut out = HashSet::new();
        for (k, v) in r {
            out.insert((k, v));
        }
        out
    });
    let p = separated_list0(tag("\n"), separated_list1(tag(","), int));
    separated_pair(l, tag("\n"), p)(input).map_err(|e| e.to_string()).map(|(_, r)| r)
}

fn middle(sequence: &Vec<u32>) -> u32 {
    sequence[sequence.len() / 2]
}

fn check_valid_order(sequence: &Vec<u32>, precedence: &HashSet<(u32, u32)>) -> bool {
    for left in 0..(sequence.len() - 1) {
        for right in left + 1..sequence.len() {
            if !precedence.contains(&(sequence[left], sequence[right])) {
                return false;
            }
        }
    }
    true
}

fn reorder(sequence: &Vec<u32>, precedence: &HashSet<(u32, u32)>) -> Vec<u32> {
    let mut out = sequence.clone();
    let mut left = 0;
    'main: loop {
        if left >= out.len() {
            break;
        }
        for right in left + 1..out.len() {
            if !precedence.contains(&(out[left], out[right])) {
                let t = out[left];
                out[left] = out[right];
                out[right] = t;
                continue 'main;
            }
        }
        left += 1;
    }
    out
}

pub fn y2024d5a(input: &str) -> Result<String, String> {
    let (precedence, pages) = parse(input)?;
    let mut acc = 0;
    for sequence in &pages {
        if check_valid_order(sequence, &precedence) {
            acc += middle(sequence);
        }
    }
    Ok(acc.to_string())
}

pub fn y2024d5b(input: &str) -> Result<String, String> {
    let (precedence, pages) = parse(input)?;
    let mut acc = 0;
    for sequence in &pages {
        if !check_valid_order(sequence, &precedence) {
            acc += middle(&reorder(sequence, &precedence));
        }
    }
    Ok(acc.to_string())
}
