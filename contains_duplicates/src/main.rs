use std::collections::HashSet;
use std::assert;

fn contains_duplicates(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for num in nums.iter() {
        if !set.insert(*num) {
            return true;
        }
    }
    false
}
fn main() {
    assert!(contains_duplicates([1, 1, 3, 4, 5].to_vec()));
}

/*
    Given an integer array nums, return true if any value appears 
    at least twice in the array, and return false 
    if every element is distinct.
*/