class Solution {
public:
    long long minimumReplacement(vector<int>& nums) {
        long long steps = 0;
        int x = nums.back();

        for (int i = nums.size() - 2; i >= 0; i--) {
            int split = (nums[i] - 1) / x;
            steps += split;
            x = nums[i] / (split + 1);
        }

        return steps;
    }
};