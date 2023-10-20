use std::ops::Range;

fn main() {
    let mut nums = vec![1, 1, 2];
    let k = remove_element(&mut nums, 2);
    println!("k = {}", k);
    println!("nums = {:?}", nums);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut found: usize = 0;
    let mut range : Range<usize> = 0..nums.len();
    let mut option_i = range.next();

    loop {
        if option_i == None {
            break;
        }

        let i = option_i.unwrap();

        if i as usize >= (nums.len() - found as usize) {
            break;
        }

        let current_move_pos = (nums.len() - 1) - found as usize;
        if nums[i] == val {
            nums[i] = nums[current_move_pos];
            found += 1;
        } else {
            option_i = range.next();
        }
    }
    return (nums.len() - found) as i32;
}
