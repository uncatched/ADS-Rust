use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let mut list = LinkedList::new();
    list.append(Node::new(1));
    list.append(Node::new(3));
    list.append(Node::new(4));

    list.print();

    let node = list.get_node(0);
    println!("Node: {:?}", node);

    let value = list.get_value(0);
    println!("Value: {:?}", value);

    let poped = list.pop();
    println!("Poped: {:?}", poped);

    list.print()
}

#[derive(Debug)]
struct Node<T: Debug> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T: Debug> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            next: None
        }
    }
}

struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>
}

impl<T: Debug> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { 
            head: None
        }
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count += 1;
            current_node = node.next.as_ref();
        }

        count
    }

    fn append(&mut self, node: Node<T>) {
        let length = self.len();
        if length > 0 {
            let last = self.get_mut_node(length - 1);
            last.map(|n| n.next = Some(Box::new(node)));
        } else {
            self.head = Some(Box::new(node));
        }
    }

    fn get_mut_node(&mut self, index: usize) -> Option<&mut Box<Node<T>>> {
        let mut next = self.head.as_mut();
        for _ in 0..index {
            next = match next {
                Some(node) => node.next.as_mut(),
                None => None
            }
        }

        next
    }

    fn get_node(&self, index: usize) -> Option<&Box<Node<T>>> {
        let mut next = self.head.as_ref();
        for _ in 0..index {
            next = match next {
                Some(node) => node.next.as_ref(),
                None => None
            }
        }
        next
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.data
        })
    }

    fn get_value(&self, index: usize) -> Option<&T> {
        match self.get_node(index) {
            Some(n) => Some(&n.data),
            None => None
        }
    }

    fn print(&self) {
        let length = self.len();
        for i in 0..length {
            let node = self.get_node(i).unwrap();
            println!("{:?}", node.data);
        }
    }
}