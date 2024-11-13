use std::io;

fn main() {
    println!("Jogo Advinha o número");

    println!("Entre com um número");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("falhou em ler a linha");

    let guess : u32 = guess.trim().parse().expect("expected a number");

        println!("Você advinhou: {}", guess);

    let secret_number = 10;

    match guess.cmp(&secret_number){
        std::cmp::Ordering::Less => println!("Menor"),
        std::cmp::Ordering::Greater => println!("Maior"),
        std::cmp::Ordering::Equal => println!("Acertou mizeravi!")
    }
}
