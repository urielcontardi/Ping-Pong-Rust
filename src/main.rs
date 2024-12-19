mod game;
mod paddle;
mod ball;
mod utils;
mod robot;

use ggez::{ContextBuilder, GameResult};
use game::Game;

fn main() -> GameResult {
    // Configuração do contexto e do loop de eventos
    let (mut ctx, event_loop) = ContextBuilder::new("pongGame", "Uriel")
        .window_setup(ggez::conf::WindowSetup {
            title: "Ping Pong".to_owned(), // Alterando o título da janela
            ..Default::default() // Mantém as outras configurações padrão
        })
        .build()
        .expect("Falha ao criar o contexto do jogo");

    // Inicializa o jogo
    let game = Game::new(&mut ctx);

    // Inicia o loop principal
    ggez::event::run(ctx, event_loop, game)
}
