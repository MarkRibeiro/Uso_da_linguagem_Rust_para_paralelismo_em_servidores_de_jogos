extern crate tungstenite;
use pf2::ThreadPool;
use std::io;
use rand::Rng;
//use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};
use std::time::Duration;
use std::{net::TcpListener};
use std::sync::{Arc, Mutex, MutexGuard};
use tungstenite::{accept, Error, Message, WebSocket};
use std::env;
use std::process;

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
  match_time:u64,
  game_is_over:bool
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len()!=4 {
    println!("\nNumero incorreto de argumentos\nDeveria receber 3 e recebeu {}\ncargo run [largura_do_canvas] [altura_do_canvas] [tempo_da_partida]\n", args.len()-1);
    process::exit(0x0100);
  }

  let canvas_height = args[1].trim().parse::<usize>().unwrap();
  let canvas_width = args[2].trim().parse::<usize>().unwrap();
  let match_time = args[3].trim().parse::<u64>().unwrap();

  //let canvas_height = 10;
  //let canvas_width = 20;
  //let match_time = 45;

  let listener = TcpListener::bind("127.0.0.1:3012").unwrap();
  listener.set_nonblocking(true).expect("Can not set non-blocking");
  let pool = ThreadPool::new(100);

  let mut state = State{ players: vec![], map: vec![], canvas_height: canvas_height, canvas_width: canvas_width, match_time:match_time, game_is_over:false
  };

  state.map = create_map(state.canvas_height, state.canvas_width);
  let current_state = Arc::new(Mutex::new(state));
  let current_state_clone = current_state.clone();
  let sockets:Vec<Arc<Mutex<WebSocket<TcpStream>>>> = Vec::new();
  let sockets:Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>> = Arc::new(Mutex::new(sockets));
  let sockets_clone = sockets.clone();
  {
    let current_state = current_state.clone();
    pool.execute(move || { //Thread do estado
      loop {
        let response;
        //tempo de atualização do jogo
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
  }
  //for stream in listener.incoming()

  loop {
    if current_state.lock().unwrap().game_is_over == true{
      break
    }
    let stream =
    match listener.accept(){
      Ok((stream, _)) => stream,
      Err(_) => {

        continue
      }
    };
    //let stream = stream.unwrap();
    stream.set_nonblocking(true).unwrap();
    let websocket = match accept(stream) {
      Ok(x) => x,
      Err(_) => {
        //println!("Ignoring because {}", e);
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

    pool.execute(move || { //Thread dos jogadores
      handle_connection(websocket, current_state.clone());
    });
  }
  println!("Encerrando servidor");
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

fn handle_connection(websocket: Arc<Mutex<WebSocket<TcpStream>>>, current_state: Arc<Mutex<State>>){
  loop  {
    if current_state.lock().unwrap().game_is_over == true{
      break
    }
    let msg = websocket.lock().unwrap().read_message();
    match msg{
      Ok(conteudo) => {
        let copy_current_state = current_state.clone();
        _process_message(websocket.clone(), conteudo, copy_current_state);
      }
      Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::WouldBlock => {
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

  let mut websocket = websocket.lock().unwrap();
  let mut state = current_state.lock().unwrap();
  if info[0]=="conecta" {
    let new_id = state.players.len();
    let jogador = Player{
      name: info[1].to_string(),
      id: new_id,
      color: info[2].to_string(),
      posi: Point { x: rand::thread_rng().gen_range(0..state.canvas_width), y: rand::thread_rng().gen_range(0..state.canvas_height) },
      score: 0
    };
    state.map[jogador.posi.x][jogador.posi.y] = info[2].to_string();
    state.players.push(jogador);
    println!("numero de jogadores: {:?}", new_id+1);
    (*websocket).write_message(Message::Text(format!("{{\"id\":{},\"canvas_height\":{},\"canvas_width\":{},\"match_time\":{}}}", new_id, state.canvas_height, state.canvas_width, state.match_time))).unwrap();
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
  for remaining_time in (0..match_time+1).rev() {
    thread::sleep(time::Duration::from_secs(1));
    send_remaining_time(remaining_time, sockets.clone())
  }

  println!("Tempo acabou!");
  current_state.lock().unwrap().game_is_over = true;
  let winner = find_winner(current_state);
  println!("Vencedor: {}", winner);
  send_winner(winner, sockets.clone());
}

fn send_remaining_time(remaining_time:u64, sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>>){
  for websocket in &*sockets.lock().unwrap() {
    let res = (websocket.lock().unwrap()).write_message(Message::Text(format!("{{\"tr\":\"{}\"}}", remaining_time.to_string())));
    res.map_err(|err| println!("{:?}", err)).ok();
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
    let res = (websocket.lock().unwrap()).write_message(Message::Text(format!("{{\"vencedor\":\"{}\"}}", winner_id.to_string())));
    res.map_err(|err| println!("{:?}", err)).ok();
  }
}