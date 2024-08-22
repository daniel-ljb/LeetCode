class FreqStack {
private:
    std::unordered_map<int, int> frequencyCount;
    std::vector<std::vector<int>> stack;
    int maxCount;
public:
    FreqStack() {
        maxCount = 0;
    }
    
    void push(int val) {
        int count = ++frequencyCount[val];
        while (stack.size() < count) {
            stack.push_back(std::vector<int>());
        }
        stack[count - 1].push_back(val);
        maxCount = max(count, maxCount);
    }
    
    int pop() {
        int val = stack[maxCount - 1].back();
        stack[maxCount - 1].pop_back();
        frequencyCount[val]--;
        if (stack[maxCount - 1].empty()) maxCount--;
        return val;
    }
};

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack* obj = new FreqStack();
 * obj->push(val);
 * int param_2 = obj->pop();
 */
