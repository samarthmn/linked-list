#[derive(Debug)]
pub struct SinglyLinkedList {
    node: NodePointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: NodePointer,
}

type NodePointer = Option<Box<Node>>;

impl SinglyLinkedList {
    pub fn new() -> SinglyLinkedList {
        SinglyLinkedList { node: None }
    }

    fn create_node(element: i32, node: NodePointer) -> NodePointer {
        Some(Box::new(Node {
            element,
            next: node,
        }))
    }

    pub fn add(&mut self, element: i32) {
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
        let mut all_nodes: Vec<i32> = Vec::new();
        loop {
            match selected_node {
                Some(node) => {
                    all_nodes.push(node.element);
                    selected_node = &node.next;
                }
                None => break,
            };
        }
        println!("List: {:?}", all_nodes.iter().rev().collect::<Vec<&i32>>())
    }
}
