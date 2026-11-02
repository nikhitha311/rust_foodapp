# Food Ordering System (Rust)

This is a desktop-based Food Ordering application built entirely using Rust. It has a graphical user interface (GUI) and works similarly to apps like Zomato or Swiggy. I built this for my Rust course project to practice core concepts like structs, enums, and error handling. 

 ## Features
- **Login system:** Takes user name, phone number, and address with basic validation.
- **Menu:** Displays 22 dishes separated into Starters, Main Course, Desserts, and Drinks.
- **Cart:** Users can add items, change quantities, and remove them. It also calculates GST (5%) and delivery fees in Rupees (₹).
- **Payment:** Options for Cash on Delivery, UPI, and Card.
- **Live Tracking:** The order status updates automatically (Preparing -> Out for delivery -> Arriving -> Delivered). I set the timer to 1 minute total just for demo purposes.
- **Notifications:** A bell icon that shows real-time updates when the order status changes.
- **Profile & History:** Users can edit their details and view past orders.

## Libraries Used
- `eframe` / `egui` - For building the desktop GUI.
- `chrono` - For getting the current date and time for order history.

## Rust Concepts Used
- **Structs & Enums:** Used for modeling the data (e.g., `MenuItem`, `OrderStatus`, `PaymentMethod`).
- **Pattern Matching:** Used `match` extensively for routing the UI, handling payment methods, and updating the live tracking stages.
- **Error Handling:** Used `Option` and `Result` for input validation and checking item stock.
- **Borrow Checker:** Handled multiple borrowing issues while passing state to the immediate-mode GUI loops.

## How to Run

1. Make sure Rust and Cargo are installed on your system.
2. Clone the repository:
   ```bash
   git clone https://github.com/nikhitha311/rust_foodapp.git
