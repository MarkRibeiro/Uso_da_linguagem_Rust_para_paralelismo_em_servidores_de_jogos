use std::net::TcpStream;
use rand::Rng;
use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use bots::ThreadPool;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1 = &args[1];
    let arg_2 = &args[2];

    let number_of_bots = arg_1.trim().parse::<i32>().unwrap();
    let port_number = arg_2;

    let pool = ThreadPool::new(number_of_bots as usize);

    for _ in 0..number_of_bots {
        let (mut socket, response) = loop {
            match connect(format!("ws://127.0.0.1:{}", port_number.trim())) {
                Ok((socket, response)) => {
                    break (socket, response);
                },
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        };

        pool.execute(move || {
            //random hexadecimal
            socket.write_message(Message::Text(format!("conecta;{};{};0;0", -1, random_hex()))).unwrap();
            let msg = socket.read_message().unwrap();
            // let info:Vec<&str> = msg.to_string().split(";").collect();
            let bot_id = msg.to_string().trim().parse::<i32>().unwrap();
            println!("Bot {} trabalhando", bot_id);
            loop {
                next_movement(&mut socket, bot_id);
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    thread::sleep(Duration::from_secs(10));
}

fn next_movement(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, bot_id: i32) {
    let x = rand::thread_rng().gen_range(0..5);
    let mut direction = "";
    match x {
        0 => {
            direction = "cima"
        },

        1 => {
            direction = "baixo"
        },

        2 => {
            direction = "esquerda"
        },

        3 => {
            direction = "direita"
        },

        _ => {
        },
    }
    //println!("{}", direction);
    socket.write_message(Message::Text(format!("atualiza;{};{}", bot_id, direction).into())).unwrap();
    socket.write_message(Message::Text(format!("pinta;{}", bot_id).into())).unwrap();
}

fn random_hex() -> String {
    let hex = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let mut num = String::from("#");

    let mut x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    return num;
}