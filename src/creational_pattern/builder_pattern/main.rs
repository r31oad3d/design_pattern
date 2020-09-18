use design_pattern::creational_pattern::builder_pattern::{way1, way2};

fn main() {
    println!("way1: >>>>>");

    let meal1 = way1::MealBuilder::prepare_veg_meal();
    println!("veg meal");
    meal1.show_item();
    println!("Total Cost: {}", meal1.get_cost());

    let meal2 = way1::MealBuilder::prepare_non_veg_meal();
    println!("non veg meal");
    meal2.show_item();
    println!("Total Cost: {}", meal2.get_cost());

    // println!("way2: >>>>>");
    //
    // let meal3 = way2::MealBuilder::prepare_veg_meal();
    // println!("veg meal");
    // meal3.show_item();
    // println!("Total Cost: {}", meal3.get_cost());
    //
    // let meal4 = way2::MealBuilder::prepare_non_veg_meal();
    // println!("non veg meal");
    // meal4.show_item();
    // println!("Total Cost: {}", meal4.get_cost());
}
