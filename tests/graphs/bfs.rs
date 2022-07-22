use rusthms::algorithms::graphs::bfs::bfs;

#[test]
pub fn it_bfs_path_through_graph() {
    // Creating a graph with 6 vertices
    //       5
    //     /   \
    //    2     3
    //   /     / \
    //  0     4   1
    let mut vec = vec![];
    vec.push(vec![2]);
    vec.push(vec![3]);
    vec.push(vec![0, 5]);
    vec.push(vec![1, 4, 5]);
    vec.push(vec![3]);
    vec.push(vec![2, 3]);

    // Declaring visited array of len 6
    let mut vis = vec![false; 6];

    bfs(&mut vec, &mut vis, 5);

    // Asserting that vertices are visited
    assert_eq!(vec![true; 6], vis);
}
