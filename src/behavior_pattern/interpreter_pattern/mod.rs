pub trait Expression<'a> {
    fn interpret(&self, context: &'a str) -> bool;
}

pub struct TerminalExpression<'a> {
    data: &'a str,
}

impl<'a> TerminalExpression<'a> {
    pub fn new(data: &'a str) -> Self {
        TerminalExpression { data }
    }
}

impl<'a> Expression<'a> for TerminalExpression<'a> {
    fn interpret(&self, context: &'a str) -> bool {
        context.contains(&self.data)
    }
}

pub struct OrExpression<'a> {
    expr1: &'a dyn Expression<'a>,
    expr2: &'a dyn Expression<'a>,
}

impl<'a> OrExpression<'a> {
    pub fn new(
        expr1: &'a dyn Expression<'a>,
        expr2: &'a dyn Expression<'a>,
    ) -> Self {
        OrExpression { expr1, expr2 }
    }
}

impl<'a> Expression<'a> for OrExpression<'a> {
    fn interpret(&self, context: &'a str) -> bool {
        self.expr1.interpret(context) || self.expr2.interpret(context)
    }
}

pub struct AndExpression<'a> {
    expr1: &'a dyn Expression<'a>,
    expr2: &'a dyn Expression<'a>,
}

impl<'a> AndExpression<'a> {
    pub fn new(
        expr1: &'a dyn Expression<'a>,
        expr2: &'a dyn Expression<'a>,
    ) -> Self {
        AndExpression { expr1, expr2 }
    }
}

impl<'a> Expression<'a> for AndExpression<'a> {
    fn interpret(&self, context: &'a str) -> bool {
        self.expr1.interpret(context) && self.expr2.interpret(context)
    }
}
