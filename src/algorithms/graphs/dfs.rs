#[allow(dead_code)]
pub fn dfs(adj: &mut Vec<Vec<u32>>, vis: &mut Vec<bool>, cur: usize, res: &mut Vec<u32>) {
    if vis[cur] {
        return;
    }
    vis[cur] = true;
    res.push(cur as u32);
    for i in 0..adj[cur].len() {
        if !vis[adj[cur][i] as usize] {
            dfs(adj, vis, adj[cur][i] as usize, res);
        }
    }
}
