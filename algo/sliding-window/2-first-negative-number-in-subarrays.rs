/*
* Given an array and a positive integer k, find the first negative integer for each and every window(contiguous subarray) of size k.
* If a window does not contain a negative integer, then print 0 for that window.

* Examples:

* Input : arr[] = {-8, 2, 3, -6, 10}, k = 2
* Output : -8 0 -6 -6
* First negative integer for each window of size k
* {-8, 2} = -8
* {2, 3} = 0 (does not contain a negative integer)
* {3, -6} = -6
* {-6, 10} = -6

* Input : arr[] = {12, -1, -7, 8, -15, 30, 16, 28}, k = 3
* Output : -1 -1 -7 -15 -15 0

*/

/*
Driver code
fn main() {
    let arr = vec![1, -1, -2, 3, 4];
    let k = 3;
    let ans = first_neg_number(arr, k);
    println!("ans = {:?}", ans);
}

 */

pub fn first_neg_number(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut buff: std::collections::VecDeque<i32> = std::collections::VecDeque::new();

    let mut i = 0;
    let mut j = 0;

    while j < arr.len() {
        if arr[j] < 0 {
            buff.push_back(arr[j]);
        }

        if j as i32 - i as i32 + 1 < k {
            j += 1;
        } else if j as i32 - i as i32 + 1 == k {
            if buff.len() == 0 {
                ans.push(0);
                i += 1;
            } else {
                let front = match buff.front() {
                    Some(f) => *f,
                    None => 0,
                };
                ans.push(front);
                if arr[i] == front {
                    buff.pop_front();
                    i += 1;
                } else {
                    i += 1;
                }
            }
        }
    }

    ans
}
