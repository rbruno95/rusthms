use rusthms::algorithms::graphs::dijkstra::dijkstra;

#[test]
pub fn it_minimum_distance() {
    // Creating a graph with 6 vertices
    // 0 - 5 - 4
    // |\ /    |
    // | 2  -  3
    // | |   /
    //  \| /
    //    1

    let adj = &vec![
        vec![(7, 1), (9, 2), (14, 5)],
        vec![(7, 0), (10, 2), (15, 3)],
        vec![(9, 0), (10, 1), (11, 3), (2, 5)],
        vec![(11, 2), (6, 4)],
        vec![(6, 3), (9, 5)],
        vec![(14, 0), (2, 2), (9, 4)],
    ];

    let dist = dijkstra(adj, 0);
    assert_eq!(vec![0, 7, 9, 20, 20, 11], dist);
}
