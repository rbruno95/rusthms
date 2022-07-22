#[allow(dead_code)]
pub fn dfs(adj: &Vec<Vec<usize>>, vis: &mut Vec<bool>, u: usize) {
    vis[u] = true;
    for &v in &adj[u] {
        if !vis[v] {
            dfs(adj, vis, v);
        }
    }
}
