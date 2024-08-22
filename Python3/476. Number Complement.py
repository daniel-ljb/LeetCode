class Solution:
    def findComplement(self, num: int) -> int:
        n = 1
        while n <= num: n *= 2
        n -= 1
        return num ^ n