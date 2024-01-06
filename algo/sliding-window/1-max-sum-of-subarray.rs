/*
* Maximum Sum Subarray of Size K
* Given an array of positive numbers and a positive number ‘k’, find the maximum sum of any contiguous subarray of size ‘k’.

* Example 1:
* Input: [2, 1, 5, 1, 3, 2], k=3
* Output: 9
* Explanation: Subarray with maximum sum is [5, 1, 3].

* Example 2:
* Input: [2, 3, 4, 1, 5], k=2
* Output: 7
* Explanation: Subarray with maximum sum is [3, 4].
*/

/*
Driver code
fn main() {
    let arr = vec![2, 1, 5, 1, 3, 2];
    let k = 3;
    println!("max_sum({:?}, {}) = {}", arr.clone(), k, max_sum(arr, k));
}
*/

// Solution

pub fn max_sum(arr: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut max_sum = std::i32::MIN;

    let mut i = 0;
    let mut j = 0;

    while j < arr.len() {
        sum += arr[j];
        if j as i32 - i as i32 + 1 != k {
            j += 1;
        } else if j as i32 - i as i32 + 1 == k {
            max_sum = std::cmp::max(sum, max_sum);
            sum -= arr[i];
            i += 1;
            j += 1;
        }
    }
    max_sum
}
