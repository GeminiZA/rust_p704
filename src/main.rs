struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut a: i32 = 0;
        let mut c: i32 = nums.len() as i32 - 1;
        while c >= a {
            let b = (a + c) / 2;
            if nums[b as usize] < target {
                a = b + 1;
            } else if nums[b as usize] > target {
                c = b - 1;
            } else {
                return b as i32;
            }
        }
        return -1;
    }
}

fn main() {
    let nums = vec![5];
    let t = 5;
    let result = Solution::search(nums, t);
    println!("{}", result);
}
