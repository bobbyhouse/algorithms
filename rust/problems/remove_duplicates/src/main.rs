fn remove_duplicates(nums: &mut Vec<i32>) -> i32{
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut unique: usize = 1;
    let mut current: usize = 1;

    loop {
        if current == nums.len() {
            break;
        }
        if nums[current] != nums[current - 1] {
            nums[unique] = nums[current];
            unique += 1;
        }
        current += 1;
    }
    return unique.try_into().unwrap();
}

fn main() {
    let mut nums = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let length = remove_duplicates(&mut nums);

    println!("nums: {:?}", nums);
    println!("Length: {}", length);
}
