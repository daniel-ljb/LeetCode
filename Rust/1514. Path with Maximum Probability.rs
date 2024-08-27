impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let mut distances: Vec<f64> = vec![0.0; n as usize];
        distances[start_node as usize] = 1.0;
        for _ in 1..n {
            let mut updated: bool = false;
            for i in 0..edges.len() {
                let u: usize = edges[i][0] as usize;
                let v: usize = edges[i][1] as usize;
                if distances[v] * succ_prob[i] > distances[u] {
                    distances[u] = distances[v] * succ_prob[i];
                    updated = true;
                }
                if distances[u] * succ_prob[i] > distances[v] {
                    distances[v] = distances[u] * succ_prob[i];
                    updated = true;
                }
            }
            if !updated { break; }
        }
        distances[end_node as usize]
    }
}