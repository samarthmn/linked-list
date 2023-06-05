use linked_list::singly::SinglyLinkedList;
use std::io::stdin;
enum MainMenu {
    SinglyLinkedList,
    DoublyLinkedList,
    CircularLinkedList,
    CircularDoublyLinkedList,
    Exit,
}

fn main() {
    let mut menu_selected: Option<MainMenu> = None;
    let mut singly_linked_list = SinglyLinkedList::new();
    loop {
        match menu_selected {
            None => {
                print_main_menu();
                let num = get_input();
                match num {
                    1 => menu_selected = Some(MainMenu::SinglyLinkedList),
                    2 => menu_selected = Some(MainMenu::DoublyLinkedList),
                    3 => menu_selected = Some(MainMenu::CircularLinkedList),
                    4 => menu_selected = Some(MainMenu::CircularDoublyLinkedList),
                    5 => menu_selected = Some(MainMenu::Exit),
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
                            let num = get_input();
                            singly_linked_list.add(num);
                        }
                        2 => {
                            singly_linked_list.remove();
                        }
                        3 => {
                            singly_linked_list.view_last_node();
                        }
                        4 => {
                            singly_linked_list.view_all_nodes();
                        }
                        5 => {
                            singly_linked_list = SinglyLinkedList::new();
                            menu_selected = None;
                        }
                        _ => print_invalid_input(),
                    }
                }
                MainMenu::DoublyLinkedList => {
                    println!("Not yet implemented");
                    menu_selected = None;
                }
                MainMenu::CircularLinkedList => {
                    println!("Not yet implemented");
                    menu_selected = None;
                }
                MainMenu::CircularDoublyLinkedList => {
                    println!("Not yet implemented");
                    menu_selected = None;
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
        2. Doubly Linked List (Not implemented)
        3. Circular Linked List(Not implemented)
        4. Circular Doubly Linked List(Not implemented)
        5. Exit"
    );
}

fn print_singly_list_menu() {
    println!(
        "Chose your action - 
        1. Add item
        2. Delete item
        3. View last item
        4. View all items
        5. Back"
    );
}
