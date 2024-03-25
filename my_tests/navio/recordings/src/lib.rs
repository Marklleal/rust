mod people;

pub use people::register::People;

pub fn show_people(people: People) {
    println!("{:#?}", people);
}