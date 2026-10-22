use std::io;
use rand::RngExt;

fn main() {
    println!("Adivinhe o número!");

    // 1. Gera o número secreto entre 1 e 100
    let secret_number = rand::rng().random_range(1..=100);

    println!("Por favor, insira o seu palpite:");

    // 2. Cria a variável mutável 'guess' para armazenar a entrada do utilizador
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    // 3. Converte a string inserida num número inteiro (u32)
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido!");
            return;
        }
    };

    println!("You guessed: {guess}");

    // 4. Compara o palpite com o número secreto
    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Muito pequeno!"),
        std::cmp::Ordering::Greater => println!("Muito grande!"),
        std::cmp::Ordering::Equal => println!("Parabéns, você acertou!"),
    }
}