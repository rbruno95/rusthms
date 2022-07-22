// linked list node struct
pub struct Node<T> {
    pub data: Option<Box<T>>,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub tail: Option<Box<Node<T>>>,
    pub size: usize,
}

impl<T> LinkedList<T> {
    pub fn add_node(&mut self, node: Node<T>) {
        let new_node: Node<T> = Node {
            data: node.data,
            next: self.tail.take(),
        };
        self.tail = Some(Box::new(new_node));
        self.size += 1;
    }
    pub fn remove_last(&mut self) -> Option<Box<T>> {
        self.tail
            .take()
            .map(|node| {
                self.tail = node.next;
                self.size -= 1;
                node.data
            })
            .unwrap()
    }
}
