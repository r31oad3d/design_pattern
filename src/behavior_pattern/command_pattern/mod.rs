pub trait Order {
    fn execute(&self);
}

pub struct Stock<'a> {
    name: &'a str,
    quantity: i32,
}

impl Default for Stock<'_> {
    fn default() -> Self {
        Stock {
            name: "ABC",
            quantity: 10,
        }
    }
}

impl Stock<'_> {
    fn buy(&self) {
        println!(
            "Stock [ Name: {}, Quantity: {} ] bought",
            self.name, self.quantity
        );
    }
    fn sell(&self) {
        println!(
            "Stock [ Name: {}, Quantity: {} ] sold",
            self.name, self.quantity
        );
    }
}

pub struct BuyStock<'a> {
    stock: &'a Stock<'a>,
}

impl<'a> BuyStock<'a> {
    pub fn new(stock: &'a Stock) -> BuyStock<'a> {
        BuyStock { stock }
    }
}

impl Order for BuyStock<'_> {
    fn execute(&self) {
        self.stock.buy()
    }
}
pub struct SellStock<'a> {
    stock: &'a Stock<'a>,
}
impl<'a> SellStock<'a> {
    pub fn new(stock: &'a Stock) -> SellStock<'a> {
        SellStock { stock }
    }
}
impl Order for SellStock<'_> {
    fn execute(&self) {
        self.stock.sell()
    }
}

pub struct Broker<'a> {
    pub order_list: Vec<Box<&'a dyn Order>>,
}

impl<'a> Broker<'a> {
    pub fn take_order(&mut self, order: &'a dyn Order) {
        self.order_list.push(Box::new(order));
    }

    pub fn place_orders(&mut self) {
        self.order_list.iter().for_each(|e| e.execute());
        self.order_list.clear();
    }
}
