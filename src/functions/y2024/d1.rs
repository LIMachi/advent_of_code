fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.split("\n")
        .fold((Vec::new(), Vec::new()), |(mut lv, mut rv), line| {
            let mut line = line.split_ascii_whitespace();
            match (line.next().and_then(|l| l.parse::<u32>().ok()), line.next().and_then(|r| r.parse::<u32>().ok())) {
                (Some(l), Some(r)) => {
                    lv.push(l);
                    rv.push(r);
                }
                _ => {}
            }
            (lv, rv)
        })
}

pub fn y2024d1a(input: &str) -> Result<String, String> {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    let res = left.iter().zip(right.iter()).fold(0u32, |a, (l, r)| {
        a + l.abs_diff(*r)
    });
    Ok(res.to_string())
}

pub fn y2024d1b(input: &str) -> Result<String, String> {
    let (left, right) = parse(input);
    let res = left.iter().fold(0, |total, l| {
        total + *l * right.iter().fold(0, |i, c| if *c == *l { i + 1 } else { i })
    });
    Ok(res.to_string())
}
