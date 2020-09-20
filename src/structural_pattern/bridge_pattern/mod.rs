pub trait DrawAPI {
    fn draw_circle(&self, radius: i32, x: i32, y: i32);
}

pub struct RedCircle {}
impl DrawAPI for RedCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!(
            "Drawing Circle[ color: red, radius: {}, x: {}, y: {} ]",
            radius, x, y
        );
    }
}

pub struct GreenCircle {}
impl DrawAPI for GreenCircle {
    fn draw_circle(&self, radius: i32, x: i32, y: i32) {
        println!(
            "Drawing Circle[ color: green, radius: {}, x: {}, y: {} ]",
            radius, x, y
        );
    }
}

pub trait ShapeDraw {
    fn draw(&self);
}

pub struct Shape<T>
where
    T: ShapeDraw,
{
    inner_shape: T,
}

impl<T> Shape<T>
where
    T: ShapeDraw,
{
    pub fn new(inner_shape: T) -> Self {
        Shape { inner_shape }
    }

    pub fn draw(&self) {
        self.inner_shape.draw();
    }
}
pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    draw_api: Box<dyn DrawAPI>,
}

impl Circle {
    pub fn new(
        x: i32,
        y: i32,
        radius: i32,
        draw_api: Box<dyn DrawAPI>,
    ) -> Self {
        Circle {
            x,
            y,
            radius,
            draw_api,
        }
    }
}

impl ShapeDraw for Circle {
    fn draw(&self) {
        self.draw_api.draw_circle(self.radius, self.x, self.y);
    }
}
