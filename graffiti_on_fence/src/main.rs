use std::io;

fn main() {
    let (fence_len, sections) = parse();
    
    let mut fence = vec![false; fence_len];
    for (start, end) in sections {
        for i in start..end {
            fence[i] = true;
        }
    }

    let mut all_painted = true;
    let mut start = 0;
    for i in 1..fence_len {
        if fence[i - 1] && !fence[i] {
            start = i;
        } else if !fence[i - 1] && fence[i] {
            all_painted = false;
            println!("{start} {i}");
        }
    }
    if !fence[fence_len - 1] {
        all_painted = false;
        println!("{start} {fence_len}");
    }

    if all_painted {
        println!("All painted");
    }
}

fn parse() -> (usize, Vec<(usize, usize)>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let fence_len: usize = input_line.trim().parse().unwrap();
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_sections: usize = input_line.trim().parse().unwrap();
    let mut sections: Vec<(usize, usize)> = vec![];
    for _ in 0..nb_sections {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let (start, end) = input_line.trim().split_once(" ").unwrap();
        sections.push((start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap()));
    }
    (fence_len, sections)
}
