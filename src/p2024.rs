use regex::Regex;
use nom::Parser;

enum XmasDirection {
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
    Top,
    TopRight
}

impl XmasDirection {
    fn to_delta(&self) -> (isize, isize) {
        match self {
            XmasDirection::Right => (1, 0),
            XmasDirection::BottomRight => (1, 1),
            XmasDirection::Bottom => (0, 1),
            XmasDirection::BottomLeft => (-1, 1),
            XmasDirection::Left => (-1, 0),
            XmasDirection::TopLeft => (-1, -1),
            XmasDirection::Top => (0, -1),
            XmasDirection::TopRight => (1, -1)
        }
    }

    fn rotate_clockwise(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::BottomRight,
            XmasDirection::BottomRight => XmasDirection::Bottom,
            XmasDirection::Bottom => XmasDirection::BottomLeft,
            XmasDirection::BottomLeft => XmasDirection::Left,
            XmasDirection::Left => XmasDirection::TopLeft,
            XmasDirection::TopLeft => XmasDirection::Top,
            XmasDirection::Top => XmasDirection::TopRight,
            XmasDirection::TopRight => XmasDirection::Right
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::TopRight,
            XmasDirection::BottomRight => XmasDirection::Right,
            XmasDirection::Bottom => XmasDirection::BottomRight,
            XmasDirection::BottomLeft => XmasDirection::Bottom,
            XmasDirection::Left => XmasDirection::BottomLeft,
            XmasDirection::TopLeft => XmasDirection::Left,
            XmasDirection::Top => XmasDirection::TopLeft,
            XmasDirection::TopRight => XmasDirection::Top,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::Left,
            XmasDirection::BottomRight => XmasDirection::TopLeft,
            XmasDirection::Bottom => XmasDirection::Top,
            XmasDirection::BottomLeft => XmasDirection::TopRight,
            XmasDirection::Left => XmasDirection::Right,
            XmasDirection::TopLeft => XmasDirection::BottomRight,
            XmasDirection::Top => XmasDirection::Bottom,
            XmasDirection::TopRight => XmasDirection::BottomLeft,
        }
    }

    fn array() -> [Self;8] {
        [
            XmasDirection::Right,
            XmasDirection::BottomRight,
            XmasDirection::Bottom,
            XmasDirection::BottomLeft,
            XmasDirection::Left,
            XmasDirection::TopLeft,
            XmasDirection::Top,
            XmasDirection::TopRight
        ]
    }

    fn corners() -> [Self;4] {
        [
            XmasDirection::BottomRight,
            XmasDirection::BottomLeft,
            XmasDirection::TopLeft,
            XmasDirection::TopRight
        ]
    }

    fn cardinals() -> [Self;4] {
        [
            XmasDirection::Right,
            XmasDirection::Bottom,
            XmasDirection::Left,
            XmasDirection::Top
        ]
    }

    fn get_grid_delta<'g, T>(&self, grid: &'g Vec<Vec<T>>, start: (usize, usize), steps: usize) -> Option<&'g T> {
        let (dx, dy) = self.to_delta();
        let (dx, dy) = (dx * steps as isize, dy * steps as isize);
        if start.0 as isize + dx >= 0 && start.1 as isize + dy >= 0 {
            grid.get((start.1 as isize + dy) as usize).and_then(|l| l.get((start.0 as isize + dx) as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::opt;
    use nom::multi::{many0, separated_list1};
    use nom::sequence::{separated_pair, terminated};
    use super::*;

    #[test]
    fn d1a() {
        let input = include_str!("../inputs/2024/1.txt");
        let (mut left, mut right) = input.split("\n")
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
            });
        left.sort();
        right.sort();
        println!("{}", left.iter().zip(right.iter()).fold(0u32, |a, (l, r)| {
            a + l.abs_diff(*r)
        }));
    }

    #[test]
    fn d1b() {
        let input = include_str!("../inputs/2024/1.txt");
        let (left, right) = input.split("\n")
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
            });
        println!("{}", left.iter().fold(0, |total, l| {
            total + *l * right.iter().fold(0, |i, c| if *c == *l { i + 1 } else { i })
        }));
    }

    fn d2(report: &Vec<i32>) -> bool {
        if report.len() <= 1 { true } else {
            if report[0] < report[1] {
                for i in 0..report.len() - 1 {
                    let delta = report[i + 1] - report[i];
                    if delta <= 0 || delta > 3 { return false; }
                }
                true
            } else if report[0] > report[1] {
                for i in 0..report.len() - 1 {
                    let delta = report[i] - report[i + 1];
                    if delta <= 0 || delta > 3 { return false; }
                }
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn d2a() {
        let input = include_str!("../inputs/2024/2.txt");
        println!("{}", input.split("\n").fold(0u32, |total, report| {
            if d2(&report.split_ascii_whitespace().filter_map(|ds| ds.parse::<i32>().ok()).collect::<Vec<i32>>()) { total + 1 } else { total }
        }));
    }

    #[test]
    fn d2b() {
        let input = include_str!("../inputs/2024/2.txt");
        println!("{}", input.split("\n").fold(0u32, |total, report| {
            let report = report.split_ascii_whitespace().filter_map(|ds| ds.parse::<i32>().ok()).collect::<Vec<i32>>();
            if d2(&report) { return total + 1; }
            for s in 0..report.len() {
                if d2(&report.iter().enumerate().filter_map(|(i, v)| if i == s { None } else { Some(*v) }).collect::<Vec<i32>>()) {
                    return total + 1;
                }
            }
            total
        }));
    }

    #[test]
    fn d3a() {
        let input = include_str!("../inputs/2024/3.txt");
        let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        println!("{}", regex.captures_iter(input).fold(0, |total, cap| {
            let [left, right] = cap.extract().1;
            match (left.parse::<i32>(), right.parse::<i32>()) {
                (Ok(left), Ok(right)) => total + left * right,
                _ => total
            }
        }));
    }

    #[test]
    fn d3b() {
        let input = include_str!("../inputs/2024/3.txt");
        let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
        let mut toggle = true;
        println!("{}", regex.captures_iter(input).fold(0, |total, cap| {
            if cap.get(3).is_some() {
                toggle = true;
                total
            } else if cap.get(4).is_some() {
                toggle = false;
                total
            } else if toggle {
                match (cap.get(1).and_then(|g| g.as_str().parse::<i32>().ok()), cap.get(2).and_then(|g| g.as_str().parse::<i32>().ok())) {
                    (Some(left), Some(right)) => total + left * right,
                    _ => total
                }
            } else {
                total
            }
        }));
    }

    #[test]
    fn d4a() {
        let input = include_str!("../inputs/2024/4.txt");
        let grid = input.split("\n").map(|l| {
            l.as_ascii().unwrap().iter().map(|c| match c.to_char() {
                'X' => 0,
                'M' => 1,
                'A' => 2,
                'S' => 3,
                c @ _ => unreachable!("the input should not have this char: {c}")
            }).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();
        let mut count = 0;
        for y in 0..grid.len(){
            for x in 0..grid[y].len() {
                if grid[y][x] != 0 { continue }
                for d in XmasDirection::array() {
                    for i in 1..4 {
                        if !d.get_grid_delta(&grid, (x, y), i).map_or(false, |v| *v == i as u8) {
                            break;
                        }
                        if i == 3 {
                            count += 1;
                        }
                    }
                }
            }
        }
        println!("{}", count);
    }

    #[test]
    fn d4b() {
        let input = include_str!("../inputs/2024/4.txt");
        let grid = input.split("\n").map(|l| {
            l.as_ascii().unwrap().iter().map(|c| match c.to_char() {
                'X' => 3,
                'M' => 1,
                'A' => 0,
                'S' => 2,
                c @ _ => unreachable!("the input should not have this char: {c}")
            }).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();
        let mut count = 0;
        for y in 0..grid.len() {
            'main_loop: for x in 0..grid[y].len() {
                if grid[y][x] != 0 { continue }
                for d in XmasDirection::corners() { //find first m, clockwise -> check if opposite is s, find next m quarter clockwise/counter clockwise, check opposite s
                    if d.get_grid_delta(&grid, (x, y), 1).map_or(false, |v| *v == 1) {
                        if d.opposite().get_grid_delta(&grid, (x, y), 1).map_or(false, |v| *v == 2) {
                            let d = d.rotate_clockwise().rotate_clockwise();
                            let t = d.get_grid_delta(&grid, (x, y), 1).map_or(0, |v| *v);
                            if t == 1 || t == 2 {
                                if d.opposite().get_grid_delta(& grid, (x, y), 1).map_or(false, |v| (*v == 2 && t == 1) || (*v == 1 && t == 2)) {
                                    count += 1;
                                    continue 'main_loop;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("{}", count);
    }

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