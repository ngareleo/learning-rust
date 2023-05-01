#![allow(unused)]

use ::std::collections::HashMap;


trait Weight {
    fn to_kg(&self) -> f32;
    fn to_lb(&self) -> f32;
}


#[derive(Debug)]
struct KgWeight {
    kg: f32,
}

impl Weight for KgWeight {
    fn to_kg(&self) -> f32 {
        self.kg
    }

    fn to_lb(&self) -> f32 {
        self.kg * 2.20462
    }
}

#[derive(Debug)]
struct LbWeight {
    lb: f32,
}

impl Weight for LbWeight   {
    fn to_kg(&self) -> f32 {
        self.lb * 0.453592
    }

    fn to_lb(&self) -> f32 {
        self.lb
    }
}

#[derive(Debug)]
enum WeightType {
    Kg(KgWeight),
    Lb(LbWeight),
}

enum CommChannel {
    Email,
    Sms,
    All,    
}


#[derive(Debug)]
struct Person  {
    name: String,
    phone_number: Option<String>,
    age: u8,
    email: String,
    weight: WeightType,
}

impl<'a> Person{

    fn new(name: String, age: u8, email: String, phone_number: Option<String>, weight: WeightType) -> Self{
        Person {
            name,
            phone_number,
            age,
            email,
            weight,
        }
    }

    fn send_email(&self, message: &str) {
        println!("Sending email to {} with message {}", self.email, message);
    }

    fn send_sms(&self, message: &str) {
        match &self.phone_number {
            Some(pn) => println!("Sending sms to {} with message {}", self.email, message),
            None => println!("Cannot send sms, no phone number")
        }
        
    }

    fn send_current_weight(&self, channel: CommChannel) {
        let weight_in_kg = self.get_weight_in_kg();
        let weight_in_lb = self.get_weight_in_lb();
        let message = format!("Your current weight is {}kg {}lb", weight_in_kg, weight_in_lb);
        match channel {
            CommChannel::Email => self.send_email(message.as_str()),
            CommChannel::Sms => self.send_sms(message.as_str()),
            CommChannel::All => {
                self.send_email(message.as_str());
                self.send_sms(message.as_str());
            }
        }
        
    }

    fn change_email_address(&mut self, new_email: String) {
        self.email = new_email;
    }

    fn get_weight_in_kg(&self) -> f32{
        match self.weight {
            WeightType::Kg(ref weight) => weight.to_kg(),
            WeightType::Lb(ref weight) => weight.to_kg()
        }
    }

    fn get_weight_in_lb(&self) -> f32{
        match self.weight {
            WeightType::Kg(ref weight) => weight.to_lb(),
            WeightType::Lb(ref weight) => weight.to_lb()
        }
    }


}

fn main() {
    let a = 23;
    let b = 30;
    let maximum = max(&a, &b);
    println!("{maximum}");

    let c = 23;
    let maximum_num;
    let d = 40;
    {
        // if we declared d here, it would cause a panic
        // d will have a short lifetime compared to a and max2
        // d's lifetime ends at the end of this block
        maximum_num = max(&c, &d);
    }
    println!("{maximum_num}");

    let mut user1 = Person::new(String::from("John"), 23, String::from("john@gmail.com"), Some(String::from("09888765")), WeightType::Lb(LbWeight { lb: 2.2 }));
    let mut user2 = Person::new(String::from("Jane"), 22, String::from("jane@gmail.com"), None, WeightType::Kg(KgWeight { kg: 65.2 }));

    user1.change_email_address(String::from("john2@gmail.com"));
    user1.send_current_weight(CommChannel::All);
    user2.send_current_weight(CommChannel::All);

}

fn max<'a, T: std::cmp::PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    // we added the lifetime 'a to the function signature
    // inidicates that both a and b are valid if they are valid for the lifetime 'a
    // just tells the compiler to relax since all the references are valid for the same lifetime
    if a > b {
        a
    } else {
        b
    }
}

// applying generics, lifetimes and traits
