fn main() {
    let txt = include_bytes!("input.txt");
    let input = String::from_utf8(txt.to_vec()).unwrap();

    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter_map(|x| x.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect();
    elves.sort_by(|a, b| b.cmp(a));

    println!("Most calories: {}", elves[0]);
    println!("Top three: {}", elves[0..3].iter().sum::<u32>());
}
