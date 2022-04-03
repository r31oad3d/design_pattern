use design_pattern::creational_pattern::abstract_factory_pattern::{
    Category, Colors, FactoryProducer, Shapes,
};

fn main() {
    let shape_factory = FactoryProducer::get_factory(Category::Shapes);

    let shape1 = shape_factory.get_shape(Shapes::Rectangle);
    shape1.draw();

    let shape2 = shape_factory.get_shape(Shapes::Circle);
    shape2.draw();

    let color_factory = FactoryProducer::get_factory(Category::Colors);

    let color1 = color_factory.get_color(Colors::Red);
    color1.fill();

    let color2 = color_factory.get_color(Colors::Blue);
    color2.fill();
}
