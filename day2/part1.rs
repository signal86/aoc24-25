use std::fs;

fn main() {
    let raw = match fs::read_to_string("in.txt") {
        Ok(raw) => raw,
        Err(_) => panic!("uhoh"),
    };

    let mut raw_vector: Vec<_> = raw.split('\n').collect();
    raw_vector.pop();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for i in raw_vector.iter() {
        let mut line: Vec<i32> = Vec::new();
        let temp_string: Vec<_> = i.split_whitespace().collect();
        for v in temp_string.iter() {
            line.push(v.parse().unwrap());
        }
        reports.push(line);
    }
    println!("{:?}", reports);

    let mut safe: i32 = 0;
    for report in reports {
        let mut level_safe: bool = true;
        let mut change: i32 = 0;
        for level_index in 1..report.len() {
            // not the same
            if report[level_index] == report[level_index - 1] {
                level_safe = false;
                break;
            }
            // differing in change direction
            if level_index == 1 {}
            else if change > 0 {
                if report[level_index] - report[level_index - 1] < 0 {
                    level_safe = false;
                    break;
                }
            }
            else if change < 1 {
                if report[level_index] - report[level_index - 1] > 0 {
                    level_safe = false;
                    break;
                }
            }
            change = report[level_index] - report[level_index - 1];
            // change is too great/minute
            if change.abs() > 3 || change.abs() < 1 {
                level_safe = false;
                break;
            }
        }
        safe += match level_safe {
            true => 1,
            false => 0,
        };
    }
    println!("{}", safe);
}
