use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// TODO: Check if word already entered
// TODO: Repeat with all the words of the list

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
    let mut tries: usize = 0;
    let mut letters: Vec<char> = Vec::new();
    let mut user_progress: Vec<char> = vec![];
    user_progress.resize(word.len(), '_');
    let mut user_word: String = String::new();

    for l in word.chars() {
        letters.push(l);
    }

    println!("Ingresa las letras faltantes");
    println!("{:?}", user_progress);

    loop {
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("ERROR leyendo la línea");

        if let Some(position) = word.find(user_input.chars().next().unwrap()) {
            let word_index = find_position(word, user_input.chars().next().unwrap());
            if word_index.len() > 1 {
                let mut i: usize = 0;
                while i < word_index.len() {
                    user_progress.insert(word_index[i], user_input.chars().next().unwrap());
                    i += 1;
                }
            } else {
                user_progress.insert(word_index[0], user_input.chars().next().unwrap());

                let _ = user_progress.remove(position + 1);
            }
            user_progress.truncate(word.len());

            user_word = user_progress.clone().into_iter().collect();

            println!("Intentos restantes: {} ", tries);
        } else {
            println!("------Error------");
            println!("Intentos restantes: {} ", tries);
        }

        // user_progress.retain(|x| *x != '_');

        //println!("{:?}", user_progress);

        println!("{}", user_word);
        user_word.truncate(word.len());

        if user_word.trim() == word {
            println!("Ganaste! \nLa palabra es: {}", word);
            break;
        }

        if tries == word.len() + 2 {
            println!("Perdiste, volvé a intentarlo");
            break;
        }

        tries += 1;
    }
}

fn find_position(word: &str, character: char) -> Vec<usize> {
    let mut positions = Vec::new();

    for (pos, c) in word.char_indices() {
        if c == character {
            positions.push(pos);
        }
    }

    positions
}
