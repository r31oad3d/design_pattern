use design_pattern::structural_pattern::composite_pattern::Employee;

fn main() {
    let mut ceo = Employee::new("John".to_owned(), "CEO".to_owned(), 30000);
    let mut head_sales =
        Employee::new("Robert".to_owned(), "Head Sales".to_owned(), 20000);

    let mut head_marketing =
        Employee::new("Robert".to_owned(), "Head Sales".to_owned(), 20000);
    let clerk1 =
        Employee::new("Laura".to_owned(), "Marketing".to_owned(), 10000);
    let clerk2 = Employee::new("Bob".to_owned(), "Marketing".to_owned(), 10000);

    let sales_executive1 =
        Employee::new("Richard".to_owned(), "Sales".to_owned(), 10000);
    let sales_executive2 =
        Employee::new("Rob".to_owned(), "Sales".to_owned(), 10000);

    head_sales.add(sales_executive1);
    head_sales.add(sales_executive2);

    head_marketing.add(clerk1);
    head_marketing.add(clerk2);

    ceo.add(head_sales);
    ceo.add(head_marketing);

    println!("{}", ceo);
    for head_employee in ceo.get_subordinates() {
        println!("{}", head_employee);
        for employ in head_employee.get_subordinates() {
            println!("{}", employ);
        }
    }
}
