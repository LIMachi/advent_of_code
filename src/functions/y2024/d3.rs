use regex::Regex;

pub fn y2024d3a(input: &str) -> Result<String, String> {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").map_err(|e| format!("{e:?}"))?;
    let res = regex.captures_iter(input).fold(0, |total, cap| {
        let [left, right] = cap.extract().1;
        match (left.parse::<i32>(), right.parse::<i32>()) {
            (Ok(left), Ok(right)) => total + left * right,
            _ => total
        }
    });
    Ok(res.to_string())
}

pub fn y2024d3b(input: &str) -> Result<String, String> {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").map_err(|e| format!("{e:?}"))?;
    let mut toggle = true;
    let res = regex.captures_iter(input).fold(0, |total, cap| {
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
    });
    Ok(res.to_string())
}
