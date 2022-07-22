// linked list node struct
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, next: None }
    }
}

pub struct LinkedList<T> {
    pub tail: Option<Box<Node<T>>>,
    pub size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            tail: None,
            size: 0,
        }
    }
    pub fn add_node(&mut self, data: T) {
        let new_node: Node<T> = Node {
            data,
            next: self.tail.take(),
        };
        self.tail = Some(Box::new(new_node));
        self.size += 1;
    }
    pub fn remove_last(&mut self) -> T {
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
