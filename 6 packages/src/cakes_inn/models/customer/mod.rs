#[derive(Debug)]
pub struct Customer {
    name: String,
    id_number: u32,
}

impl Customer {
    pub fn new(name: String, id_number: u32) -> Customer{
        Customer{
            name,
            id_number
        }
    }

    pub fn update_customer_name(mut self, new_name: String) -> Self{
        self.name = new_name;
        self
    }
}

pub mod dish {
    pub struct Dish {
        name: String,
    }
}