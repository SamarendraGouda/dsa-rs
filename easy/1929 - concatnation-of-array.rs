pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut arr_copy = nums.clone();
    for num in nums {
        arr_copy.push(num);
    }
    arr_copy
}
