use design_pattern::behavior_pattern::command_pattern::{
    Broker, BuyStock, SellStock, Stock,
};

fn main() {
    let abc_stock = Stock::default();
    let buy_stock = BuyStock::new(&abc_stock);
    let sell_stock = SellStock::new(&abc_stock);

    let mut broker = Broker { order_list: vec![] };
    broker.take_order(&buy_stock);
    broker.take_order(&sell_stock);

    broker.place_orders();
}
