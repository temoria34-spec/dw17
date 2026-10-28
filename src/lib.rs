//! # DW17 — Item Struct and Methods
//!
//! ```text
//! Pretzels $1.25
//! [A1]
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod item;

/// Shows the item in slot A1 as `name $price`.
///
/// ```
/// assert_eq!(dw17::show_item(), "Pretzels $1.25");
/// ```
pub fn show_item() -> String {
    let item = Item::new("A1", "Pretzels", 125);
    format!("{} ${}", item.name(), item.price_dollars())
}

/// Shows the slot label for an item, like `[A1]`.
///
/// ```
/// use dw17::item::Item;
///
/// let item = Item::new("B4", "Chips", 90);
/// assert_eq!(dw17::slot_label(&item), "[B4]");
/// ```
pub fn slot_label(item: &item::Item) -> String {
    format!("[{}]", item.code)
}
