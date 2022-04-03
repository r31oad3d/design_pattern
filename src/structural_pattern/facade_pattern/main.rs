use design_pattern::structural_pattern::facade_pattern::ShapeMaker;
fn main() {
    let shape_maker = ShapeMaker::new();

    shape_maker.draw_circle();
    shape_maker.draw_rectangle();
    shape_maker.draw_square();
}
