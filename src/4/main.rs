fn main() {
    let txt = include_bytes!("input.txt");
    let input = String::from_utf8(txt.to_vec()).unwrap();

    let contains = input
        .split("\n")
        .filter_map(|pair| pair.split_once(","))
        .filter_map(|(a, b)| Some((a.split_once("-")?, b.split_once("-")?)))
        .filter_map(|(a, b)| {
            Some((
                (a.0.parse::<u32>().ok()?, a.1.parse::<u32>().ok()?),
                (b.0.parse::<u32>().ok()?, b.1.parse::<u32>().ok()?),
            ))
        })
        .map(|(a, b)| {
            let a_contains_b = a.0 >= b.0 && a.1 <= b.1;
            let b_contains_a = b.0 >= a.0 && b.1 <= a.1;
            a_contains_b || b_contains_a
        })
        .filter(|contains| *contains)
        .count();

    println!("Proper subsets: {:?}", contains);

    let overlaps = input
        .split("\n")
        .filter_map(|pair| pair.split_once(","))
        .filter_map(|(a, b)| Some((a.split_once("-")?, b.split_once("-")?)))
        .filter_map(|(a, b)| {
            Some((
                (a.0.parse::<u32>().ok()?, a.1.parse::<u32>().ok()?),
                (b.0.parse::<u32>().ok()?, b.1.parse::<u32>().ok()?),
            ))
        })
        .map(|(a, b)| a.0 <= b.1 && b.0 <= a.1)
        .filter(|contains| *contains)
        .count();

    println!("Overlaps: {:?}", overlaps);
}
