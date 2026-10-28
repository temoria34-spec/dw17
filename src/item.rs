//! An item for sale.

pub struct Item {
    code: String,
    name: String,
    price_cents: u32,
}

impl Item {
    /// Creates an item from its slot code, name and price in cents.
    ///
    /// ```
    /// use dw17::item::Item;
    ///
    /// let item = Item::new("A1", "Pretzels", 125);
    /// assert_eq!(item.code(), "A1");
    /// ```
    pub fn new(code: &str, name: &str, price_cents: u32) -> Item {
        Item {
            code: code.to_string(),
            name: name.to_string(),
            price_cents,
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// The price in dollars.
    ///
    /// ```
    /// use dw17::item::Item;
    ///
    /// let item = Item::new("A1", "Pretzels", 125);
    /// assert_eq!(item.price_dollars(), 1.25);
    /// ```
    pub fn price_dollars(&self) -> f64 {
        let cents = self.price_cents as f64
        cents / 100.0
    }
}
