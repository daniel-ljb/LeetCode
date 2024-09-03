class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        indexes = {}
        for i, num in enumerate(nums):
            complement = target - num
            if complement in indexes:
                return [indexes[complement], i]
            else:
                indexes[num] = i
