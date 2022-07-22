use rusthms::algorithms::data_structures::linked_list::LinkedList;

#[test]
pub fn it_add_node() {
    let mut linked_list = LinkedList::new();
    linked_list.add_node(1);
    assert_eq!(linked_list.size, 1);
    linked_list.add_node(2);
    assert_eq!(linked_list.size, 2);
    linked_list.add_node(3);
    assert_eq!(linked_list.size, 3);
}

#[test]
pub fn it_remove_node() {
    let mut linked_list = LinkedList::new();
    linked_list.add_node(1);
    linked_list.add_node(2);
    linked_list.add_node(3);
    assert_eq!(3, linked_list.remove_last());
    assert_eq!(linked_list.size, 2);
    assert_eq!(2, linked_list.remove_last());
    assert_eq!(linked_list.size, 1);
    assert_eq!(1, linked_list.remove_last());
    assert_eq!(linked_list.size, 0);
}
