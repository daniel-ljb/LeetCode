class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        nums = []
        while nums1 and nums2:
            if nums1[0] > nums2[0]:
                nums.append(nums2.pop(0))
            else:
                nums.append(nums1.pop(0))
        nums += nums1
        nums += nums2
        
        length = len(nums)
        if length%2:
            return nums[length//2]
        else:
            return sum(nums[length//2-1:length//2+1])/2
        