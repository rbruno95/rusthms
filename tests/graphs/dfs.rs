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

    // Declaring an array of len 6
    let mut path = vec![];

    // Calling dfs function on the graph
    dfs(&mut vec, &mut vec![false; 6], 5, &mut path);

    // Asserting that the graph is traversed in the correct order
    assert_eq!(vec![5, 2, 0, 3, 1, 4], path);
}
