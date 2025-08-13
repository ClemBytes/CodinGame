use std::io;

fn main() {
    let (fence_len, mut sections) = parse();

    // Sort sections by start points
    sections.sort();

    // Then merge touching of overlapping intervals
    let mut merged_sections: Vec<(usize, usize)> = vec![];
    let mut current_section = sections[0];
    for (i, next_section) in sections.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if next_section.0 <= current_section.1 {
            current_section.1 = current_section.1.max(next_section.1);
        } else {
            merged_sections.push(current_section);
            current_section = *next_section;
        }
    }
    merged_sections.push(current_section);

    // All painted case
    if merged_sections.len() == 1 && merged_sections[0].0 == 0 && merged_sections[0].1 == fence_len {
        println!("All painted");
        return;
    }

    // Otherwise: look for empty parts
    if merged_sections[0].0 != 0 {
        println!("{} {}", 0, merged_sections[0].0);
    }
    let nb_sections = merged_sections.len();
    for i in 1..nb_sections {
        println!("{} {}", merged_sections[i - 1].1, merged_sections[i].0);
    }
    if merged_sections[nb_sections - 1].1 != fence_len {
        println!("{} {}", merged_sections[nb_sections - 1].1, fence_len);
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
