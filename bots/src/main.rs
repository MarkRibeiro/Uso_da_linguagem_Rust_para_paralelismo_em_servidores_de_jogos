use std::net::TcpStream;
use rand::Rng;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;

fn main() {
    let id = 0;
    let mut number_of_bots = String::new();
    let mut port_number = String::new();
    println!("Quantos jogadores voce quer simular? ");
    std::io::stdin().read_line(&mut number_of_bots).unwrap();
    let number_of_bots = number_of_bots.trim().parse::<i32>().unwrap();

    println!("Qual a porta desejada? ");
    std::io::stdin().read_line(&mut port_number).unwrap();

    let (mut socket, response) =
        //connect(format!("ws://127.0.0.1:{}", port_number.trim())).expect("Can't connect");
        connect("ws://127.0.0.1:3012").expect("Can't connect");
    socket.write_message(Message::Text(format!("conecta;{};yellow;0;0", id))).unwrap();


    loop {
        next_movement(&mut socket, id);
        thread::sleep(Duration::from_millis(500));
    }
}

fn next_movement(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, id: i32) {
    let mut x = rand::thread_rng().gen_range(0..5);
    let mut direction = "";
    if x == 0 { //cima
        println!("cima");
        direction = "cima";
    }
    if x == 1 { //baixo
        println!("baixo");
        direction = "baixo";
    }
    if x == 2 { //esquerda
        println!("esquerda");
        direction = "esquerda";
    }
    if x == 3 { //direita
        println!("direita");
        direction = "direita";
    }
    if x == 4 { //parado
        socket.write_message(Message::Text(format!("pinta;{};0;0", id).into())).unwrap();
        return;
    }
    socket.write_message(Message::Text(format!("atualiza;{};{}", id, direction).into())).unwrap();
    socket.write_message(Message::Text(format!("pinta;{};0;0", id).into())).unwrap();
}