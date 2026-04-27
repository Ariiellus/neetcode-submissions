impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for &n in &nums {
            if !num_set.contains(&(n - 1)) {
                let mut length = 0;
                while num_set.contains(&(n + length)) {
                    length += 1;
                }
                longest = longest.max(length);
            }
        }

        longest
    }
}