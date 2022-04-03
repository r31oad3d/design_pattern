use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct Person {
    pub name: String,
    pub gender: String,
    pub marital_status: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Person {{ name: {},\t gender: {},\t marital_status:{} }}",
            self.name, self.gender, self.marital_status
        )
    }
}

impl Person {
    pub fn new(name: String, gender: String, marital_status: String) -> Person {
        Person {
            name,
            gender,
            marital_status,
        }
    }
}

pub trait Criteria {
    fn meet_criteria<'a>(&self, persons: &'a [Rc<Person>]) -> Vec<Rc<Person>>;
}

pub struct CriteriaMale;
impl Criteria for CriteriaMale {
    fn meet_criteria(&self, persons: &[Rc<Person>]) -> Vec<Rc<Person>> {
        persons
            .iter()
            .filter(|person| person.gender.eq_ignore_ascii_case("MALE"))
            .cloned()
            .collect::<Vec<Rc<Person>>>()
    }
}

pub struct CriteriaFemale;
impl Criteria for CriteriaFemale {
    fn meet_criteria(&self, persons: &[Rc<Person>]) -> Vec<Rc<Person>> {
        persons
            .iter()
            .filter(|person| person.gender.eq_ignore_ascii_case("FEMALE"))
            .cloned()
            .collect::<Vec<Rc<Person>>>()
    }
}

pub struct CriteriaSingle;
impl Criteria for CriteriaSingle {
    fn meet_criteria(&self, persons: &[Rc<Person>]) -> Vec<Rc<Person>> {
        persons
            .iter()
            .filter(|person| {
                person.marital_status.eq_ignore_ascii_case("SINGLE")
            })
            .cloned()
            .collect::<Vec<Rc<Person>>>()
    }
}

pub struct AndCriteria {
    criteria: Rc<dyn Criteria>,
    criteria_other: Rc<dyn Criteria>,
}
impl AndCriteria {
    pub fn new(
        criteria: Rc<dyn Criteria>,
        criteria_other: Rc<dyn Criteria>,
    ) -> Self {
        AndCriteria {
            criteria,
            criteria_other,
        }
    }
}
impl Criteria for AndCriteria {
    fn meet_criteria(&self, persons: &[Rc<Person>]) -> Vec<Rc<Person>> {
        let temp = self.criteria_other.meet_criteria(persons);
        self.criteria.meet_criteria(temp.as_slice())
    }
}

pub struct OrCriteria {
    criteria: Rc<dyn Criteria>,
    criteria_other: Rc<dyn Criteria>,
}
impl OrCriteria {
    pub fn new(
        criteria: Rc<dyn Criteria>,
        criteria_other: Rc<dyn Criteria>,
    ) -> Self {
        OrCriteria {
            criteria,
            criteria_other,
        }
    }
}
impl Criteria for OrCriteria {
    fn meet_criteria(&self, persons: &[Rc<Person>]) -> Vec<Rc<Person>> {
        let mut temp1 = self.criteria.meet_criteria(persons);
        let temp2 = self.criteria_other.meet_criteria(persons);
        for person in &temp2 {
            if !temp1.contains(&person) {
                temp1.push(person.clone());
            }
        }
        temp1
    }
}
