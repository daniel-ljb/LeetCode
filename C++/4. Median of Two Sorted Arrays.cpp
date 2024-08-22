class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        int N1 = nums1.size();
        int N2 = nums2.size();
        if (N2 > N1) return findMedianSortedArrays(nums2, nums1);

        int low = 0;
        int high = 2 * N2;
        while (low <= high) {
            int C2 = (low + high) / 2;
            int C1 = N1 + N2 - C2;

            double L1 = (C1 == 0) ? INT_MIN : nums1[(C1 - 1)/2];
            double L2 = (C2 == 0) ? INT_MIN : nums2[(C2 - 1)/2];
            double R1 = (C1 == 2 * N1) ? INT_MAX : nums1[C1 / 2];
            double R2 = (C2 == 2 * N2) ? INT_MAX : nums2[C2 / 2];

            if (L1 > R2) low = C2 + 1;
            else if (L2 > R1) high = C2 - 1;
            else return (max(L1, L2) + min(R1, R2)) / 2; 
        }
        return -1;
    }
};