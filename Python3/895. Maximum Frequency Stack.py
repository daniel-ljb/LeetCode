class FreqStack:

    def __init__(self):
        self.frequencyCount = {}
        self.maxCount = 0
        self.stack = []

    def push(self, val: int) -> None:
        count = 1 + self.frequencyCount.get(val, 0)
        self.frequencyCount[val] = count
        self.maxCount = max(self.maxCount, count)
        while len(self.stack) < count:
            self.stack.append([])
        self.stack[count - 1].append(val)
    
    def pop(self) -> int:
        val = self.stack[self.maxCount - 1].pop()
        if len(self.stack[self.maxCount - 1]) == 0:
            self.maxCount -= 1
        self.frequencyCount[val] -= 1
        return val        


# Your FreqStack object will be instantiated and called as such:
# obj = FreqStack()
# obj.push(val)
# param_2 = obj.pop()