use eframe::egui::{self, Color32, RichText, Frame, Rounding, Stroke, Margin, Vec2};
use chrono::Local;
use std::collections::VecDeque;

// ==================== THEME ====================
const BRAND_PRIMARY:  Color32 = Color32::from_rgb(59, 130, 246);
const BG_DARK:        Color32 = Color32::from_rgb(15, 23, 42);
const BG_PANEL:       Color32 = Color32::from_rgb(30, 41, 59);
const BG_CARD:        Color32 = Color32::from_rgb(30, 41, 59);
const BG_HOVER:       Color32 = Color32::from_rgb(51, 65, 85);
const ACCENT_GREEN:   Color32 = Color32::from_rgb(34, 197, 94);
const ACCENT_ORANGE:  Color32 = Color32::from_rgb(251, 146, 60);
const ACCENT_RED:     Color32 = Color32::from_rgb(239, 68, 68);
const ACCENT_YELLOW:  Color32 = Color32::from_rgb(250, 204, 21);
const ACCENT_PURPLE:  Color32 = Color32::from_rgb(168, 85, 247);
const TEXT_PRIMARY:   Color32 = Color32::from_rgb(241, 245, 249);
const TEXT_DIM:       Color32 = Color32::from_rgb(148, 163, 184);
const TEXT_MUTED:     Color32 = Color32::from_rgb(100, 116, 139);

// ==================== DELIVERY TIMING (5:30) ====================
const STAGE_1_TIME: f64 = 5.0;
const STAGE_2_TIME: f64 = 10.0;
const STAGE_3_TIME: f64 = 22.0;
const STAGE_4_TIME: f64 = 30.0;

// ==================== HELPERS ====================
fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgb(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
    )
}

fn primary_button(ui: &mut egui::Ui, text: &str, fill: Color32) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(14.0).strong().color(Color32::WHITE))
            .fill(fill).rounding(Rounding::same(8.0))
            .min_size(Vec2::new(0.0, 38.0)),
    )
}

// ==================== ENUMS ====================
#[derive(Debug, Clone, PartialEq)]
enum Category { Appetizer, MainCourse, Dessert, Beverage }

impl Category {
    fn emoji(&self) -> &str {
        match self {
            Category::Appetizer => "🍟",
            Category::MainCourse => "🍛",
            Category::Dessert => "🍰",
            Category::Beverage => "☕",
        }
    }
    fn name(&self) -> &str {
        match self {
            Category::Appetizer => "Starters",
            Category::MainCourse => "Main Course",
            Category::Dessert => "Desserts",
            Category::Beverage => "Drinks",
        }
    }
    fn all() -> Vec<Category> {
        vec![Category::Appetizer, Category::MainCourse, Category::Dessert, Category::Beverage]
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Pending, Preparing, OutForDelivery, Arriving, Delivered }

impl OrderStatus {
    fn label(&self) -> &str {
        match self {
            OrderStatus::Pending => "Order Placed",
            OrderStatus::Preparing => "Preparing",
            OrderStatus::OutForDelivery => "On the Way",
            OrderStatus::Arriving => "Arriving Soon",
            OrderStatus::Delivered => "Delivered",
        }
    }
    fn emoji(&self) -> &str {
        match self {
            OrderStatus::Pending => "📝",
            OrderStatus::Preparing => "👨‍🍳",
            OrderStatus::OutForDelivery => "🛵",
            OrderStatus::Arriving => "🏠",
            OrderStatus::Delivered => "✅",
        }
    }
    fn index(&self) -> usize {
        match self {
            OrderStatus::Pending => 0,
            OrderStatus::Preparing => 1,
            OrderStatus::OutForDelivery => 2,
            OrderStatus::Arriving => 3,
            OrderStatus::Delivered => 4,
        }
    }
}

// ==================== NOTIFICATION ====================
#[derive(Debug, Clone)]
struct Notification {
    title: String,
    message: String,
    emoji: String,
    color: Color32,
    time: String,
    age: f64,
}

// ==================== STRUCTS ====================
#[derive(Debug, Clone)]
struct MenuItem {
    name: String,
    price: f64,
    original_price: f64,
    category: Category,
    stock: u32,
    rating: f32,
    is_veg: bool,
    description: String,
}

#[derive(Debug, Clone)]
struct CartItem {
    name: String,
    quantity: u32,
    unit_price: f64,
}

impl CartItem {
    fn subtotal(&self) -> f64 { self.quantity as f64 * self.unit_price }
}

#[derive(Debug, Clone)]
struct OrderRecord {
    id: u32,
    customer_name: String,
    address: String,
    items: Vec<CartItem>,
    payment: String,
    total: f64,
    timestamp: String,
    status: OrderStatus,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    phone: String,
    address: String,
}

// ==================== APP STATE ====================
struct FoodApp {
    logged_in: bool,
    user: Option<User>,
    login_name: String,
    login_phone: String,
    login_address: String,
    login_error: String,

    show_profile_edit: bool,
    edit_name: String,
    edit_phone: String,
    edit_address: String,
    edit_error: String,

    menu: Vec<MenuItem>,
    cart: Vec<CartItem>,
    history: Vec<OrderRecord>,
    payment_method: String,
    search_query: String,
    category_filter: Option<Category>,

    order_placed: bool,
    current_status: OrderStatus,
    current_order_total: f64,
    elapsed_time: f64,

    error_msg: String,
    next_order_id: u32,
    show_history: bool,
    show_cart: bool,
    show_notifications: bool,
    toast: Option<(String, f64)>,
    notifications: VecDeque<Notification>,
    unread_count: u32,
}

impl Default for FoodApp {
    fn default() -> Self {
        Self {
            logged_in: false,
            user: None,
            login_name: String::new(),
            login_phone: String::new(),
            login_address: String::new(),
            login_error: String::new(),
            show_profile_edit: false,
            edit_name: String::new(),
            edit_phone: String::new(),
            edit_address: String::new(),
            edit_error: String::new(),
            menu: vec![
                // ===== STARTERS =====
                MenuItem { name: "Spring Rolls".into(),      price: 199.0,  original_price: 279.0,  category: Category::Appetizer,  stock: 20, rating: 4.3, is_veg: true,  description: "Crispy veggie rolls with sweet chili dip".into() },
                MenuItem { name: "Paneer Tikka".into(),      price: 349.0,  original_price: 449.0,  category: Category::Appetizer,  stock: 15, rating: 4.7, is_veg: true,  description: "Char-grilled cottage cheese with spices".into() },
                MenuItem { name: "Chicken Wings".into(),     price: 399.0,  original_price: 499.0,  category: Category::Appetizer,  stock: 18, rating: 4.6, is_veg: false, description: "Spicy buffalo wings with ranch dip".into() },
                MenuItem { name: "Garlic Bread".into(),      price: 149.0,  original_price: 199.0,  category: Category::Appetizer,  stock: 22, rating: 4.4, is_veg: true,  description: "Toasted bread with garlic butter".into() },
                MenuItem { name: "Onion Rings".into(),       price: 179.0,  original_price: 249.0,  category: Category::Appetizer,  stock: 16, rating: 4.2, is_veg: true,  description: "Golden fried onion rings with mayo".into() },
                // ===== MAIN COURSE =====
                MenuItem { name: "Butter Chicken".into(),    price: 499.0,  original_price: 649.0,  category: Category::MainCourse, stock: 10, rating: 4.8, is_veg: false, description: "Creamy tomato chicken curry".into() },
                MenuItem { name: "Veg Biryani".into(),       price: 299.0,  original_price: 399.0,  category: Category::MainCourse, stock: 12, rating: 4.5, is_veg: true,  description: "Aromatic basmati rice with veggies".into() },
                MenuItem { name: "Margherita Pizza".into(),  price: 399.0,  original_price: 549.0,  category: Category::MainCourse, stock: 8,  rating: 4.6, is_veg: true,  description: "Mozzarella, tomato, basil".into() },
                MenuItem { name: "Chicken Biryani".into(),   price: 449.0,  original_price: 599.0,  category: Category::MainCourse, stock: 14, rating: 4.9, is_veg: false, description: "Hyderabadi dum biryani with raita".into() },
                MenuItem { name: "Paneer Butter Masala".into(), price: 349.0, original_price: 449.0, category: Category::MainCourse, stock: 11, rating: 4.7, is_veg: true, description: "Cottage cheese in rich tomato gravy".into() },
                MenuItem { name: "Beef Burger".into(),       price: 299.0,  original_price: 399.0,  category: Category::MainCourse, stock: 13, rating: 4.5, is_veg: false, description: "Juicy beef patty with cheese & lettuce".into() },
                MenuItem { name: "Veg Fried Rice".into(),    price: 249.0,  original_price: 349.0,  category: Category::MainCourse, stock: 15, rating: 4.3, is_veg: true,  description: "Wok-tossed rice with fresh vegetables".into() },
                // ===== DESSERTS =====
                MenuItem { name: "Gulab Jamun".into(),       price: 99.0,   original_price: 149.0,  category: Category::Dessert, stock: 25, rating: 4.9, is_veg: true, description: "Soft milk dumplings in rose syrup".into() },
                MenuItem { name: "Ice Cream".into(),         price: 79.0,   original_price: 119.0,  category: Category::Dessert, stock: 30, rating: 4.4, is_veg: true, description: "Vanilla scoop with chocolate drizzle".into() },
                MenuItem { name: "Chocolate Brownie".into(), price: 149.0,  original_price: 199.0,  category: Category::Dessert, stock: 20, rating: 4.8, is_veg: true, description: "Warm fudge brownie with scoop".into() },
                MenuItem { name: "Cheesecake".into(),        price: 199.0,  original_price: 279.0,  category: Category::Dessert, stock: 15, rating: 4.7, is_veg: true, description: "New York style creamy cheesecake".into() },
                MenuItem { name: "Rasmalai".into(),          price: 129.0,  original_price: 179.0,  category: Category::Dessert, stock: 18, rating: 4.6, is_veg: true, description: "Soft paneer discs in saffron milk".into() },
                // ===== BEVERAGES =====
                MenuItem { name: "Masala Chai".into(),       price: 49.0,   original_price: 79.0,   category: Category::Beverage, stock: 50, rating: 4.7, is_veg: true, description: "Spiced Indian tea with milk".into() },
                MenuItem { name: "Cold Coffee".into(),       price: 129.0,  original_price: 169.0,  category: Category::Beverage, stock: 20, rating: 4.5, is_veg: true, description: "Chilled creamy coffee shake".into() },
                MenuItem { name: "Mango Lassi".into(),       price: 99.0,   original_price: 149.0,  category: Category::Beverage, stock: 22, rating: 4.8, is_veg: true, description: "Sweet yogurt drink with mango".into() },
                MenuItem { name: "Fresh Lime Soda".into(),   price: 79.0,   original_price: 109.0,  category: Category::Beverage, stock: 40, rating: 4.4, is_veg: true, description: "Refreshing lime with soda & mint".into() },
                MenuItem { name: "Orange Juice".into(),      price: 99.0,   original_price: 149.0,  category: Category::Beverage, stock: 25, rating: 4.5, is_veg: true, description: "Freshly squeezed orange juice".into() },
            ],
            cart: Vec::new(),
            history: Vec::new(),
            payment_method: "Cash on Delivery".to_string(),
            search_query: String::new(),
            category_filter: None,
            order_placed: false,
            current_status: OrderStatus::Pending,
            current_order_total: 0.0,
            elapsed_time: 0.0,
            error_msg: String::new(),
            next_order_id: 1001,
            show_history: false,
            show_cart: false,
            show_notifications: false,
            toast: None,
            notifications: VecDeque::new(),
            unread_count: 0,
        }
    }
}

impl FoodApp {
    fn total(&self) -> f64 { self.cart.iter().map(|i| i.subtotal()).sum() }
    fn item_count(&self) -> u32 { self.cart.iter().map(|c| c.quantity).sum() }
    fn savings(&self) -> f64 {
        self.cart.iter().map(|c| {
            if let Some(m) = self.menu.iter().find(|m| m.name == c.name) {
                (m.original_price - m.price) * c.quantity as f64
            } else { 0.0 }
        }).sum()
    }

    fn toast(&mut self, msg: &str) {
        self.toast = Some((msg.to_string(), 2.5));
    }

    fn push_notification(&mut self, title: &str, message: &str, emoji: &str, color: Color32) {
        self.notifications.push_front(Notification {
            title: title.to_string(),
            message: message.to_string(),
            emoji: emoji.to_string(),
            color,
            time: Local::now().format("%I:%M %p").to_string(),
            age: 0.0,
        });
        if self.notifications.len() > 20 { self.notifications.pop_back(); }
        self.unread_count += 1;
    }

    fn try_login(&mut self) {
        let name = self.login_name.trim();
        let phone = self.login_phone.trim();
        let address = self.login_address.trim();

        if name.len() < 2 {
            self.login_error = "Please enter your full name".to_string();
            return;
        }
        if phone.len() < 10 {
            self.login_error = "Please enter a valid 10-digit phone number".to_string();
            return;
        }
        if address.len() < 5 {
            self.login_error = "Please enter a valid delivery address".to_string();
            return;
        }

        self.user = Some(User {
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
        });
        self.logged_in = true;
        self.login_error.clear();
        let welcome_msg = format!("Hello {}, ready to order?", name);
        self.push_notification("Welcome!", &welcome_msg, "👋", ACCENT_PURPLE);
    }

    fn logout(&mut self) {
        *self = FoodApp::default();
    }

    fn open_profile_editor(&mut self) {
        if let Some(user) = &self.user {
            self.edit_name = user.name.clone();
            self.edit_phone = user.phone.clone();
            self.edit_address = user.address.clone();
            self.edit_error.clear();
            self.show_profile_edit = true;
        }
    }

    fn save_profile(&mut self) {
        let name = self.edit_name.trim();
        let phone = self.edit_phone.trim();
        let address = self.edit_address.trim();

        if name.len() < 2 {
            self.edit_error = "Name must be at least 2 characters".to_string();
            return;
        }
        if phone.len() < 10 {
            self.edit_error = "Phone must be 10 digits".to_string();
            return;
        }
        if address.len() < 5 {
            self.edit_error = "Address must be at least 5 characters".to_string();
            return;
        }

        let old_name = self.user.as_ref().map(|u| u.name.clone()).unwrap_or_default();

        self.user = Some(User {
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
        });

        self.show_profile_edit = false;
        self.edit_error.clear();

        for order in self.history.iter_mut() {
            if order.customer_name == old_name {
                order.customer_name = name.to_string();
                order.address = address.to_string();
            }
        }

        self.toast("✅ Profile updated!");
        self.push_notification(
            "Profile Updated",
            "Your details have been saved successfully.",
            "✏️",
            BRAND_PRIMARY,
        );
    }

    fn add_to_cart(&mut self, idx: usize) {
        let item = &self.menu[idx];
        if item.stock == 0 {
            self.error_msg = format!("'{}' is out of stock!", item.name);
            self.toast(&format!("❌ {} out of stock", item.name));
            return;
        }
        let name = item.name.clone();
        let price = item.price;

        if let Some(existing) = self.cart.iter_mut().find(|c| c.name == name) {
            existing.quantity += 1;
        } else {
            self.cart.push(CartItem { name: name.clone(), quantity: 1, unit_price: price });
        }
        if let Some(m) = self.menu.iter_mut().find(|m| m.name == name) { m.stock -= 1; }
        self.error_msg.clear();
        self.toast(&format!("✅ {} added", name));
    }

    fn remove_from_cart(&mut self, idx: usize) {
        let removed = self.cart.remove(idx);
        if let Some(m) = self.menu.iter_mut().find(|m| m.name == removed.name) {
            m.stock += removed.quantity;
        }
    }

    fn change_quantity(&mut self, idx: usize, delta: i32) {
        if idx >= self.cart.len() { return; }
        let name = self.cart[idx].name.clone();

        if delta > 0 {
            if let Some(m) = self.menu.iter_mut().find(|m| m.name == name) {
                if m.stock > 0 { m.stock -= 1; self.cart[idx].quantity += 1; }
                else { self.error_msg = format!("No more stock for '{}'", name); }
            }
        } else if self.cart[idx].quantity > 1 {
            self.cart[idx].quantity -= 1;
            if let Some(m) = self.menu.iter_mut().find(|m| m.name == name) { m.stock += 1; }
        } else {
            self.remove_from_cart(idx);
        }
    }

    fn place_order(&mut self) {
        if self.cart.is_empty() { self.error_msg = "Cart is empty!".to_string(); return; }

        let user = match &self.user {
            Some(u) => u.clone(),
            None => return,
        };

        let record = OrderRecord {
            id: self.next_order_id,
            customer_name: user.name.clone(),
            address: user.address.clone(),
            items: self.cart.clone(),
            payment: self.payment_method.clone(),
            total: self.total(),
            timestamp: Local::now().format("%d %b, %I:%M %p").to_string(),
            status: OrderStatus::Pending,
        };
        self.next_order_id += 1;
        self.history.push(record);
        self.current_order_total = self.total();
        self.order_placed = true;
        self.current_status = OrderStatus::Pending;
        self.elapsed_time = 0.0;
        self.show_cart = false;
        self.error_msg.clear();
        self.toast("🎉 Order placed!");

        let order_id = self.next_order_id - 1;
        let confirm_msg = format!("Order #{} received. Preparing now!", order_id);
        self.push_notification("Order Confirmed", &confirm_msg, "📝", BRAND_PRIMARY);
    }

    fn update_delivery_status(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.stable_dt as f64).min(0.1);
        for n in self.notifications.iter_mut() {
            n.age += dt;
        }

        if !self.order_placed || self.current_status == OrderStatus::Delivered { return; }

        self.elapsed_time += dt;

        let new_status = if self.elapsed_time >= STAGE_4_TIME {
            OrderStatus::Delivered
        } else if self.elapsed_time >= STAGE_3_TIME {
            OrderStatus::Arriving
        } else if self.elapsed_time >= STAGE_2_TIME {
            OrderStatus::OutForDelivery
        } else if self.elapsed_time >= STAGE_1_TIME {
            OrderStatus::Preparing
        } else {
            OrderStatus::Pending
        };

        if new_status != self.current_status {
            self.current_status = new_status.clone();
            if let Some(last) = self.history.last_mut() {
                last.status = new_status.clone();
            }

            let (title, msg, emoji, color): (String, String, &str, Color32) = match &new_status {
                OrderStatus::Preparing => (
                    "Preparing".to_string(),
                    "Chef started cooking your delicious food!".to_string(),
                    "🍳",
                    ACCENT_ORANGE,
                ),
                OrderStatus::OutForDelivery => (
                    "Out for Delivery".to_string(),
                    "Ravi picked up your order and is on the way!".to_string(),
                    "🛵",
                    BRAND_PRIMARY,
                ),
                OrderStatus::Arriving => (
                    "Arriving Soon".to_string(),
                    "Ravi is 2 minutes away from your location!".to_string(),
                    "📍",
                    ACCENT_YELLOW,
                ),
                OrderStatus::Delivered => (
                    "Delivered!".to_string(),
                    "Your order has been delivered. Enjoy your meal! 🎉".to_string(),
                    "🎉",
                    ACCENT_GREEN,
                ),
                _ => (String::new(), String::new(), "", BRAND_PRIMARY),
            };

            if !title.is_empty() {
                self.toast(&format!("{} {}", emoji, title));
                self.push_notification(&title, &msg, emoji, color);
            }
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }

    fn new_order(&mut self) {
        self.cart.clear();
        self.order_placed = false;
        self.current_status = OrderStatus::Pending;
        self.elapsed_time = 0.0;
        self.error_msg.clear();
    }

    fn format_time(&self) -> String {
        let s = self.elapsed_time as u32;
        format!("{:02}:{:02}", s / 60, s % 60)
    }

    fn remaining_time(&self) -> String {
        if self.current_status == OrderStatus::Delivered { return "Delivered!".to_string(); }
        let rem = (STAGE_4_TIME - self.elapsed_time).max(0.0) as u32;
        format!("{}:{:02} remaining", rem / 60, rem % 60)
    }
}

// ==================== LOGIN PAGE ====================
fn render_login(app: &mut FoodApp, ctx: &egui::Context) {
    let time = ctx.input(|i| i.time) as f32;

    egui::CentralPanel::default()
        .frame(Frame::none().fill(BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                let pulse = ((time * 2.0).sin() * 0.5 + 0.5) as f32;
                let logo_size = 70.0 + pulse * 10.0;
                ui.label(RichText::new("🍕").size(logo_size));

                ui.add_space(8.0);
                ui.label(RichText::new("FoodExpress").size(34.0).strong().color(BRAND_PRIMARY));
                ui.label(RichText::new("Fresh • Fast • Delicious").size(12.0).color(TEXT_DIM));
                ui.add_space(30.0);

                let card_width = 450.0;

                ui.allocate_ui_with_layout(
                    Vec2::new(card_width, 0.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        Frame::none()
                            .fill(BG_PANEL)
                            .rounding(Rounding::same(16.0))
                            .stroke(Stroke::new(1.5_f32, BRAND_PRIMARY.linear_multiply(0.4)))
                            .inner_margin(Margin::same(30.0))
                            .show(ui, |ui| {
                                ui.set_width(card_width - 60.0);

                                ui.label(RichText::new("👋 Welcome Back").size(22.0).strong().color(TEXT_PRIMARY));
                                ui.label(RichText::new("Sign in to start ordering").size(11.0).color(TEXT_DIM));
                                ui.add_space(20.0);

                                ui.label(RichText::new("👤  Full Name").size(12.0).strong().color(TEXT_DIM));
                                ui.add_space(4.0);
                                ui.add(egui::TextEdit::singleline(&mut app.login_name)
                                    .hint_text("e.g., Nikhitha Reddy")
                                    .desired_width(f32::INFINITY)
                                    .font(egui::FontId::proportional(14.0)));
                                ui.add_space(12.0);

                                ui.label(RichText::new("📱  Phone Number").size(12.0).strong().color(TEXT_DIM));
                                ui.add_space(4.0);
                                ui.add(egui::TextEdit::singleline(&mut app.login_phone)
                                    .hint_text("10-digit mobile number")
                                    .desired_width(f32::INFINITY)
                                    .font(egui::FontId::proportional(14.0)));
                                ui.add_space(12.0);

                                ui.label(RichText::new("📍  Delivery Address").size(12.0).strong().color(TEXT_DIM));
                                ui.add_space(4.0);
                                ui.add(egui::TextEdit::multiline(&mut app.login_address)
                                    .hint_text("House no, street, area, city")
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(2)
                                    .font(egui::FontId::proportional(14.0)));

                                if !app.login_error.is_empty() {
                                    ui.add_space(12.0);
                                    Frame::none().fill(ACCENT_RED.linear_multiply(0.15))
                                        .stroke(Stroke::new(1.0_f32, ACCENT_RED))
                                        .rounding(Rounding::same(8.0))
                                        .inner_margin(Margin::same(10.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new("⚠️").size(16.0));
                                                ui.label(RichText::new(&app.login_error).size(12.0).color(ACCENT_RED));
                                            });
                                        });
                                }

                                ui.add_space(20.0);

                                let btn = egui::Button::new(
                                    RichText::new("🚀   Sign In & Start Ordering").size(15.0).strong().color(Color32::WHITE),
                                ).fill(BRAND_PRIMARY).rounding(Rounding::same(10.0))
                                 .min_size(Vec2::new(ui.available_width(), 48.0));

                                if ui.add(btn).clicked() {
                                    app.try_login();
                                }

                                ui.add_space(10.0);
                                ui.label(RichText::new("📍 Delivery available in Shamshabad, India").size(10.0).color(TEXT_MUTED));
                            });
                    },
                );
            });
        });
}

// ==================== LIVE TRACKING ====================
fn render_live_tracking(app: &FoodApp, ui: &mut egui::Ui) {
    let order_id = app.history.last().map(|o| o.id).unwrap_or(0);
    let progress = (app.elapsed_time / STAGE_4_TIME).min(1.0) as f32;
    let time = ui.input(|i| i.time) as f32;

    ui.horizontal(|ui| {
        ui.label(RichText::new("🎯").size(24.0));
        ui.vertical(|ui| {
            if app.current_status == OrderStatus::Delivered {
                let bounce = ((time * 4.0).sin() * 0.5 + 0.5) as f32;
                let c = lerp_color(ACCENT_GREEN, Color32::WHITE, bounce * 0.3);
                ui.label(RichText::new("Delivered! 🎉").size(16.0).strong().color(c));
            } else {
                let pulse = (time * 2.0).sin() * 0.5 + 0.5;
                let c = lerp_color(BRAND_PRIMARY, ACCENT_GREEN, pulse);
                ui.label(RichText::new("● LIVE TRACKING").size(14.0).strong().color(c));
            }
            ui.label(RichText::new(format!("Order #{}", order_id)).size(11.0).color(TEXT_DIM));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            Frame::none().fill(BG_HOVER).rounding(Rounding::same(6.0))
                .inner_margin(Margin::symmetric(8.0, 4.0))
                .show(ui, |ui| {
                    ui.label(RichText::new(format!("⏱️ {}", app.format_time())).size(11.0).strong().color(TEXT_PRIMARY));
                });
        });
    });

    ui.add_space(10.0);

    let (bar_color, bar_label): (Color32, &str) = match app.current_status {
        OrderStatus::Pending => (BRAND_PRIMARY, "📝 Confirming order..."),
        OrderStatus::Preparing => (ACCENT_ORANGE, "👨‍🍳 Chef is cooking..."),
        OrderStatus::OutForDelivery => (BRAND_PRIMARY, "🛵 On the way to you..."),
        OrderStatus::Arriving => (ACCENT_YELLOW, "🏠 Arriving in 2 mins..."),
        OrderStatus::Delivered => (ACCENT_GREEN, "✅ Delivered! Enjoy 🎉"),
    };

    ui.add(
        egui::ProgressBar::new(progress)
            .text(RichText::new(bar_label).size(13.0).strong().color(Color32::WHITE))
            .animate(true)
            .fill(bar_color)
            .desired_height(30.0),
    );

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("📍 Shamshabad, India → You").size(10.0).color(TEXT_DIM));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(app.remaining_time()).size(10.0).strong().color(ACCENT_GREEN));
        });
    });

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(12.0);

    let stages = [
        (OrderStatus::Pending,        "Order Confirmed", "Restaurant received your order"),
        (OrderStatus::Preparing,      "Preparing",       "Chef is cooking your food"),
        (OrderStatus::OutForDelivery, "On the Way",      "Delivery partner picked up"),
        (OrderStatus::Arriving,       "Arriving Soon",   "Ravi is 2 minutes away"),
        (OrderStatus::Delivered,      "Delivered",       "Enjoy your meal!"),
    ];

    let current_idx = app.current_status.index();

    for (i, (stage, title, subtitle)) in stages.iter().enumerate() {
        let is_done = i <= current_idx;
        let is_current = i == current_idx && app.current_status != OrderStatus::Delivered;

        ui.horizontal(|ui| {
            let dot_color = if is_done { ACCENT_GREEN } else { BG_HOVER };
            let dot_radius = if is_current { 9.0 } else if is_done { 7.0 } else { 6.0 };

            let (rect, _) = ui.allocate_exact_size(Vec2::new(22.0, 22.0), egui::Sense::hover());
            let center = rect.center();

            if is_current {
                let pulse = ((time * 3.0).sin() * 0.5 + 0.5) as f32;
                ui.painter().circle_stroke(
                    center,
                    dot_radius + 3.0 + pulse * 5.0,
                    Stroke::new(2.0_f32, ACCENT_GREEN.linear_multiply(0.6 - pulse * 0.4)),
                );
            }

            ui.painter().circle_filled(center, dot_radius, dot_color);

            if is_done && !is_current {
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    "✓",
                    egui::FontId::proportional(10.0),
                    Color32::WHITE,
                );
            }

            ui.add_space(10.0);

            ui.vertical(|ui| {
                let title_color = if is_done { TEXT_PRIMARY } else { TEXT_DIM };
                let mut t = RichText::new(format!("{} {}", stage.emoji(), title))
                    .size(13.0).color(title_color);
                if is_current {
                    let pulse = (time * 3.0).sin() * 0.5 + 0.5;
                    let c = lerp_color(BRAND_PRIMARY, ACCENT_GREEN, pulse);
                    t = t.strong().color(c);
                }
                ui.label(t);

                let sub_color = if is_done { TEXT_DIM } else { TEXT_MUTED };
                ui.label(RichText::new(*subtitle).size(10.0).color(sub_color));
            });

            if is_current {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let pulse = (time * 4.0).sin() * 0.5 + 0.5;
                    let c = ACCENT_GREEN.linear_multiply(0.5 + pulse * 0.5);
                    ui.label(RichText::new("● LIVE").size(10.0).strong().color(c));
                });
            }
        });

        if i < stages.len() - 1 {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                let line_color = if i < current_idx { ACCENT_GREEN } else { BG_HOVER };
                let (rect, _) = ui.allocate_exact_size(Vec2::new(2.0, 20.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 1.0, line_color);
            });
        }
    }

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(10.0);

    if app.current_status == OrderStatus::OutForDelivery || app.current_status == OrderStatus::Arriving {
        Frame::none().fill(ACCENT_ORANGE.linear_multiply(0.15))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::same(12.0))
            .stroke(Stroke::new(1.0_f32, ACCENT_ORANGE.linear_multiply(0.5)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let offset = ((time * 3.0).sin() * 3.0) as f32;
                    ui.add_space(offset.abs());
                    ui.label(RichText::new("🛵").size(28.0));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Ravi Kumar").size(13.0).strong().color(TEXT_PRIMARY));
                        if app.current_status == OrderStatus::Arriving {
                            ui.label(RichText::new("Just arrived at your location!").size(10.0).color(ACCENT_YELLOW));
                        } else {
                            ui.label(RichText::new("On the way to your address").size(10.0).color(TEXT_DIM));
                        }
                        ui.label(RichText::new("⭐ 4.9  •  🛵 12 deliveries today").size(9.0).color(TEXT_MUTED));
                    });
                });
            });
        ui.add_space(10.0);
    }

    if app.current_status == OrderStatus::Delivered {
        Frame::none().fill(ACCENT_GREEN.linear_multiply(0.15))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::same(14.0))
            .stroke(Stroke::new(2.0_f32, ACCENT_GREEN))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let bounce = (time * 5.0).sin() * 0.5 + 0.5;
                    ui.label(RichText::new("✅").size(32.0 + bounce * 6.0));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Order Delivered!").size(15.0).strong().color(ACCENT_GREEN));
                        ui.label(RichText::new("Enjoy your delicious meal 🎉").size(11.0).color(TEXT_DIM));
                    });
                });
            });
        ui.add_space(10.0);
    }

    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("💰 Total Paid: ₹{:.2}", app.current_order_total))
            .size(13.0).strong().color(ACCENT_GREEN));
    });
}

// ==================== NOTIFICATIONS PANEL ====================
fn render_notifications(app: &mut FoodApp, ctx: &egui::Context) {
    egui::Window::new("🔔 Notifications")
        .open(&mut app.show_notifications)
        .resizable(true)
        .default_size([380.0, 500.0])
        .frame(Frame::window(&ctx.style()).fill(BG_PANEL))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{} notifications", app.notifications.len()))
                    .size(12.0).color(TEXT_DIM));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Clear all").clicked() {
                        app.notifications.clear();
                        app.unread_count = 0;
                    }
                });
            });
            ui.separator();
            ui.add_space(5.0);

            if app.notifications.is_empty() {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("🔕").size(48.0));
                    ui.add_space(8.0);
                    ui.label(RichText::new("No notifications").size(14.0).color(TEXT_DIM));
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let notifications_clone: Vec<Notification> = app.notifications.iter().cloned().collect();
                    for n in notifications_clone.iter() {
                        let fade = if n.age < 1.0 { n.age as f32 } else { 1.0 };
                        Frame::none()
                            .fill(BG_CARD)
                            .stroke(Stroke::new(1.0_f32, n.color.linear_multiply(fade * 0.6)))
                            .rounding(Rounding::same(10.0))
                            .inner_margin(Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    Frame::none()
                                        .fill(n.color.linear_multiply(0.2))
                                        .rounding(Rounding::same(20.0))
                                        .inner_margin(Margin::same(10.0))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(&n.emoji).size(20.0));
                                        });

                                    ui.add_space(8.0);

                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&n.title).size(13.0).strong().color(TEXT_PRIMARY));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(&n.time).size(9.0).color(TEXT_MUTED));
                                            });
                                        });
                                        ui.label(RichText::new(&n.message).size(11.0).color(TEXT_DIM));
                                    });
                                });
                            });
                        ui.add_space(6.0);
                    }
                });
            }
        });
}

// ==================== PROFILE EDIT WINDOW ====================
fn render_profile_edit(app: &mut FoodApp, ctx: &egui::Context) {
    let mut open = app.show_profile_edit;
    let mut should_save = false;
    let mut should_cancel = false;

    egui::Window::new("✏️ Edit Profile")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size([440.0, 0.0])
        .anchor(egui::Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
        .frame(Frame::window(&ctx.style()).fill(BG_PANEL))
        .show(ctx, |ui| {
            ui.label(RichText::new("Update your account details")
                .size(12.0).color(TEXT_DIM));
            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                Frame::none()
                    .fill(BRAND_PRIMARY.linear_multiply(0.2))
                    .stroke(Stroke::new(2.0_f32, BRAND_PRIMARY))
                    .rounding(Rounding::same(50.0))
                    .inner_margin(Margin::same(18.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new("👤").size(40.0));
                    });
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label(RichText::new("👤  Full Name").size(12.0).strong().color(TEXT_DIM));
            ui.add_space(4.0);
            ui.add(egui::TextEdit::singleline(&mut app.edit_name)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(14.0)));

            ui.add_space(12.0);

            ui.label(RichText::new("📱  Phone Number").size(12.0).strong().color(TEXT_DIM));
            ui.add_space(4.0);
            ui.add(egui::TextEdit::singleline(&mut app.edit_phone)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(14.0)));

            ui.add_space(12.0);

            ui.label(RichText::new("📍  Delivery Address").size(12.0).strong().color(TEXT_DIM));
            ui.add_space(4.0);
            ui.add(egui::TextEdit::multiline(&mut app.edit_address)
                .desired_width(f32::INFINITY)
                .desired_rows(2)
                .font(egui::FontId::proportional(14.0)));

            if !app.edit_error.is_empty() {
                ui.add_space(10.0);
                Frame::none()
                    .fill(ACCENT_RED.linear_multiply(0.15))
                    .stroke(Stroke::new(1.0_f32, ACCENT_RED))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("⚠️ {}", app.edit_error))
                            .size(12.0).color(ACCENT_RED));
                    });
            }

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                let save_btn = egui::Button::new(
                    RichText::new("💾  Save Changes").size(14.0).strong().color(Color32::WHITE)
                ).fill(ACCENT_GREEN).rounding(Rounding::same(10.0))
                 .min_size(Vec2::new(190.0, 42.0));

                if ui.add(save_btn).clicked() {
                    should_save = true;
                }

                ui.add_space(8.0);

                let cancel_btn = egui::Button::new(
                    RichText::new("✖  Cancel").size(14.0).color(TEXT_PRIMARY)
                ).fill(BG_HOVER).rounding(Rounding::same(10.0))
                 .min_size(Vec2::new(120.0, 42.0));

                if ui.add(cancel_btn).clicked() {
                    should_cancel = true;
                }
            });
        });

    if should_save {
        app.save_profile();
    } else if should_cancel || !open {
        app.show_profile_edit = false;
        app.edit_error.clear();
    }
}

// ==================== MAIN GUI ====================
impl eframe::App for FoodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = BG_DARK;
        visuals.window_fill = BG_PANEL;
        visuals.extreme_bg_color = BG_DARK;
        visuals.widgets.hovered.bg_fill = BG_HOVER;
        visuals.selection.bg_fill = BRAND_PRIMARY.linear_multiply(0.4);
        ctx.set_visuals(visuals);

        if !self.logged_in {
            render_login(self, ctx);
            return;
        }

        self.update_delivery_status(ctx);

        // ========== NAVBAR ==========
        egui::TopBottomPanel::top("navbar")
            .frame(Frame::none().fill(BG_PANEL).inner_margin(Margin::symmetric(20.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🍕").size(28.0));
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("FoodExpress").size(20.0).strong().color(BRAND_PRIMARY));
                        ui.label(RichText::new("Shamshabad, India").size(10.0).color(TEXT_DIM));
                    });

                    ui.add_space(20.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new(RichText::new("🚪 Logout").size(12.0).color(TEXT_PRIMARY))
                            .fill(BG_HOVER).rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(0.0, 36.0))).clicked() {
                            self.logout();
                            return;
                        }

                        ui.add_space(6.0);

                        let bell_text = if self.unread_count > 0 {
                            format!("🔔 ({})", self.unread_count)
                        } else {
                            "🔔".to_string()
                        };
                        let bell_fill = if self.unread_count > 0 { ACCENT_RED } else { BG_HOVER };
                        if ui.add(egui::Button::new(RichText::new(bell_text).size(13.0).color(Color32::WHITE))
                            .fill(bell_fill).rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(0.0, 36.0))).clicked() {
                            self.show_notifications = !self.show_notifications;
                            if self.show_notifications { self.unread_count = 0; }
                        }

                        ui.add_space(6.0);

                        if ui.add(egui::Button::new(RichText::new("📜 Orders").size(12.0).color(TEXT_PRIMARY))
                            .fill(BG_HOVER).rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(0.0, 36.0))).clicked() {
                            self.show_history = !self.show_history;
                            self.show_cart = false;
                        }

                        ui.add_space(6.0);

                        let cart_label = if self.show_cart {
                            "✖ Close Cart".to_string()
                        } else {
                            format!("🛒 Cart • {} • ₹{:.2}", self.item_count(), self.total())
                        };
                        let cart_fill = if self.show_cart { BG_HOVER } else { BRAND_PRIMARY };
                        if ui.add(egui::Button::new(RichText::new(cart_label).size(13.0).strong().color(Color32::WHITE))
                            .fill(cart_fill).rounding(Rounding::same(8.0))
                            .min_size(Vec2::new(170.0, 36.0))).clicked() {
                            self.show_cart = !self.show_cart;
                            self.show_history = false;
                        }

                        ui.add_space(6.0);

                        if let Some(user) = self.user.clone() {
                            let profile_btn = egui::Button::new(
                                RichText::new(format!("👤  {}", user.name))
                                    .size(12.0).strong().color(TEXT_PRIMARY)
                            ).fill(BG_HOVER).rounding(Rounding::same(20.0))
                             .min_size(Vec2::new(0.0, 36.0));

                            if ui.add(profile_btn).on_hover_text("Click to edit profile").clicked() {
                                self.open_profile_editor();
                            }
                        }
                    });
                });
            });

        // ========== TOAST ==========
        if let Some((msg, ttl)) = self.toast.clone() {
            let new_ttl = ttl - ctx.input(|i| i.stable_dt as f64);
            if new_ttl > 0.0 {
                self.toast = Some((msg.clone(), new_ttl));
                let alpha = (new_ttl / 2.5).min(1.0) as f32;
                egui::Area::new("toast".into())
                    .anchor(egui::Align2::CENTER_TOP, Vec2::new(0.0, 80.0))
                    .show(ctx, |ui| {
                        Frame::none().fill(ACCENT_GREEN.linear_multiply(alpha))
                            .rounding(Rounding::same(20.0))
                            .inner_margin(Margin::symmetric(20.0, 10.0))
                            .show(ui, |ui| {
                                ui.label(RichText::new(msg).size(14.0).strong().color(Color32::WHITE));
                            });
                    });
            } else {
                self.toast = None;
            }
            ctx.request_repaint();
        }

        // ========== PROFILE EDIT WINDOW ==========
        if self.show_profile_edit {
            render_profile_edit(self, ctx);
        }

        // ========== NOTIFICATIONS WINDOW ==========
        if self.show_notifications {
            render_notifications(self, ctx);
        }

        // ========== ORDER HISTORY WINDOW ==========
        if self.show_history {
            egui::Window::new("📜 My Orders")
                .open(&mut self.show_history)
                .resizable(true)
                .default_size([600.0, 500.0])
                .frame(Frame::window(&ctx.style()).fill(BG_PANEL))
                .show(ctx, |ui| {
                    if self.history.is_empty() {
                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("📭").size(64.0));
                            ui.add_space(8.0);
                            ui.label(RichText::new("No orders yet").size(18.0).color(TEXT_DIM));
                        });
                    } else {
                        let history_clone: Vec<OrderRecord> = self.history.iter().cloned().collect();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for order in history_clone.iter().rev() {
                                Frame::none().fill(BG_CARD).rounding(Rounding::same(12.0))
                                    .inner_margin(Margin::same(16.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(RichText::new(format!("Order #{}", order.id)).size(16.0).strong().color(TEXT_PRIMARY));
                                                ui.label(RichText::new(&order.timestamp).size(11.0).color(TEXT_DIM));
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                Frame::none().fill(ACCENT_GREEN.linear_multiply(0.2))
                                                    .rounding(Rounding::same(15.0))
                                                    .inner_margin(Margin::symmetric(10.0, 4.0))
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new(format!("{} {}", order.status.emoji(), order.status.label()))
                                                            .size(11.0).strong().color(ACCENT_GREEN));
                                                    });
                                            });
                                        });
                                        ui.add_space(6.0);
                                        ui.label(RichText::new(format!("👤 {}  •  📍 {}", order.customer_name, order.address)).size(12.0).color(TEXT_DIM));
                                        ui.add_space(8.0);
                                        ui.separator();
                                        for item in &order.items {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(format!("• {}  ×{}", item.name, item.quantity)).size(12.0).color(TEXT_PRIMARY));
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.label(RichText::new(format!("₹{:.2}", item.subtotal())).size(12.0).color(TEXT_DIM));
                                                });
                                            });
                                        }
                                        ui.separator();
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(format!("💳 {}", order.payment)).size(11.0).color(TEXT_DIM));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(format!("Total ₹{:.2}", order.total)).size(15.0).strong().color(ACCENT_GREEN));
                                            });
                                        });
                                    });
                                ui.add_space(10.0);
                            }
                        });
                    }
                });
        }

        // ========== LIVE ORDER TRACKING ==========
        if self.order_placed {
            egui::Window::new("🎯 Order Tracking")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-20.0, -20.0))
                .default_size([380.0, 0.0])
                .frame(Frame::window(&ctx.style()).fill(BG_PANEL))
                .show(ctx, |ui| {
                    render_live_tracking(self, ui);
                    if self.current_status == OrderStatus::Delivered {
                        ui.add_space(10.0);
                        if primary_button(ui, "🔄  Order Again", ACCENT_GREEN).clicked() {
                            self.new_order();
                        }
                    }
                });
        }

        // ========== CART SIDEBAR ==========
        if self.show_cart {
            egui::SidePanel::right("cart_panel")
                .resizable(true)
                .default_width(440.0)
                .min_width(380.0)
                .max_width(550.0)
                .frame(Frame::none().fill(BG_PANEL).inner_margin(Margin::same(20.0)))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🛒 Your Cart").size(20.0).strong().color(TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(egui::Button::new(RichText::new("✖").size(16.0).color(TEXT_PRIMARY))
                                .fill(BG_HOVER).rounding(Rounding::same(6.0))
                                .min_size(Vec2::new(30.0, 30.0))).clicked() {
                                self.show_cart = false;
                            }
                        });
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    if self.cart.is_empty() {
                        ui.add_space(30.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("🛒").size(56.0));
                            ui.add_space(8.0);
                            ui.label(RichText::new("Cart is empty").size(16.0).color(TEXT_DIM));
                        });
                    } else {
                        let mut action: Option<(usize, i32)> = None;

                        egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                            for (i, item) in self.cart.iter().enumerate() {
                                Frame::none().fill(BG_CARD).rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::same(12.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(RichText::new(&item.name).size(14.0).strong().color(TEXT_PRIMARY));
                                                ui.label(RichText::new(format!("₹{:.2} each", item.unit_price)).size(11.0).color(TEXT_DIM));
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(format!("₹{:.2}", item.subtotal())).size(14.0).strong().color(TEXT_PRIMARY));

                                                if ui.add(egui::Button::new(RichText::new("+").size(14.0).strong().color(Color32::WHITE))
                                                    .fill(BRAND_PRIMARY).rounding(Rounding::same(6.0))
                                                    .min_size(Vec2::new(26.0, 26.0))).clicked() {
                                                    action = Some((i, 1));
                                                }
                                                ui.add_space(3.0);
                                                ui.label(RichText::new(format!("{}", item.quantity)).size(13.0).strong().color(ACCENT_GREEN));
                                                ui.add_space(3.0);
                                                if ui.add(egui::Button::new(RichText::new("−").size(14.0).strong().color(TEXT_PRIMARY))
                                                    .fill(BG_HOVER).rounding(Rounding::same(6.0))
                                                    .min_size(Vec2::new(26.0, 26.0))).clicked() {
                                                    action = Some((i, -1));
                                                }
                                                if ui.add(egui::Button::new(RichText::new("🗑").size(11.0))
                                                    .fill(Color32::TRANSPARENT).rounding(Rounding::same(6.0))
                                                    .min_size(Vec2::new(26.0, 26.0))).clicked() {
                                                    action = Some((i, -999));
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(6.0);
                            }
                        });

                        if let Some((i, delta)) = action {
                            if delta == -999 { self.remove_from_cart(i); }
                            else { self.change_quantity(i, delta); }
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.label(RichText::new("🧾 Bill Details").size(14.0).strong().color(TEXT_PRIMARY));
                        ui.add_space(6.0);

                        let item_total = self.total() + self.savings();
                        let delivery = 40.0;
                        let gst = self.total() * 0.05;
                        let savings = self.savings();

                        egui::Grid::new("bill").num_columns(2).spacing([10.0, 5.0]).show(ui, |ui| {
                            ui.label(RichText::new("Item Total").size(11.0).color(TEXT_DIM));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("₹{:.2}", item_total)).size(11.0).color(TEXT_PRIMARY));
                            });
                            ui.end_row();
                            ui.label(RichText::new("Delivery Fee").size(11.0).color(TEXT_DIM));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("₹{:.2}", delivery)).size(11.0).color(TEXT_PRIMARY));
                            });
                            ui.end_row();
                            ui.label(RichText::new("GST (5%)").size(11.0).color(TEXT_DIM));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("₹{:.2}", gst)).size(11.0).color(TEXT_PRIMARY));
                            });
                            ui.end_row();
                            if savings > 0.0 {
                                ui.label(RichText::new("🎉 Savings").size(11.0).color(ACCENT_GREEN));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(format!("-₹{:.2}", savings)).size(11.0).strong().color(ACCENT_GREEN));
                                });
                                ui.end_row();
                            }
                        });

                        ui.add_space(8.0);
                        Frame::none().stroke(Stroke::new(1.5_f32, BRAND_PRIMARY))
                            .rounding(Rounding::same(10.0))
                            .inner_margin(Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("To Pay").size(15.0).strong().color(TEXT_PRIMARY));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(RichText::new(format!("₹{:.2}", self.total() + delivery + gst))
                                            .size(19.0).strong().color(ACCENT_GREEN));
                                    });
                                });
                            });

                        ui.add_space(12.0);
                        ui.label(RichText::new("💳 Payment Method").size(13.0).strong().color(TEXT_PRIMARY));
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            for (m, emoji) in [("Cash on Delivery", "💵"), ("UPI", "📱"), ("Credit Card", "💳")] {
                                let selected = self.payment_method == m;
                                let fill = if selected { BRAND_PRIMARY } else { BG_CARD };
                                let tc = if selected { Color32::WHITE } else { TEXT_PRIMARY };
                                if ui.add(egui::Button::new(RichText::new(format!("{} {}", emoji, m)).size(11.0).color(tc))
                                    .fill(fill).rounding(Rounding::same(15.0))
                                    .min_size(Vec2::new(0.0, 28.0))).clicked() {
                                    self.payment_method = m.to_string();
                                }
                            }
                        });

                        if !self.error_msg.is_empty() {
                            ui.add_space(10.0);
                            Frame::none().fill(ACCENT_RED.linear_multiply(0.2))
                                .stroke(Stroke::new(1.0_f32, ACCENT_RED))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::same(10.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(format!("⚠️ {}", self.error_msg)).size(12.0).color(ACCENT_RED));
                                });
                        }

                        ui.add_space(12.0);
                        if primary_button(ui, "✅  Place Order", BRAND_PRIMARY).clicked() {
                            self.place_order();
                        }

                        ui.add_space(8.0);
                        if ui.add(egui::Button::new(RichText::new("🗑️ Clear Cart").size(11.0).color(TEXT_DIM))
                            .fill(Color32::TRANSPARENT).rounding(Rounding::same(6.0))).clicked() {
                            while !self.cart.is_empty() { self.remove_from_cart(0); }
                        }
                    }
                });
        }

        // ========== MAIN CONTENT ==========
        egui::CentralPanel::default()
            .frame(Frame::none().fill(BG_DARK).inner_margin(Margin::symmetric(20.0, 16.0)))
            .show(ctx, |ui| {
                let user_name = self.user.as_ref().map(|u| u.name.clone()).unwrap_or_default();
                let hero_text = format!("👋 Hi {}, Hungry?", user_name);

                Frame::none().fill(BG_PANEL).rounding(Rounding::same(16.0))
                    .inner_margin(Margin::same(20.0))
                    .stroke(Stroke::new(2.0_f32, BRAND_PRIMARY.linear_multiply(0.4)))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&hero_text).size(24.0).strong().color(TEXT_PRIMARY));
                                ui.label(RichText::new("Order now, delivered in 30 minutes")
                                    .size(12.0).color(TEXT_DIM));
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    for (emoji, txt, col) in [
                                        ("⭐", "4.5 Rated", ACCENT_GREEN),
                                        ("🚀", "Fast Delivery", ACCENT_ORANGE),
                                        ("🎁", "30% OFF", BRAND_PRIMARY),
                                    ] {
                                        Frame::none().fill(col.linear_multiply(0.2))
                                            .rounding(Rounding::same(12.0))
                                            .inner_margin(Margin::symmetric(10.0, 4.0))
                                            .show(ui, |ui| {
                                                ui.label(RichText::new(format!("{} {}", emoji, txt)).size(10.0).strong().color(col));
                                            });
                                    }
                                });
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("🍔🍕🍟").size(50.0));
                            });
                        });
                    });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    Frame::none().fill(BG_PANEL).rounding(Rounding::same(10.0))
                        .inner_margin(Margin::symmetric(12.0, 6.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🔍").size(14.0).color(TEXT_DIM));
                                ui.add(egui::TextEdit::singleline(&mut self.search_query)
                                    .hint_text("Search dishes...").desired_width(300.0).frame(false));
                            });
                        });

                    ui.add_space(10.0);

                    let chip = |ui: &mut egui::Ui, sel: bool, label: &str| -> bool {
                        let (f, t) = if sel { (BRAND_PRIMARY, Color32::WHITE) } else { (BG_PANEL, TEXT_PRIMARY) };
                        ui.add(egui::Button::new(RichText::new(label).size(11.0).strong().color(t))
                            .fill(f).rounding(Rounding::same(18.0))
                            .min_size(Vec2::new(0.0, 30.0))).clicked()
                    };

                    if chip(ui, self.category_filter.is_none(), "🍽️ All") {
                        self.category_filter = None;
                    }
                    for cat in Category::all() {
                        let is_sel = self.category_filter.as_ref() == Some(&cat);
                        let label = format!("{} {}", cat.emoji(), cat.name());
                        if chip(ui, is_sel, &label) {
                            self.category_filter = if is_sel { None } else { Some(cat) };
                        }
                    }
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("🔥 Popular Dishes").size(20.0).strong().color(TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{} dishes", self.menu.len())).size(12.0).color(TEXT_DIM));
                    });
                });

                ui.add_space(12.0);

                let query = self.search_query.to_lowercase();
                let filter = self.category_filter.clone();

                let visible: Vec<(usize, String, f64, f64, String, u32, f32, bool, String, String)> = self.menu.iter().enumerate()
                    .filter(|(_, item)| {
                        (query.is_empty() || item.name.to_lowercase().contains(&query)
                            || item.description.to_lowercase().contains(&query))
                            && (filter.is_none() || filter.as_ref() == Some(&item.category))
                    })
                    .map(|(i, item)| (
                        i, item.name.clone(), item.price, item.original_price,
                        item.category.name().to_string(), item.stock,
                        item.rating, item.is_veg, item.description.clone(),
                        item.category.emoji().to_string(),
                    ))
                    .collect();

                let mut add_idx: Option<usize> = None;

                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    if visible.is_empty() {
                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("🔍").size(56.0).color(TEXT_DIM));
                            ui.add_space(10.0);
                            ui.label(RichText::new("No dishes found").size(16.0).color(TEXT_PRIMARY));
                        });
                        return;
                    }

                    let rows = (visible.len() + 1) / 2;
                    for row in 0..rows {
                        let start = row * 2;
                        let end = (start + 2).min(visible.len());

                        ui.columns(2, |cols| {
                            for (col_i, item_i) in (start..end).enumerate() {
                                let (idx, name, price, orig, cat, stock, rating, is_veg, desc, emoji) = &visible[item_i];
                                let ui = &mut cols[col_i];

                                Frame::none().fill(BG_PANEL).stroke(Stroke::new(1.0_f32, BG_HOVER))
                                    .rounding(Rounding::same(12.0))
                                    .inner_margin(Margin::same(14.0))
                                    .show(ui, |ui| {
                                        ui.set_min_width(ui.available_width());
                                        ui.horizontal(|ui| {
                                            Frame::none().fill(BG_HOVER).rounding(Rounding::same(10.0))
                                                .inner_margin(Margin::same(12.0))
                                                .show(ui, |ui| { ui.label(RichText::new(emoji).size(28.0)); });
                                            ui.add_space(8.0);
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    let (bc, dc) = if *is_veg { (ACCENT_GREEN, ACCENT_GREEN) } else { (ACCENT_RED, ACCENT_RED) };
                                                    Frame::none().stroke(Stroke::new(1.5_f32, bc)).rounding(Rounding::same(3.0))
                                                        .inner_margin(Margin::symmetric(3.0, 1.0))
                                                        .show(ui, |ui| { ui.label(RichText::new("●").size(7.0).color(dc)); });
                                                    ui.label(RichText::new(name).size(14.0).strong().color(TEXT_PRIMARY));
                                                });
                                                ui.label(RichText::new(desc).size(10.0).color(TEXT_DIM));
                                                ui.horizontal(|ui| {
                                                    Frame::none().fill(ACCENT_GREEN).rounding(Rounding::same(5.0))
                                                        .inner_margin(Margin::symmetric(5.0, 2.0))
                                                        .show(ui, |ui| {
                                                            ui.label(RichText::new(format!("⭐{:.1}", rating)).size(9.0).strong().color(Color32::WHITE));
                                                        });
                                                    ui.label(RichText::new(cat).size(9.0).color(TEXT_MUTED));
                                                });
                                            });
                                        });

                                        ui.add_space(8.0);
                                        ui.separator();
                                        ui.add_space(4.0);

                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(RichText::new(format!("₹{:.2}", price)).size(16.0).strong().color(TEXT_PRIMARY));
                                                if orig > price {
                                                    ui.label(RichText::new(format!("₹{:.2}", orig)).size(10.0).color(TEXT_DIM).strikethrough());
                                                }
                                            });
                                            if orig > price {
                                                let disc = ((orig - price) / orig * 100.0).round() as i32;
                                                Frame::none().fill(ACCENT_ORANGE.linear_multiply(0.2)).rounding(Rounding::same(5.0))
                                                    .inner_margin(Margin::symmetric(5.0, 2.0))
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new(format!("{}%", disc)).size(9.0).strong().color(ACCENT_ORANGE));
                                                    });
                                            }
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let ok = *stock > 0;
                                                let fill = if ok { BRAND_PRIMARY } else { BG_HOVER };
                                                let txt = if ok { "ADD" } else { "SOLD" };
                                                let b = egui::Button::new(RichText::new(txt).size(11.0).strong().color(Color32::WHITE))
                                                    .fill(fill).rounding(Rounding::same(8.0))
                                                    .min_size(Vec2::new(70.0, 30.0));
                                                if ui.add_enabled(ok, b).clicked() { add_idx = Some(*idx); }
                                            });
                                        });

                                        if *stock < 5 && *stock > 0 {
                                            ui.label(RichText::new(format!("⚠️ {} left", stock)).size(9.0).color(ACCENT_ORANGE));
                                        }
                                    });
                            }
                        });
                        ui.add_space(10.0);
                    }
                });

                if let Some(i) = add_idx { self.add_to_cart(i); }
            });
    }
}

// ==================== MAIN ====================
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1450.0, 880.0])
            .with_min_inner_size([1100.0, 700.0])
            .with_title("🍕 FoodExpress — Shamshabad"),
        ..Default::default()
    };

    eframe::run_native(
        "FoodExpress",
        options,
        Box::new(|_cc| Box::new(FoodApp::default())),
    )
}