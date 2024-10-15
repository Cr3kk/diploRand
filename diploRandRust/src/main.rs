use rand::seq::SliceRandom; 
use rand::thread_rng; 
use std::collections::{HashMap, HashSet};

fn main() {
    let names_previous_round = vec![
        ("Ben", "Turkey"),
        ("Casper", "Germany"),
        ("Niels", "Austria"),
        ("Koen", "Italy"),
        ("Lock", "England"),
        ("Jan-Jan", "Russia"),
        ("Wouter", "France"),
    ];

    let country_to_number: HashMap<&str, u32> = [
        ("France", 1),
        ("Russia", 2),
        ("Italy", 3),
        ("Germany", 4),
        ("England", 5),
        ("Austria", 6),
        ("Turkey", 7),
    ].iter().cloned().collect();

    let previous_round_set: HashSet<u32> = names_previous_round
        .iter()
        .filter_map(|&(_, country)| country_to_number.get(country).cloned())
        .collect();
    

    let mut rng = thread_rng();
    let mut random_array: Vec<u32> = country_to_number.values().cloned().collect();
    random_array.shuffle(&mut rng);

    random_array.retain(|num| !previous_round_set.contains(num));

    let required_count = names_previous_round.len() as u32;
    if random_array.len() < required_count.try_into().unwrap() {
        random_array = country_to_number.values().cloned().collect(); 
    }

    let number_to_country: HashMap<u32, &str> = country_to_number.iter()
        .map(|(&country, &number)| (number, country))
        .collect();

    let mut name_dict: HashMap<&str, &str> = HashMap::new();
    let names: Vec<&str> = names_previous_round.iter().map(|&(name, _)| name).collect();

    for (i, name) in names.iter().enumerate() {
        let country_number = random_array[i % random_array.len()]; 
        let country_name = number_to_country.get(&country_number).unwrap();
        name_dict.insert(name, country_name);
    }

    for (name, country) in name_dict.iter() {
        println!("name: {}, country: {}", name, country);
    }
}
