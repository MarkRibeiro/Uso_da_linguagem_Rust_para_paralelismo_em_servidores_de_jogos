let aWebSocket = new WebSocket("ws://127.0.0.1:3012");
let canvas = document.getElementById("Canvas");
let contcanvas = canvas.getContext("2d");
let pontuacao = document.getElementById("Pontuacao");
let loginDiv = document.getElementById("login");
let gameDiv = document.getElementById("game");

loginDiv.style.display = "none"

let posi = {x:0, y:0};
let largura = 1000;
let altura = 500;
let tamCelula = 50;
let tamJogador = 1;
let cor = "green";
let piscar = 0;
let piscadaTempo = 10;
let estado;
let id;
let jogadores;

aWebSocket.onopen = function(event) {
    console.log("WebSocket is open now.");
    aWebSocket.send("conecta;" + id + ";" + cor + ";" + posi.x + ";" + posi.y);
    contcanvas.beginPath();
    contcanvas.fillStyle = cor;
    contcanvas.rect(posi.x*tamCelula, posi.y*tamCelula, tamCelula, tamCelula);
    contcanvas.fill();
};

window.requestAnimationFrame(atualizaCanvas);

aWebSocket.onmessage = function(event) {
    estado = JSON.parse(event.data);
    if (estado.id !== undefined) {
        id = estado.id
        sessionStorage.setItem("id", estado.id)
    }
    //let id = sessionStorage.getItem("id")
    console.log(estado);
    console.log(id);
    jogadores = estado.jogadores

    pontuacao.innerHTML = "Pontuação: " +  estado.jogadores[id].pontuacao;

};

function atualizaCanvas() {
    if (!estado) {
        window.requestAnimationFrame(atualizaCanvas);
        return
    }

    let mapa = estado.mapa

    contcanvas.fillStyle = "white"
    contcanvas.beginPath();
    contcanvas.rect(0, 0, largura, altura);
    contcanvas.fill();

    let x = 0
    for(let coluna of mapa){
        let y = 0
        for(let corQuadrado of coluna){
            contcanvas.fillStyle = corQuadrado;
            contcanvas.beginPath();
            contcanvas.rect(x*tamCelula, y*tamCelula, tamCelula, tamCelula);
            contcanvas.fill();
            y+=1
        }
        x+=1
    }
    if (jogadores == undefined){
        window.requestAnimationFrame(atualizaCanvas);
        return
    }

    for (let jogador of jogadores) {
        contcanvas.fillStyle = "green";
        contcanvas.strokeStyle = "black";
        contcanvas.beginPath();
        contcanvas.rect(jogador.x*tamCelula, jogador.y*tamCelula, tamCelula, tamCelula);
        contcanvas.fill();
        contcanvas.stroke();

        let z = tamCelula/4;
        let j = tamCelula/5;
        contcanvas.fillStyle = "black";

        if (jogador.blinking) {
            contcanvas.beginPath();
            contcanvas.rect(jogador.x*tamCelula+tamCelula/2-z/2 - j, jogador.y*tamCelula+tamCelula/4 + 3*z/4, z, z/4);
            contcanvas.rect(jogador.x*tamCelula+tamCelula/2-z/2 + j, jogador.y*tamCelula+tamCelula/4+ 3*z/4, z, z/4);
            contcanvas.fill();
            piscar -= 1;
        }
        else {
            contcanvas.beginPath();
            contcanvas.rect(jogador.x*tamCelula+tamCelula/2-z/2 - j, jogador.y*tamCelula+tamCelula/4, z, z);
            contcanvas.rect(jogador.x*tamCelula+tamCelula/2-z/2 + j, jogador.y*tamCelula+tamCelula/4, z, z);
            contcanvas.fill();
        }
    }
        //console.log(event.data);
    window.requestAnimationFrame(atualizaCanvas);
}


function teclaPressionada(evento) {
    for(let jogador of jogadores) {
        console.log(jogador.id, id);
        if(jogador.id == id) {
            if(event.key == "ArrowUp" && jogador.y-1>= 0){
                aWebSocket.send("atualiza;" + id +";cima");
            }

            if(event.key == "ArrowDown" && jogador.y+1+tamJogador <= altura/tamCelula){
                aWebSocket.send("atualiza;" + id +";baixo");
            }

            if(event.key == "ArrowLeft" && jogador.x-1 >= 0){
                aWebSocket.send("atualiza;" + id +";esquerda");
            }

            if(event.key == "ArrowRight" && jogador.x+1+tamJogador <= largura/tamCelula){
                aWebSocket.send("atualiza;" + id +";direita");
            }
            if(event.key == " " ){
                aWebSocket.send("pinta;" + id +";" + jogador.x + ";" + jogador.y);
                piscar = piscadaTempo;
            }
            aWebSocket.send("atualiza;" + id + ";" + cor + ";" + jogador.x + ";" + jogador.y);
        }
    }
}
document.addEventListener("keydown", teclaPressionada);
