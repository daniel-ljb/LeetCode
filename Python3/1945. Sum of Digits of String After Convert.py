class Solution:
    def getLucky(self, s: str, k: int) -> int:
        n = "".join([str(ord(x)-96) for x in s])
        for _ in range(k):
            n = sum([int(x) for x in str(n)])
        return n
