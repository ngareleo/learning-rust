mod pages;
mod cakes_inn;


use cakes_inn::models::customer::Customer;

fn main() {
    let customer = Customer::new(String::from("Leo"), 345353);
    let second_customer = Customer::new(String::from("Ngare"), 332535);
    let second_customer = second_customer.update_customer_name(String::from("Sugar"));
    // The following line won't compile because fields are private by default
    // customer.name = "Ngare";
    dbg!("{}", customer);
    dbg!("{}", second_customer);
}