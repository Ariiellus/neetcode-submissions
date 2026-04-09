impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
          let mut count: HashMap<i32, i32> = HashMap::new();
          let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

          for n in nums {
              *count.entry(n).or_insert(0) += 1;
          }

          for (n, c) in count {
              freq[c as usize].push(n);
          }

          let mut res = Vec::new();

          for i in (1..freq.len()).rev() {
              for &n in &freq[i] {
                  res.push(n);
                  if res.len() == k as usize {
                      return res;
                  }
              }
          }

          res
      }
  }
