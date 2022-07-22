use rusthms::algorithms::data_structures::linked_list::LinkedList;
use rusthms::algorithms::data_structures::linked_list::Node;

fn create_node(id: i32) -> Node<i32> {
    Node {
        data: Some(Box::new(id)),
        next: None,
    }
}

fn create_linked_list() -> LinkedList<i32> {
    LinkedList {
        tail: None,
        size: 0,
    }
}

#[test]
pub fn it_add_node() {
    let mut linked_list = create_linked_list();
    let mut node = create_node(1);
    linked_list.add_node(node);
    assert_eq!(linked_list.size, 1);
    node = create_node(2);
    linked_list.add_node(node);
    assert_eq!(linked_list.size, 2);
    node = create_node(3);
    linked_list.add_node(node);
    assert_eq!(linked_list.size, 3);
}

#[test]
pub fn it_remove_node() {
    let mut linked_list = create_linked_list();
    let node_1 = create_node(1);
    let node_2 = create_node(2);
    let node_3 = create_node(3);
    linked_list.add_node(node_1);
    linked_list.add_node(node_2);
    linked_list.add_node(node_3);
    assert_eq!(Some(Box::new(3)), linked_list.remove_last());
    assert_eq!(Some(Box::new(2)), linked_list.remove_last());
    assert_eq!(Some(Box::new(1)), linked_list.remove_last());
}
