/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                nums.push(nums1[i]);
                i += 1;
            } else {
                nums.push(nums2[j]);
                j += 1;
            }
        }
        while i < nums1.len() {
            nums.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            nums.push(nums2[j]);
            j += 1;
        }
        let len = nums.len();
        if len % 2 == 0 {
            (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0
        } else {
            nums[len / 2] as f64
        }
    }
}
// @lc code=end
