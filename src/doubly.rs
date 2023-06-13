use std::cell::RefCell;
use std::rc::Rc;

pub struct DoublyLinkedList<T: std::fmt::Display> {
    head: NodePointer<T>,
    tail: NodePointer<T>,
}

type NodePointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: std::fmt::Display> {
    element: T,
    next: NodePointer<T>,
    prev: NodePointer<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}

impl<T: std::fmt::Display> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn add_to_head(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn remove_from_head(&mut self) {
        match self.head.take() {
            Some(current_head) => {
                match current_head.borrow_mut().next.take() {
                    Some(next_head) => {
                        self.head = Some(next_head);
                    }
                    None => {
                        self.head = None;
                    }
                };
            }
            None => {
                println!("List already empty")
            }
        }
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn add_to_tail(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(old_tail.clone());
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn remove_from_tail(&mut self) {
        match self.tail.take() {
            Some(current_tail) => {
                match current_tail.borrow_mut().prev.take() {
                    Some(prev_tail) => {
                        self.tail = Some(prev_tail.clone());
                        prev_tail.borrow_mut().next = None;
                    }
                    None => {
                        self.tail = None;
                    }
                };
            }
            None => {
                println!("List already empty")
            }
        }
        print!("Updated List: ");
        self.view_all_nodes();
    }

    pub fn view_all_nodes(&self) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        } else {
            let mut current_node = self.head.clone();
            while !current_node.is_none() {
                print!("{}", current_node.as_ref().unwrap().borrow().element);
                if current_node.as_ref().unwrap().borrow().next.is_some() {
                    print!(" - ");
                }
                current_node = current_node.unwrap().borrow().next.clone();
            }
            println!();
        }
    }
}
