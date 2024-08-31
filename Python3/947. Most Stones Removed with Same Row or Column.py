class Solution:
    class UnionFind():
        def __init__(self):
            self.parent = {}
        
        def find(self, x):
            if self.parent.setdefault(x, x) != x:
                self.parent[x] = self.find(self.parent[x])
            return self.parent[x]
        
        def union(self, x, y):
            rootX = self.find(x)
            rootY = self.find(y)
            if rootX != rootY:
                self.parent[rootX] = rootY

    def removeStones(self, stones: List[List[int]]) -> int:
        uf = self.UnionFind()

        for x, y in stones:
            uf.union(x, ~y)
        
        return len(stones) - len({uf.find(x) for x, y in stones})
