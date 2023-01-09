use std::collections::HashMap;
use std::assert_eq;

fn is_anagram(s: String, t: String) -> bool {
    if t.len() != s.len() {
        return false;
    }

    let mut map: HashMap<char, i64> = HashMap::new();

    /*
        zip creates a tuple of our 
        first iter and the second iter 
        passed into zip

        or_default returns a mutable reference
            We must dereference our value to perform calculations
                -> dereference gets the int from the &mut i64
            refereneces cannot be calculated on.
    */
    for (a, b) in s.chars().zip(t.chars()) {
        *map.entry(a).or_default() += 1;
        *map.entry(b).or_default() -= 1;
    }
    // T: O(n)

    map.into_values().all(|cnt| cnt == 0)
    /*
        into_values() -> returns a consuminug iterator of the map values
        all -> test every element against a condition and break if not true
    */
}

fn main() {
    assert_eq!(is_anagram(String::from("racecar"), String::from("carrace")), true);
}
