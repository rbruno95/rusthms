use std::collections::BinaryHeap;
use std::usize::MAX;

#[allow(dead_code)]
pub fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![MAX; adj.len()];
    dist[start] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((0, start));
    while let Some((cur_dist, u)) = queue.pop() {
        if cur_dist > dist[u] {
            continue;
        }
        for &(cost, v) in &adj[u] {
            if dist[u] + cost < dist[v] {
                dist[v] = dist[u] + cost;
                queue.push((dist[v], v));
            }
        }
    }
    dist
}
