use design_pattern::behavior_pattern::mediator_pattern::User;

fn main() {
    let robert = User::new("Robert");
    let john = User::new("John");

    robert.send_message("Hi! John!".to_owned());
    john.send_message("Hello! Robert!".to_owned());
}
