use rusthms::algorithms::graphs::dfs::dfs;

#[test]
pub fn it_dfs_path_through_graph() {
    // Creating a graph with 6 vertices
    //       5
    //     /   \
    //    2     3
    //   /     / \
    //  0     4   1
    let mut vec = vec![
        vec![2],
        vec![3],
        vec![0, 5],
        vec![1, 4, 5],
        vec![3],
        vec![2, 3],
    ];

    // Initializing a vector of booleans to mark visited vertices
    let mut vis = vec![false; 6];

    // Calling dfs function on the graph
    dfs(&mut vec, &mut vis, 5);

    // Asserting that the graph is traversed in the correct order
    assert_eq!(vec![true; 6], vis);
}
