#[cfg(test)]
mod tests {
    use nom::bytes::complete::tag;
    use nom::combinator::opt;
    use nom::multi::{many0, separated_list1};
    use nom::sequence::{separated_pair, terminated};

    #[test]
    fn d5a() {
        use nom::character::complete::i32 as int;
        let input = include_str!("../inputs/2024/5.txt");
        let input = include_str!("../inputs/2024/5_ex.txt");
        let (_, (order, pages)) = separated_pair(
        many0(terminated(separated_pair(int::<&str, ()>, tag("|"), int), tag("\n"))),
        tag("\n"),
        many0(terminated(separated_list1(tag(","), int), opt(tag("\n")))))(input).unwrap();
        dbg!(order);
        dbg!(pages);
    }
}