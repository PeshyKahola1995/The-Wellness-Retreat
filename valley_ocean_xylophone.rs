// Main 
fn main() {
    println!("Welcome to The Wellness Retreat!");
}

// Structs
struct Guests {
    first_name: String,
    last_name: String,
    check_in_date: String,
    check_out_date: String,
    room_type: String,
    room_number: i32,
}

// Accommodations
struct Rooms {
    room_type: String,
    bed_type: String,
    room_size: String,
    max_occupancy: i8,
    rate_per_day: i32,
}

struct Amenities {
    spa_services: Vec<String>,
    pool_features: Vec<String>,
    room_amenities: Vec<String>,
}

// Services
struct Massages {
    type_of_massage: String,
    duration: i8,
    rate: i32,
}

struct Facials {
    type_of_facial: String,
    duration: i8,
    rate: i32,
}

struct Manicures {
    type_of_manicure: String,
    duration: i8,
    rate: i32,
}

struct Pedicures {
    type_of_pedicure: String,
    duration: i8,
    rate: i32,
}

struct Exercise_Classes {
    type_of_class: String,
    duration: i8,
    rate: i32,
}

// Food & Beverages
struct Restaurant {
    name: String,
    type_of_cuisine: String,
    menu_items: Vec<String>,
    rate_per_item: i32,
}

struct Bar {
    name:  String,
    menu_items: Vec<String>,
    rate_per_item: i32,
}

// Functions 
// Calculate total cost of stay
fn calculate_total_cost() -> i32 {

}

// Assign guests to rooms
fn assign_guests_to_rooms() {
    // code here
}

// Check in guests
fn check_in_guests() {
    // code here
}

// Check out guests
fn check_out_guests() {
    // code here
}

// Get available rooms
fn get_available_rooms() -> Vec<Rooms> {
    // code here
}

// Get guest list
fn get_guest_list() -> Vec<Guests> {
    // code here
}

// Get restaurant menu
fn get_restaurant_menu() {
    // code here
}

// Get bar menu
fn get_bar_menu() {
    // code here
}

// Make reservations
fn make_reservations() {
    // code here
}

// Book massages
fn book_massages() {
    // code here
}

// Book facials
fn book_facials() {
    // code here
}

// Book manicures
fn book_manicures() {
    // code here
}

// Book pedicures
fn book_pedicures() {
    // code here
}

// Book exercise classes
fn book_exercise_classes() {
    // code here
}