use design_pattern::behavior_pattern::interpreter_pattern::{
    AndExpression, Expression, OrExpression, TerminalExpression,
};

fn main() {
    let robert: &dyn Expression = &TerminalExpression::new("Robert");
    let john: &dyn Expression = &TerminalExpression::new("John");

    let julie: &dyn Expression = &TerminalExpression::new("Julie");
    let married: &dyn Expression = &TerminalExpression::new("Married");

    let is_male = OrExpression::new(robert, john);
    let is_married_women = AndExpression::new(julie, married);

    println!("John is male? {}", is_male.interpret("John"));
    println!(
        "Julie is a married women? {}",
        is_married_women.interpret("Married Julie")
    );
}
