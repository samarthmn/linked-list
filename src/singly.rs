#[derive(Debug)]
pub struct SinglyLinkedList<T: std::fmt::Debug + std::marker::Copy> {
    node: NodePointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: NodePointer<T>,
}

type NodePointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList { node: None }
    }

    fn create_node(element: T, node: NodePointer<T>) -> NodePointer<T> {
        Some(Box::new(Node {
            element,
            next: node,
        }))
    }

    pub fn add(&mut self, element: T) {
        let current_node = self.node.take();
        let new_node = Self::create_node(element, current_node);
        self.node = new_node;
    }

    pub fn remove(&mut self) {
        let current_node = self.node.take();
        let new_node = match current_node {
            Some(node) => node.next,
            None => None,
        };
        self.node = new_node;
    }

    pub fn view_last_node(&self) {
        match &self.node {
            Some(node) => {
                println!("Element: {:?}", node.element)
            }
            None => {
                println!("None")
            }
        }
    }

    pub fn view_all_nodes(&self) {
        let mut selected_node = &self.node;
        let mut all_nodes: Vec<T> = Vec::new();
        loop {
            match selected_node {
                Some(node) => {
                    all_nodes.push(node.element);
                    selected_node = &node.next;
                }
                None => break,
            };
        }
        println!("List: {:?}", all_nodes.iter().rev().collect::<Vec<&T>>())
    }
}
