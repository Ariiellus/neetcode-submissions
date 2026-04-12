 impl Solution {
      pub fn encode(strs: Vec<String>) -> String {
          let mut res = String::new();

          for s in strs {
              res.push_str(&s.len().to_string());
              res.push('#');
              res.push_str(&s);
          }

          res
      }

      pub fn decode(encoded: String) -> Vec<String> {
          let mut res = Vec::new();
          let mut i = 0;

          while i < encoded.len() {
              let mut j = i;

              while encoded.as_bytes()[j] != b'#' {
                  j += 1;
              }

              let length: usize = encoded[i..j].parse().unwrap();
              let start = j + 1;
              let end = start + length;

              res.push(encoded[start..end].to_string());
              i = end;
          }

          res
      }
  }
