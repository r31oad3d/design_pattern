use design_pattern::structural_pattern::filter_pattern::way1::{
    AndCriteria, Criteria, CriteriaFemale, CriteriaMale, CriteriaSingle,
    OrCriteria, Person,
};
use design_pattern::structural_pattern::filter_pattern::way2::{
    do_filter, get_criteria, Criterias,
};
use std::rc::Rc;

fn main() {
    let persons = vec![
        Rc::new(Person::new(
            "Robert".to_owned(),
            "Male".to_owned(),
            "Single".to_owned(),
        )),
        Rc::new(Person::new(
            "John".to_owned(),
            "Male".to_owned(),
            "Married".to_owned(),
        )),
        Rc::new(Person::new(
            "Laura".to_owned(),
            "Female".to_owned(),
            "Married".to_owned(),
        )),
        Rc::new(Person::new(
            "Diana".to_owned(),
            "Female".to_owned(),
            "Single".to_owned(),
        )),
        Rc::new(Person::new(
            "Mike".to_owned(),
            "Male".to_owned(),
            "Single".to_owned(),
        )),
        Rc::new(Person::new(
            "Bobby".to_owned(),
            "Male".to_owned(),
            "Single".to_owned(),
        )),
    ];

    let male = Rc::new(CriteriaMale {});
    let female = Rc::new(CriteriaFemale {});
    let single = Rc::new(CriteriaSingle {});
    let single_and_male = AndCriteria::new(male.clone(), single.clone());
    let male_or_female = OrCriteria::new(male.clone(), female.clone());
    let male_or_single = OrCriteria::new(male.clone(), single.clone());
    fn print_persons(persons: &[Rc<Person>]) {
        persons.iter().for_each(|person| {
            println!("{}", person);
        });
    }

    println!("Males: ");
    print_persons(&male.meet_criteria(&persons));

    println!("Femals: ");
    print_persons(&female.meet_criteria(&persons));

    println!("Singles: ");
    print_persons(&single.meet_criteria(&persons));

    println!("Singles and Male: ");
    print_persons(&single_and_male.meet_criteria(&persons));

    println!("Male or Female: ");
    print_persons(&male_or_female.meet_criteria(&persons));

    println!("Male or single: ");
    print_persons(&male_or_single.meet_criteria(&persons));

    println!("Way2: ");
    println!("Males: ");
    print_persons(&do_filter(
        &persons,
        get_criteria(Criterias::Marital(String::from("SINGLE"))),
    ));

    println!("All: ");
    print_persons(&do_filter(&persons, get_criteria(Criterias::All)));

    //get_criteria!()
}
