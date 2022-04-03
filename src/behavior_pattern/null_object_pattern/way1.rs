pub trait AbstractCustomer {
    fn is_null(&self) -> bool;
    fn get_name(&self) -> &str;
}

pub struct RealCustomer {
    name: String,
}
pub struct NullCustomer {}

impl AbstractCustomer for RealCustomer {
    fn is_null(&self) -> bool {
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl AbstractCustomer for NullCustomer {
    fn is_null(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        "Not Available in Customer Database"
    }
}

pub struct CustomerFactory {}

impl CustomerFactory {
    pub fn get_customer(name: &str) -> Box<dyn AbstractCustomer> {
        lazy_static! {
            pub static ref NAMES: Vec<&'static str> = {
                let mut v = vec![];
                v.push("Rob");
                v.push("Joe");
                v.push("Julie");
                v
            };
        }
        NAMES.iter().find(|&&n| n.eq(name)).map_or_else(
            || Box::new(NullCustomer {}) as Box<dyn AbstractCustomer>,
            |&n| {
                Box::new(RealCustomer {
                    name: String::from(n),
                })
            },
        )
    }
}
