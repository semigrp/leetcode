/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max = 0;
        let mut start = 0;
        let mut end = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            let mut j = 0;
            while i - j >= 0 && i + j < chars.len() && chars[i - j] == chars[i + j] {
                j += 1;
            }
            if 2 * j - 1 > max {
                max = 2 * j - 1;
                start = i - j + 1;
                end = i + j;
            }
            j = 0;
            while i - j >= 0 && i + j + 1 < chars.len() && chars[i - j] == chars[i + j + 1] {
                j += 1;
            }
            if 2 * j > max {
                max = 2 * j;
                start = i - j + 1;
                end = i + j + 1;
            }
        }
        s[start..end].to_string()
    }
}
// @lc code=end
