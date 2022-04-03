use design_pattern::structural_pattern::bridge_pattern::{
    Circle, GreenCircle, RedCircle, Shape,
};

fn main() {
    let red_circle =
        Shape::<Circle>::new(Circle::new(100, 100, 10, Box::new(RedCircle {})));
    let green_circle = Shape::<Circle>::new(Circle::new(
        100,
        100,
        10,
        Box::new(GreenCircle {}),
    ));

    red_circle.draw();
    green_circle.draw();
}
