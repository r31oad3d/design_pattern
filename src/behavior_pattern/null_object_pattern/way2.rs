// same when use Result
pub enum CustomerResult {
    Found(RealCustomer),
    NotFound(NullCustomer),
}

impl AbstractCustomer for CustomerResult {
    fn is_null(&self) -> bool {
        match self {
            CustomerResult::Found(_) => true,
            CustomerResult::NotFound(_) => false,
        }
    }

    fn get_name(&self) -> &str {
        match self {
            CustomerResult::Found(customer) => customer.name.as_str(),
            CustomerResult::NotFound(_) => "Not Available in Customer Database",
        }
    }
}

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
    pub fn get_customer(name: &str) -> CustomerResult {
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
            || CustomerResult::NotFound(NullCustomer {}),
            |&n| {
                CustomerResult::Found(RealCustomer {
                    name: String::from(n),
                })
            },
        )
    }
}
