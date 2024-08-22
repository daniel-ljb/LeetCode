/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let mut lower: i32 = 0;
        let length: i32 = mountainArr.length();
        let mut upper: i32 = length - 1;
        let maxIndex: i32 = loop {
            if lower >= upper {
                break lower
            }
            let mid: i32 = (lower + upper) / 2;
            let midValue: i32 = mountainArr.get(mid);
            if mid == length {
                break midValue
            }
            let midValueP: i32 = mountainArr.get(mid + 1);
            if midValue < midValueP {
                lower = mid + 1;
            }
            else {
                upper = mid;
            }
        };
        lower = 0;
        upper = maxIndex;
        loop {
            if upper <= lower {
                if mountainArr.get(lower) == target {
                    return lower;
                }
                else {
                    break
                }
            }
            let mid: i32 = (lower + upper + 1) / 2;
            let midValue: i32 = mountainArr.get(mid);
            if midValue > target {
                upper = mid - 1;
            }
            else {
                lower = mid;
            }
        }
        lower = maxIndex;
        upper = length - 1;
        loop {
            if upper <= lower {
                if mountainArr.get(lower) == target {
                    return lower;
                }
                else {
                    break
                }
            }
            let mid: i32 = (lower + upper + 1) / 2;
            let midValue: i32 = mountainArr.get(mid);
            if midValue < target {
                upper = mid - 1;
            }
            else {
                lower = mid;
            }
        }
        -1
    }
}