use std::fs;

fn main() {
    let raw = match fs::read_to_string("in.txt") {
        Ok(raw) => raw,
        Err(_) => panic!("uhoh"),
    };

    let raw_vector: Vec<_> = raw.split_whitespace().collect();
    raw_vector.join(" ");
    //println!("{:?}", v);
    let mut vec = Vec::new();
    let iter = raw_vector.iter();
    for i in iter {
        let a: i32 = i.parse().unwrap();
        vec.push(a);
    }
    println!("{:?}", vec);
    let mut similarity: i32 = 0;
    for (i, v) in vec.iter().enumerate() {
        if i % 2 == 0 {
            for (j, k) in vec.iter().enumerate() {
                if j % 2 == 1 && *k == *v {
                    similarity += *v;
                }
            }
        }
    }
    println!("{}", similarity);
}
