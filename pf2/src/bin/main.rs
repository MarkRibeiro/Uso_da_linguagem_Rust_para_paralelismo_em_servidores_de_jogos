extern crate tungstenite;
use pf2::ThreadPool;
use std::{env, io};
use rand::Rng;
//use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};
use std::time::Duration;
use std::{net::TcpListener};
use std::sync::{Arc, Mutex, MutexGuard};
use tungstenite::{accept, Error, Message, WebSocket};

struct Point {
  x:usize,
  y:usize
}

struct Player{
  name: String,
  id : usize,
  color: String,
  posi: Point,
  score: u32
}

struct State {
  players: Vec<Player>,
  map: Vec<Vec<String>>,
  canvas_height:usize,
  canvas_width:usize,
}
//static height:usize = 10;
//static width:usize = 20;
//let match_time:u64 = 120;

fn main() {
  /*
  let args: Vec<String> = env::args().collect();

  let canvas_height = args[1].trim().parse::<usize>().unwrap();
  let canvas_width = args[2].trim().parse::<usize>().unwrap();
  let match_time:u64 = args[3].trim().parse::<u64>().unwrap();*/

  let canvas_height = 10;
  let canvas_width = 20;
  let match_time:u64 = 120;

  let listener = TcpListener::bind("127.0.0.1:3012").unwrap();
  let pool = ThreadPool::new(40);

  let mut state = State{ players: vec![], map: vec![],canvas_height: canvas_height,  canvas_width: canvas_width};
  let mut game_is_over= Arc::new(Mutex::new(false));

  state.map = create_map(state.canvas_height, state.canvas_width);
  let current_state = Arc::new(Mutex::new(state));
  let current_state_clone = current_state.clone();
  let sockets:Vec<Arc<Mutex<WebSocket<TcpStream>>>> = Vec::new();
  let sockets:Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>> = Arc::new(Mutex::new(sockets));
  let sockets_clone = sockets.clone();
  pool.execute(move || {
    loop {
      //println!("oi");
      let response;
      thread::sleep(Duration::from_millis(100));
      {
        let state = current_state.clone();
        response = build_response(false, &mut state.lock().unwrap());
      }
      let sockets = sockets_clone.clone();
      let sockets = &*sockets.lock().unwrap();
      for socket in sockets {
        let mut socket = socket.lock().unwrap();
        let _ = (*socket).write_message(Message::Text(response.clone()));
      }
    }
  });
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    stream.set_nonblocking(true).unwrap();
    let websocket = match accept(stream) {
      Ok(x) => x,
      Err(e) => {
        println!("Ignoring because {}", e);
        continue
      }
    };
    let websocket =  Arc::new(Mutex::new(websocket));
    let current_state = current_state_clone.clone();
    let current_state_clone2 = current_state_clone.clone();
    {
      let sockets = sockets.clone();
      let socket_clone = sockets.clone();
      let mut sockets = sockets.lock().unwrap();
      sockets.push(websocket.clone());

      //começa corrotina do contdown
      if current_state_clone.clone().lock().unwrap().players.len() == 0 {
        pool.execute(move || {
          countdown(match_time, socket_clone.clone(), current_state_clone2.clone());
        });
      }
    }

    pool.execute(move || {
      handle_connection(websocket, current_state.clone());
    });



  }

  println!("Shutting down.");
}

fn create_map(height:usize, width:usize) -> Vec<Vec<String>> {

  let mut matrix = vec![];
  let mut vector = vec![];
  for _ in 0..height {
    vector.push("white".to_string());
  }
  for _ in 0..width {
    matrix.push(vector.clone());
  }
  return matrix;
}

/*
fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "hello.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
*/

fn handle_connection(websocket: Arc<Mutex<WebSocket<TcpStream>>>, current_state: Arc<Mutex<State>>){
  println!("Conectei");
  loop {
    let msg = websocket.lock().unwrap().read_message();
    match msg{
      Ok(conteudo) => {
        let copy_current_state = current_state.clone();
        /*{
            let mut state = current_state.lock().unwrap();
            *state = msg.clone();
        }*/
        _process_message(websocket.clone(), conteudo, copy_current_state);
      }
      Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::WouldBlock => {
        //println!("WouldBlock");
        continue;
      },
      Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::ConnectionReset => {
        println!("Connection reset");
      },
      Err(Error::AlreadyClosed) => {
        println!("Conexão encerrada");
        break;
      },
      Err(Error::ConnectionClosed) => {
        println!("Conexão encerrada");
        break;
      },
      Err(e) => panic!("encountered IO error: {e}")
    }
  }
}

fn _process_message(websocket: Arc<Mutex<WebSocket<TcpStream>>>, message:Message,
                          current_state: Arc<Mutex<State>>){ //responde o cliente
  let msg = message.to_string();
  let info:Vec<&str> = msg.split(";").collect();
  //print com mensagens recebidas dos clientes
  //println!("{:?}", info);

  let mut websocket = websocket.lock().unwrap();
  let mut state = current_state.lock().unwrap();
  if info[0]=="conecta" {
    println!("Novo Jogador");
    let newID = state.players.len();
    let jogador = Player{
      name: info[1].to_string(),
      id: newID,
      color: info[2].to_string(),
      posi: Point { x: rand::thread_rng().gen_range(0..state.canvas_width), y: rand::thread_rng().gen_range(0..state.canvas_height) },
      score: 0
    };
    state.players.push(jogador);
    println!("numero de jogadores: {:?}", newID);
    (*websocket).write_message(Message::Text(format!("{{\"id\":{}}}", newID))).unwrap();
  }
  if info[0]=="atualiza" {
    let id = info[1].parse::<usize>().unwrap();
    if info[2] == "cima" && state.players[id].posi.y >= 1 {
      state.players[id].posi.y -= 1;
    }
    if info[2] == "baixo" && state.players[id].posi.y + 1 < state.canvas_height {
      state.players[id].posi.y += 1;
    }
    if info[2] == "esquerda" && state.players[id].posi.x >= 1 {
      state.players[id].posi.x -= 1;
    }
    if info[2] == "direita" && state.players[id].posi.x + 1 < state.canvas_width {
      state.players[id].posi.x += 1;
    }
    let x = state.players[id].posi.x;
    let y = state.players[id].posi.y;

    state.map[x][y] = state.players[id].color.to_string();

  }

  for mut jogador in &mut state.players{
    jogador.score = 0;
  }

  for coluna in state.map.clone(){
    for cor in coluna{
      for mut jogador in &mut state.players{
        if jogador.color == *cor {
          jogador.score += 1;
        }
      }
    }
  }

  let ret = build_response(info[0] == "conecta", &mut state);

  let _ = (*websocket).write_message(Message::Text(ret.clone()));

  //Print com informaçoes do mapa
  //println!("{:?}", ret);

}

fn build_response(is_connected: bool, state: &mut MutexGuard<State>) -> String {
  let mut ret = String::from("{\"jogadores\" : [");
  let mut count = 0;
  for jogador in &state.players {
    let aaa = format!("{{\"id\":\"{}\" ,\"cor\":\"{}\" , \"x\":{} , \"y\":{} , \"pontuacao\":{}}}",
                      jogador.id, jogador.color, jogador.posi.x, jogador.posi.y, jogador.score);
    ret = ret + &aaa;

    if count < state.players.len() - 1 {
      ret.push(',');
    }
    count = count + 1
  }
  ret = ret + &format!("], \"mapa\":{:?}", state.map);

  if is_connected {
    ret = ret + &format!(",\"id\": {}", state.players.len() - 1);
  }

  ret = ret + "}";
  ret
}

fn countdown(match_time:u64, sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>>, current_state: Arc<Mutex<State>>) {
  let two_minutes = time::Duration::from_secs(match_time+1);

  for remaining_time in (0..match_time+1).rev() {
    println!("{} seconds remaining", remaining_time);
    thread::sleep(time::Duration::from_secs(1));
    send_remaining_time(remaining_time, sockets.clone())
  }

  println!("Time's up!");
  let winner = find_winner(current_state);
  send_winner(winner, sockets.clone())
  //game_is_over = true;
}

fn send_remaining_time(remaining_time:u64, sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>>){
  for websocket in &*sockets.lock().unwrap() {
    (websocket.lock().unwrap()).write_message(Message::Text(format!("{{\"tr\":\"{}\"}}", remaining_time.to_string())));
  }
}

fn find_winner(current_state: Arc<Mutex<State>>) -> String {
  let mut score_vencedor = current_state.lock().unwrap().players[0].score;
  let mut vencedor = current_state.lock().unwrap().players[0].name.clone();

  for player in &current_state.lock().unwrap().players {
    if player.score > score_vencedor {
      score_vencedor = player.score;
      vencedor = player.name.clone();
    }
  }
  return vencedor;
}

fn send_winner(winner_id: String, sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>>){
  for websocket in &*sockets.lock().unwrap() {
    (websocket.lock().unwrap()).write_message(Message::Text(format!("{{\"vencedor\":\"{}\"}}", winner_id.to_string())));
  }
}