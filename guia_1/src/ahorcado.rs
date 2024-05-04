use std::fs::File;
use std::io::{self, BufRead};
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
    let mut tries: usize = 0;
    // For each word
    let word = &words[0];
    let mut letters: Vec<char> = Vec::new();
    let mut user_progress: Vec<char> = Vec::new();
    let mut user_word: String = String::new();
    let mut check;

    for l in word.chars() {
        letters.push(l);
    }

    println!("Ingresa las letras faltantes");
    println!("_ _ _ _");

    loop {
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("ERROR leyendo la línea");

        check = letters
            .iter()
            .find(|&&l| l == user_input.chars().next().unwrap());

        if let Some(position) = word.find(user_input.chars().next().unwrap()) {
            println!("La letra está en posición {}", position);
            user_progress.push(user_input.chars().next().unwrap());
            user_word = user_progress.clone().into_iter().collect();
        } else {
            println!("------Error------");
            println!("Intentos restantes: {} ", 5 - tries);
        }

        println!("{}", user_word);

        if &user_word == word {
            println!("Ganaste! \nLa palabra es: {}", word);
            break;
        }

        if tries == 4 {
            println!("Perdiste, volvé a intentarlo");
            break;
        }

        tries += 1;
    }
}

fn find_position(word: &str, character: char) -> Vec<usize> {}
