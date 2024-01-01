pub fn remove_duplicates(mut nums: Vec<i32>) -> i32 {
    let mut i: i32 = 0;
    for j in 1..nums.len() {
        if nums[i as usize] != nums[j] {
            i += 1;
            nums[i as usize] = nums[j]
        }
    }
    i + 1
}
