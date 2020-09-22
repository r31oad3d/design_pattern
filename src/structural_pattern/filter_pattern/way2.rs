use super::way1::Person;
use std::rc::Rc;
//TODO
pub enum Criterias {
    Name(String),
    Gender(String),
    Marital(String),
    All,
    Not(Box<Criterias>),
    And(Box<Criterias>, Box<Criterias>),
    Or(Box<Criterias>, Box<Criterias>),
}

#[deprecated(note = "use the macro instead, this do not support And,Or,Not")]
pub fn get_criteria(
    keyword: Criterias,
) -> Box<dyn FnMut(&&Rc<Person>) -> bool> {
    match keyword {
        Criterias::All => Box::new(|_person: &&Rc<Person>| true),
        Criterias::Gender(key) => Box::new(move |person: &&Rc<Person>| {
            person.gender.eq_ignore_ascii_case(&key.clone())
        }),
        Criterias::Marital(key) => Box::new(move |person: &&Rc<Person>| {
            person.marital_status.eq_ignore_ascii_case(key.as_str())
        }),
        Criterias::Name(key) => Box::new(move |person: &&Rc<Person>| {
            person.name.eq_ignore_ascii_case(key.as_str())
        }),
        _ => unimplemented!(),
    }
}

pub fn do_filter<F>(persons: &[Rc<Person>], criteria: F) -> Vec<Rc<Person>>
where
    F: FnMut(&&Rc<Person>) -> bool,
{
    persons
        .iter()
        .filter(criteria)
        .cloned()
        .collect::<Vec<Rc<Person>>>()
}

// get_criteria!(gender="MALE")
// get_criteria!(and(gender="MALE", name="John"))
// get_criteria!(or(and(gender="FEMALE", marital="SINGLE"), not(name="John")))
// get_criteria!(not(marital="married"))
// macro_rules! get_criteria {
//     ($field:expr => $value:expr) => {{
//         println!("{},{}", $field, $value)
//     }};
// }

//
// fn way2_filter_test() {
//     let c1 = get_criteria!(
//             "person.gender" => "male"
//     );
// }
