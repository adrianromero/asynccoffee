//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use rand::rngs::StdRng;
use rand_distr::{Distribution, Uniform};

const NAMES: &[&str] = &[
    "Elisa",
    "Mateo",
    "Isaac",
    "Luz",
    "Pablo",
    "Elena",
    "Sofía",
    "Isabella",
    "Salvador",
    "María",
    "Roberto",
    "Natalia",
    "Mercedes",
    "Samantha",
    "Jimena",
    "Daniela",
    "Josefina",
    "Soledad",
    "Carmen",
    "Alejandro",
];
const SURNAMES: &[&str] = &[
    "Ruiz",
    "Hernandez",
    "Díaz",
    "Moreno",
    "Muñoz",
    "Álvarez",
    "Romero",
    "Alonso",
    "Gutiérrez",
    "Navarro",
    "Torres",
    "Domínguez",
    "Vázquez",
    "Ramos",
    "Gil",
    "Ramírez",
    "Serrano",
    "Blanco",
    "Molina",
    "Morales",
    "Suarez",
    "Ortega",
];

pub fn generate_name(rng: &mut StdRng) -> String {
    let uninames: Uniform<usize> = Uniform::new(0, NAMES.len());
    let unisurnames: Uniform<usize> = Uniform::new(0, SURNAMES.len());

    format!(
        "{} {}",
        NAMES[uninames.sample(rng)],
        SURNAMES[unisurnames.sample(rng)]
    )
}
