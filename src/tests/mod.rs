use fake::{Fake, Faker};

pub mod mail;

pub fn get_random_string() -> String {
    Faker.fake::<String>()
}