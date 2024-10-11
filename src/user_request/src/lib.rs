use std::io::{self, Write};


// Define a structure for an item
#[derive(Clone)]
struct Item {
    name: String,
    price: f64,
}

impl Item {
    fn new(name: String, price: f64) -> Self {
        Self { name, price }
    }
}

// Define a structure for an order
#[derive(Clone)]
struct Order {
    user: String,
    items: Vec<Item>,
    accepted_by: Option<String>,
}

impl Order {
    fn new(user: String, items: Vec<Item>) -> Self {
        Self {
            user,
            items,
            accepted_by: None,
        }
    }
}

// Define a response type for accepting orders
struct AcceptOrderResponse {
    success: bool,
    message: String,
}

impl AcceptOrderResponse {
    fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

// List to store all orders
static mut ORDERS: Vec<Order> = Vec::new();

// Create an order
fn create_order(user: String, items: Vec<Item>) -> String {
    unsafe {
        ORDERS.push(Order::new(user, items));
    }
    "Order created successfully!".to_string()
}

// Accept an order
fn accept_order(order_index: usize, accepting_user: String) -> AcceptOrderResponse {
    unsafe {
        if order_index >= ORDERS.len() {
            return AcceptOrderResponse::new(false, "Invalid order index!".to_string());
        }

        let order = &mut ORDERS[order_index];

        if order.accepted_by.is_some() {
            return AcceptOrderResponse::new(
                false,
                "Order already accepted by another user!".to_string(),
            );
        }

        order.accepted_by = Some(accepting_user);
        AcceptOrderResponse::new(true, "Order accepted successfully!".to_string())
    }
}

// View all orders
fn get_orders() -> Vec<Order> {
    unsafe { ORDERS.clone() }
}

// Display the menu
fn display_menu() {
    println!("\n1. Create Order");
    println!("2. Accept Order");
    println!("3. View Orders");
    println!("4. Exit");
}

// Get user input
fn get_user_input() -> usize {
    print!("\nSelect an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap_or(0)
}

// Create order interaction
fn create_order_interaction() {
    let mut user = String::new();
    println!("\nEnter your username: ");
    io::stdin().read_line(&mut user).unwrap();
    let user = user.trim().to_string();

    let mut items = Vec::new();

    loop {
        let mut item_name = String::new();
        println!("Enter item name (or 'done' to finish): ");
        io::stdin().read_line(&mut item_name).unwrap();
        let item_name = item_name.trim().to_string();

        if item_name.to_lowercase() == "done" {
            break;
        }

        let mut item_price = String::new();
        println!("Enter item price: ");
        io::stdin().read_line(&mut item_price).unwrap();
        let item_price: f64 = item_price.trim().parse().unwrap();

        items.push(Item::new(item_name, item_price));
    }

    println!("{}", create_order(user, items));
}

// Accept order interaction
fn accept_order_interaction() {
    unsafe {
        if ORDERS.is_empty() {
            println!("No orders available to accept.");
            return;
        }
    }

    let mut user = String::new();
    println!("\nEnter your username: ");
    io::stdin().read_line(&mut user).unwrap();
    let user = user.trim().to_string();

    let mut order_index = String::new();
    println!("Enter the order index you want to accept: ");
    io::stdin().read_line(&mut order_index).unwrap();
    let order_index: usize = order_index.trim().parse().unwrap();

    let response = accept_order(order_index, user);
    println!("{}", response.message);
}

// View orders interaction
fn view_orders_interaction() {
    let orders = get_orders();

    if orders.is_empty() {
        println!("\nNo orders available.");
        return;
    }

    for (i, order) in orders.iter().enumerate() {
        println!("\nOrder {}:", i);
        println!("  Created by: {}", order.user);
        for item in &order.items {
            println!("  - {}: ${:.2}", item.name, item.price);
        }
        if let Some(ref accepted_by) = order.accepted_by {
            println!("  Accepted by: {}", accepted_by);
        } else {
            println!("  Not yet accepted");
        }
    }
}

fn main() {
    loop {
        display_menu();
        let option = get_user_input();

        match option {
            1 => create_order_interaction(),
            2 => accept_order_interaction(),
            3 => view_orders_interaction(),
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}






