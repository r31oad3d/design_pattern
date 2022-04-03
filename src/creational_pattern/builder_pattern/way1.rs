pub trait Item<'a> {
    fn name(&self) -> &'a str;
    fn packing(&self) -> Box<dyn Packing<'a>>;
    fn price(&self) -> f32;
}

pub trait Packing<'a> {
    fn pack(&self) -> &'a str;
}

#[derive(Default)]
pub struct Wrapper;
impl Wrapper {
    fn new() -> Self {
        println!("Wrapper created.");
        Wrapper
    }
}
impl<'a> Packing<'a> for Wrapper {
    fn pack(&self) -> &'a str {
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
impl<'a> Packing<'a> for Bottle {
    fn pack(&self) -> &'a str {
        "Bottle"
    }
}

pub trait Burger<'a>: Item<'a> {
    fn name(&self) -> &'a str;
    fn packing(&self) -> Box<dyn Packing<'a>> {
        Box::new(Wrapper::new())
    }

    fn price(&self) -> f32;
}

pub trait Drink<'a>: Item<'a> {
    fn name(&self) -> &'a str;
    fn packing(&self) -> Box<dyn Packing<'a>> {
        Box::new(Bottle::new())
    }

    fn price(&self) -> f32;
}

pub struct VegBurger;
impl<'a> Item<'a> for VegBurger {
    fn name(&self) -> &'a str {
        <VegBurger as Burger>::name(&self)
    }

    fn packing(&self) -> Box<dyn Packing<'a>> {
        <VegBurger as Burger>::packing(&self)
    }

    fn price(&self) -> f32 {
        <VegBurger as Burger>::price(&self)
    }
}
impl<'a> Burger<'a> for VegBurger {
    fn name(&self) -> &'a str {
        "Veg Burger"
    }

    fn price(&self) -> f32 {
        25.5
    }
}

pub struct ChickenBurger;
impl<'a> Item<'a> for ChickenBurger {
    fn name(&self) -> &'a str {
        <ChickenBurger as Burger>::name(&self)
    }

    fn packing(&self) -> Box<dyn Packing<'a>> {
        <ChickenBurger as Burger>::packing(&self)
    }

    fn price(&self) -> f32 {
        <ChickenBurger as Burger>::price(&self)
    }
}
impl<'a> Burger<'a> for ChickenBurger {
    fn name(&self) -> &'a str {
        "Chicken Burger"
    }

    fn price(&self) -> f32 {
        50.0
    }
}

pub struct Coke;
impl<'a> Item<'a> for Coke {
    fn name(&self) -> &'a str {
        <Coke as Drink>::name(&self)
    }

    fn packing(&self) -> Box<dyn Packing<'a>> {
        <Coke as Drink>::packing(&self)
    }

    fn price(&self) -> f32 {
        <Coke as Drink>::price(&self)
    }
}
impl<'a> Drink<'a> for Coke {
    fn name(&self) -> &'a str {
        "Coke"
    }

    fn price(&self) -> f32 {
        11.5
    }
}

pub struct Pepsi;
impl<'a> Item<'a> for Pepsi {
    fn name(&self) -> &'a str {
        <Pepsi as Drink>::name(&self)
    }

    fn packing(&self) -> Box<dyn Packing<'a>> {
        <Pepsi as Drink>::packing(&self)
    }

    fn price(&self) -> f32 {
        <Pepsi as Drink>::price(&self)
    }
}
impl<'a> Drink<'a> for Pepsi {
    fn name(&self) -> &'a str {
        "Pepsi"
    }

    fn price(&self) -> f32 {
        12.0
    }
}

pub struct Meal<'a> {
    pub items: Vec<Box<dyn Item<'a>>>,
}
impl<'a> Meal<'a> {
    pub fn add_item(&mut self, item: Box<dyn Item<'a>>) {
        self.items.push(item);
    }

    pub fn get_cost(&self) -> f32 {
        self.items.iter().map(|x| x.price()).sum()
    }
    pub fn show_item(&self) {
        self.items.iter().for_each(|x| {
            println!(
                "Item: {}, Packing: {}, Price: {}",
                x.name(),
                x.packing().pack(),
                x.price()
            )
        });
    }
}

pub struct MealBuilder;
impl MealBuilder {
    pub fn prepare_veg_meal<'a>() -> Meal<'a> {
        Meal {
            items: vec![Box::new(VegBurger), Box::new(Coke)],
        }
    }

    pub fn prepare_non_veg_meal<'a>() -> Meal<'a> {
        Meal {
            items: vec![Box::new(ChickenBurger), Box::new(Pepsi)],
        }
    }
}
