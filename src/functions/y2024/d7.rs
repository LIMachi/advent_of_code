fn parser(input: &str) -> Result<Vec<(u32, Vec<u32>)>, String> {
    use nom::Parser;
    use nom::character::complete::u32 as int;
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;
    use nom::multi::separated_list1;

    separated_list1(
        tag("\n"),
        separated_pair(
            int,
            tag(": "),
            separated_list1(
                tag(" "),
                int::<&str, ()>)
        )
    )(input).map_err(|e| e.to_string()).map(|(_, v)| v)
}

enum Operators {
    ADD,
    MUL
}

impl Operators {

}

fn rec_a(res: u32, acc: u32, pos: usize, op: Operators, input: &Vec<u32>) -> Option<u32> {

}

fn a(res: u32, input: &Vec<u32>) -> Option<u32> {
    rec_a(res, 0, 0, Operators::ADD, input)
        .or_else(|| rec_a(res, 0, 0, Operators::MUL, input))
}

pub fn y2024d7a(input: &str) -> Result<String, String> {
    let v = parser(input)?;
    let mut acc = 0;
    for (e, v) in v {
        if let Some(a) = a(e, &v) {
            acc += a;
        }
    }
    Ok(acc.to_string())
}

pub fn y2024d7b(_input: &str) -> Result<String, String> {
    Ok("".to_string())
}
