use std::ops::Deref;

pub struct Item
{
    meal: Box<dyn Category>,
}

impl Deref for Item
{
    type Target = Box<dyn Category>;

    fn deref(&self) -> &Self::Target {
        &self.meal
    }
}

pub trait Packing {
    fn pack(&self) -> &str;
}

#[derive(Default)]
pub struct Wrapper;
impl Wrapper {
    fn new() -> Self {
        println!("Wrapper created.");
        Wrapper
    }
}
impl Packing for Wrapper {
    fn pack(&self) -> &str {
        "Wrapper"
    }
}

#[derive(Default)]
pub struct Bottle;
impl Bottle {
    pub fn new() -> Self {
        println!("Bottle created.");
        Bottle
    }
}
impl Packing for Bottle {
    fn pack(&self) -> &str {
        "Bottle"
    }
}

pub trait Category {
    fn get_name(&self) -> &str;
    fn packing(&self) -> Box<dyn Packing>;
    fn get_price(&self) -> f32;
}

pub struct VegBurger;
pub struct ChickenBurger;
pub struct Coke;
pub struct Pepsi;

impl Category for VegBurger {
    fn get_name(&self) -> &str {
        "Veg Burger"
    }
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper::new())
    }
    fn get_price(&self) -> f32 {
        25.5
    }
}

impl Category for ChickenBurger {
    fn get_name(&self) -> &str {
        "Chicken Burger"
    }
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper::new())
    }
    fn get_price(&self) -> f32 {
        50.0
    }
}

impl Category for Coke {
    fn get_name(&self) -> &str {
        "Coke"
    }
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle::new())
    }
    fn get_price(&self) -> f32 {
        15.5
    }
}

impl Category for Pepsi {
    fn get_name(&self) -> &str {
        "Pepsi"
    }
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle::new())
    }
    fn get_price(&self) -> f32 {
        16.0
    }
}

pub struct Meal
{
    pub items: Vec<Item>,
}
impl Meal
{
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn get_cost(&self) -> f32 {
        self.items.iter().map(|x| x.get_price()).sum()
    }

    pub fn show_item(&self) {
        self.items.iter().for_each(|x| {
            println!(
                "Item: {}, Packing: {}, Price: {}",
                x.get_name(),
                x.packing().pack(),
                x.get_price()
            )
        });
    }
}

pub struct MealBuilder {

}
impl MealBuilder {
    pub fn prepare_veg_meal() -> Meal
    {
        Meal {
            items: vec![
                Item{meal: Box::new(VegBurger)},
                Item{meal: Box::new(Coke)},
            ],
        }
    }

    pub fn prepare_non_veg_meal() -> Meal{
        Meal {
            items: vec![
                Item{meal: Box::new(ChickenBurger)},
                Item{meal: Box::new(Pepsi)},
            ],
        }
    }
}
