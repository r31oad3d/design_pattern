// same when use CustomerResult
pub trait AbstractCustomer {
    fn is_null(&self) -> bool;
    fn get_name(&self) -> &str;
}
impl AbstractCustomer for Result<RealCustomer, &str> {
    fn is_null(&self) -> bool {
        self.is_ok()
    }

    fn get_name(&self) -> &str {
        match self {
            Result::Ok(customer) => customer.name.as_str(),
            Result::Err(_) => "Not Available in Customer Database",
        }
    }
}

pub struct RealCustomer {
    name: String,
}

pub struct CustomerFactory {}

impl CustomerFactory {
    pub fn get_customer(name: &str) -> Result<RealCustomer, &str> {
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
            || Result::Err("Not Available in Customer Database"),
            |&n| {
                Result::Ok(RealCustomer {
                    name: String::from(n),
                })
            },
        )
    }
}
