fn check(report: &Vec<i32>) -> bool {
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

pub fn y2024d2a(input: &str) -> Result<String, String> {
    let res = input.split("\n").fold(0u32, |total, report| {
        if check(&report.split_ascii_whitespace().filter_map(|ds| ds.parse::<i32>().ok()).collect::<Vec<i32>>()) { total + 1 } else { total }
    });
    Ok(res.to_string())
}

pub fn y2024d2b(input: &str) -> Result<String, String> {
    let res = input.split("\n").fold(0u32, |total, report| {
        let report = report.split_ascii_whitespace().filter_map(|ds| ds.parse::<i32>().ok()).collect::<Vec<i32>>();
        if check(&report) { return total + 1; }
        for s in 0..report.len() {
            if check(&report.iter().enumerate().filter_map(|(i, v)| if i == s { None } else { Some(*v) }).collect::<Vec<i32>>()) {
                return total + 1;
            }
        }
        total
    });
    Ok(res.to_string())
}
