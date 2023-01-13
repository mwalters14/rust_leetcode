use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    // enumerate gives us the index and value of our iter
    for (i, num) in nums.iter().enumerate() {
        match map.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => map.insert(target - num, i as i32),
        };
    }

    vec![]
}

fn main() {
    println!("Hello, world!");
}
