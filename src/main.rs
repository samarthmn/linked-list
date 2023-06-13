use linked_list::{doubly::DoublyLinkedList, singly::SinglyLinkedList};
use std::io::stdin;
enum MainMenu {
    SinglyLinkedList,
    DoublyLinkedList,
    Exit,
}

fn main() {
    let mut menu_selected: Option<MainMenu> = None;
    let mut singly_linked_list: SinglyLinkedList<i32> = SinglyLinkedList::new();
    let mut doubly_linked_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    loop {
        match menu_selected {
            None => {
                print_main_menu();
                let num = get_input();
                match num {
                    1 => menu_selected = Some(MainMenu::SinglyLinkedList),
                    2 => menu_selected = Some(MainMenu::DoublyLinkedList),
                    3 => menu_selected = Some(MainMenu::Exit),
                    _ => print_invalid_input(),
                }
            }
            Some(ref menu) => match menu {
                MainMenu::SinglyLinkedList => {
                    print_singly_list_menu();
                    let num = get_input();
                    match num {
                        1 => {
                            println!("Enter the element:");
                            let element = get_input();
                            singly_linked_list.add(element);
                        }
                        2 => {
                            singly_linked_list.remove();
                        }
                        3 => {
                            singly_linked_list.view_all_nodes();
                        }
                        4 => {
                            singly_linked_list = SinglyLinkedList::new();
                            menu_selected = None;
                        }
                        _ => print_invalid_input(),
                    }
                }
                MainMenu::DoublyLinkedList => {
                    print_doubly_list_menu();
                    let num = get_input();
                    match num {
                        1 => {
                            println!("Enter the element:");
                            let element = get_input();
                            doubly_linked_list.add_to_head(element);
                        }
                        2 => {
                            doubly_linked_list.remove_from_head();
                        }
                        3 => {
                            println!("Enter the element:");
                            let element = get_input();
                            doubly_linked_list.add_to_tail(element);
                        }
                        4 => {
                            doubly_linked_list.remove_from_tail();
                        }
                        5 => {
                            doubly_linked_list.view_all_nodes();
                        }
                        6 => {
                            doubly_linked_list = DoublyLinkedList::new();
                            menu_selected = None;
                        }
                        _ => print_invalid_input(),
                    }
                }
                MainMenu::Exit => {
                    break;
                }
            },
        }
    }
}

fn get_input() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");
    let input_num: i32 = input.trim().parse().expect("Invalid input, try again.");
    input_num
}

fn print_invalid_input() {
    println!("Invalid input, try again.")
}

fn print_main_menu() {
    println!(
        "Select linked list - 
        1. Singly Linked List
        2. Doubly Linked List
        3. Exit"
    );
}

fn print_singly_list_menu() {
    println!(
        "Chose your action - 
        1. Add item
        2. Remove item
        3. View all items
        4. Back"
    );
}

fn print_doubly_list_menu() {
    println!(
        "Chose your action - 
        1. Add item in front
        2. Remove item from front
        3. Add item in back
        4. Remove item from back
        5. View items
        6. Back"
    );
}
