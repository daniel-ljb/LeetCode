use std::collections::HashMap;

struct FreqStack {
    frequencyCount: HashMap<i32, i32>,
    stack: Vec<Vec<i32>>,
    maxCount: i32,
}

impl FreqStack {

    fn new() -> Self {
        FreqStack {
            frequencyCount: HashMap::new(),
            stack: Vec::new(),
            maxCount: 0,
        }
    }
    
    fn push(&mut self, val: i32) {
        let count: &mut i32 = self.frequencyCount.entry(val).or_insert(0);
        *count += 1;
        while self.stack.len() < *count as usize {
            self.stack.push(Vec::new());
        }
        self.stack[(*count - 1) as usize].push(val);
        if *count > self.maxCount {
            self.maxCount = *count;
        }
    }
    
    fn pop(&mut self) -> i32 {
        let val = self.stack[(self.maxCount - 1) as usize].pop().unwrap();
        *self.frequencyCount.get_mut(&val).unwrap() -= 1;
        if self.stack[(self.maxCount - 1) as usize].is_empty() {
            self.maxCount -= 1;
        }
        val
    }
}