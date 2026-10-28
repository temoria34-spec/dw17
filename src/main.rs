use dw17::item::Item;

fn main() {
    let item = Item::new("A1", "Pretzels", 125);
    println!("{}", dw17::show_item());
    println!("{}", dw17::slot_label(&item));
}
