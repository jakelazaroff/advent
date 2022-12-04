use itertools::Itertools;
use std::collections::HashSet;

trait Item {
    fn priority(self) -> u32;
}

impl Item for char {
    fn priority(self) -> u32 {
        if self.is_lowercase() {
            self as u32 - 96
        } else {
            self as u32 - 38
        }
    }
}

fn main() {
    let txt = include_bytes!("input.txt");
    let input = String::from_utf8(txt.to_vec()).unwrap();

    let rucksacks: u32 = input
        .split("\n") // split input by strings
        .map(|contents| contents.split_at(contents.len() / 2)) // split each string in half
        .filter_map(|(one, two)| {
            // create a hash set of the items in the first string
            let items: HashSet<char> = HashSet::from_iter(one.chars());
            // find the item in the second string also contained in the first
            two.chars().find(|item| items.contains(item))
        })
        .map(|item| item.priority()) // get the item priority
        .sum(); // summ all the items

    println!("Duplicate priorities: {}", rucksacks);

    let badges: u32 = input
        .split("\n") // split input by strings
        .chunks(3) // group input into chunks of three lines
        .into_iter()
        .map(|group| {
            // convert each line in the group into a hashset of the characters
            group.map(|items| -> HashSet<char> { HashSet::from_iter(items.chars()) })
        })
        .map(|mut group| {
            // intersect each group with the first
            let mut result = group.next().unwrap_or(HashSet::new());
            for items in group {
                result.retain(|&item| items.contains(&item));
            }
            result
        })
        .filter_map(|items| items.into_iter().next()) // get the first item from the group
        .map(|item| item.priority()) // get the item priority
        .sum(); // sum all the items

    println!("Badge priorities: {}", badges);
}
