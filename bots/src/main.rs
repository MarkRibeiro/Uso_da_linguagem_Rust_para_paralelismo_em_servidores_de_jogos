use rand::Rng;

fn main() {
    let mut number_of_bots = String::new();
    let mut port_number = String::new();
    println!("Quantos jogadores voce quer simular? ");
    std::io::stdin().read_line(&mut number_of_bots).unwrap();
    let number_of_bots = number_of_bots.trim().parse::<i32>().unwrap();

    println!("Qual a porta desejada? ");
    std::io::stdin().read_line(&mut port_number).unwrap();
    let port_number = port_number.trim().parse::<i32>().unwrap();

    while true {
        next_moviment();
    }
}

fn next_moviment() -> i32 {
    let mut x = rand::thread_rng().gen_range(0..5);
    if x == 0 {
        println!("Cima");
    }
    if x == 1 {
        println!("Baixo");
    }
    if x == 2 {
        println!("Esquerda");
    }
    if x == 3 {
        println!("Direita");
    }
    if x == 4 {
        println!("Parado");
    }
    return x;
}