class Solution:
    def construct2DArray(self, original: List[int], m: int, n: int) -> List[List[int]]:
        if len(original) != m*n: return []
        return [original[j*n:(j+1)*n] for j in range(m)]