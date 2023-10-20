use std::collections::HashMap;

fn remove_duplicates(nums: &mut Vec<i32>) -> i32{
    let mut keys: HashMap<i32, bool> = HashMap::new();
    let mut found_count: usize = 0;
    let mut i: usize = 0;

    loop {
        if (i == nums.len() - found_count) {
            break;
        }

        let found = keys.get(&nums[i]);
        match found {
            Some(_) => {
                nums[i] = nums[array_end - 1];
                array_end -= 1;
            },
            None => {
                keys.insert(nums[i], true);
                i += 1;
            }
        }
    }
    return array_end as i32;
}

fn main() {
    let mut nums = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let length = remove_duplicates(&mut nums);

    println!("nums: {:?}", nums);
    println!("Length: {}", length);
}
