use design_pattern::behavior_pattern::template_pattern::{
    Cricket,
    Game,
    Football
};

fn main() {
    let mut game = Cricket::new_game();
    game.play();
    game = Box::new(Football) as Box<dyn Game>;
    game.play();
}
