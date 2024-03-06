use std::io;
use rand::Rng;

fn main() {
    println!("Devinez le nombre !");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Le nombre secret est : {}", secret_number);

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}