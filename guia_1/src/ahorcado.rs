use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Write};
use std::path::Path;
use std::vec::Vec;

pub fn game() {
    let file_path = "./resources/ahorcado.txt";
    let mut words: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            words.push(line);
        }
    }

    println!("Bienvenido al rusty ahorcado");

    compare_words(words);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compare_words(words: Vec<String>) {
    // For each word
    let word = &words[0];
    let mut letters: Vec<char> = Vec::new();
    let mut user_progress: Vec<char> = Vec::new();
    let mut user_word: Vec<String> = Vec::new();

    for l in word.chars() {
        letters.push(l);
    }

    println!("{} - {} -", letters[0], letters[2]);

    println!("Ingresa las letras faltantes");

    loop {
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("ERROR leyendo la línea");

        println!(
            "{:?}",
            letters
                .iter()
                .find(|&&l| l == user_input.chars().next().unwrap())
        );

        user_progress.push(user_input.chars().next().unwrap());

        for c in &user_progress {
            println!("{}", c);
        }

        // if user_input.chars().next().unwrap() == letters[0] {
        //     println!("Correcto");
        //     println!("{} {} {} -", letters[0], letters[1], letters[2]);
        // } else {
        //     println!("Prueba de nuevo");
        //     break;
        // }
    }
}
