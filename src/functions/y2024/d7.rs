fn parser(input: &str) -> Result<Vec<(u64, Vec<u64>)>, String> {
    use nom::Parser;
    use nom::character::complete::u64 as int;
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

#[derive(Copy, Clone, Debug)]
enum Operators {
    ADD,
    MUL,
    CAT
}

impl Operators {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operators::ADD => left + right,
            Operators::MUL => left * right,
            Operators::CAT => if left > 0 && right > 0 {
                (left.to_string() + &right.to_string()).parse::<u64>().unwrap_or(u64::MAX)
            } else {
                left + right
            }
        }
    }

    fn a_array() -> [Operators; 2] {
        [Operators::ADD, Operators::MUL]
    }

    fn b_array() -> [Operators; 3] {
        [Operators::ADD, Operators::MUL, Operators::CAT]
    }
}

fn rec(res: u64, mut acc: u64, mut pos: usize, op: Operators, input: &Vec<u64>, operators: &[Operators]) -> bool {
    if pos >= input.len() {
        return false;
    }
    acc = op.apply(acc, input[pos]);
    if acc == res && pos + 1 == input.len() {
        return true;
    }
    if acc > res {
        return false;
    }
    pos += 1;
    for &op in operators {
        if rec(res, acc, pos, op, input, operators) {
            return true;
        }
    }
    false
}

fn calc(res: u64, input: &Vec<u64>, operators: &[Operators]) -> bool {
    rec(res, 0, 0, Operators::ADD, input, operators)
}

pub fn y2024d7a(input: &str) -> Result<String, String> {
    let v = parser(input)?;
    let mut acc = 0;
    for (e, v) in v {
        if calc(e, &v, &Operators::a_array()) {
            acc += e;
        }
    }
    Ok(acc.to_string())
}

pub fn y2024d7b(input: &str) -> Result<String, String> {
    let v = parser(input)?;
    let mut acc = 0;
    for (e, v) in v {
        if calc(e, &v, &Operators::b_array()) {
            acc += e;
        }
    }
    Ok(acc.to_string())
}
