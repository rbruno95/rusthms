#[allow(dead_code)]
pub fn dfs(adj: &mut Vec<Vec<usize>>, vis: &mut Vec<bool>, v: usize, res: &mut Vec<usize>) {
    if vis[v] {
        return;
    }
    vis[v] = true;
    res.push(v);
    for u in 0..adj[v].len() {
        if !vis[adj[v][u]] {
            dfs(adj, vis, adj[v][u], res);
        }
    }
}
