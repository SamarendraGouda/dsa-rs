pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    
    let mut i = 0;
    let mut j = 0;

    while j < arr.len(){
        sum += arr[j];
        if j as i32 - i as i32 + 1 != k {
            j +=1;
        } else if j as i32 - i as i32 + 1 == k {
            let average: f32 = sum as f32 / k as f32;
            if average >= threshold as f32{
                count += 1;
            }
            sum = sum - arr[i];
            i += 1;
            j += 1;
        }
    }
    count
}