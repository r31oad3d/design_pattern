use std::fmt::Formatter;

#[derive(Eq, PartialEq)]
pub struct Employee {
    name: String,
    dept: String,
    salary: i32,
    subordinates: Vec<Employee>,
}

impl Employee {
    pub fn new(name: String, dept: String, salary: i32) -> Self {
        Employee {
            name,
            dept,
            salary,
            subordinates: Vec::<Employee>::new(),
        }
    }

    pub fn add(&mut self, e: Employee) {
        self.subordinates.push(e);
    }

    pub fn remove(&mut self, e: Employee) {
        if let Some(index) = self.subordinates.iter().position(|x| *x == e) {
            self.subordinates.remove(index);
        }
    }

    pub fn get_subordinates(&self) -> &Vec<Employee> {
        &self.subordinates
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Employee: [name:{}, dept:{}, salary:{}]",
            self.name, self.dept, self.salary
        )
    }
}
