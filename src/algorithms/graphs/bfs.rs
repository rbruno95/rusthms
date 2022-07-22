use std::collections::VecDeque;

#[allow(dead_code)]
pub fn bfs(adj: &mut Vec<Vec<usize>>, vis: &mut Vec<bool>, v: usize) {
    let mut q = VecDeque::new();
    q.push_back(v);
    vis[v] = true;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        vis[u] = true;
        for i in 0..adj[u].len() {
            let v = adj[u][i];
            if !vis[v] {
                q.push_back(v);
            }
        }
    }
}
