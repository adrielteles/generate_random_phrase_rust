use rand::Rng;
use std::io;

fn main() -> ! {
    //tupla

    let articles: [&str; 8] = ["o","os","a","as","um","uns","uma","umas"];
    let subjects: [&str; 8] = ["cachorro", "gato", "homem", "elefante", "rato", "cavalo", "pato", "papagaio"];
    let verbs: [&str; 8] = ["corre", "pula", "anda", "voa", "nada", "dorme", "come", "brinca"];
    let adverbs: [&str; 8] = ["bem", "mal", "rápido", "devagar", "muito","nascer","viver","proceder"];

    let sentence: &[&[[&str; 8]; 4]; 2] = &[
        &[articles, subjects, verbs, adverbs],
        &[articles, subjects, verbs, adverbs],
    ];
    loop {
        println!("numero de frases: ");
        let mut number_of_sentences = String::new();
    
        io::stdin()
            .read_line(&mut number_of_sentences)
            .expect("Falha ao ler a linha.");
    
        let number_of_sentences: u32 = match number_of_sentences.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("numero de frases: {}", number_of_sentences);
    
        let mut lines = 0;
        while lines < number_of_sentences {
            let sentence_type = &sentence[ rand::thread_rng().gen_range(0..sentence.len()) ];
            let mut line = String::new();
            let mut column = 0;
    
            while column < sentence_type.len() {
                line += sentence_type[column][ rand::thread_rng().gen_range(0..8) ];
                line += " ";
                column += 1;
            }
            println!("{}", line);
            lines += 1;
        }
    }
} 