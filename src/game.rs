use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;
use ggez::input::keyboard::{self, KeyCode}; // Adicionado aqui
use crate::paddle::Paddle;

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED_X: f32 = 200.0;
const BALL_SPEED_Y: f32 = 150.0;

pub struct Game {
    paddle1: Paddle,
    paddle2: Paddle,
    screen_width: f32,
    screen_height: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        // Obtém as dimensões da tela
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Configura as raquetes
        let paddle1 = Paddle::new(30.0, screen_height / 2.0 - PADDLE_HEIGHT / 2.0, PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_SPEED);
        let paddle2 = Paddle::new(
            screen_width - 40.0,
            screen_height / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            PADDLE_SPEED,
        );

        // Configura a bola
        let ball = Ball::new(
            screen_width / 2.0,
            screen_height / 2.0,
            BALL_RADIUS,
            BALL_SPEED_X,
            BALL_SPEED_Y,
        );

        // Retorna a instância de Game
        Self {
            paddle1,
            paddle2,
            screen_width,
            screen_height,
        }
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ggez::timer::delta(ctx).as_secs_f32();

        // Controles para a raquete 1
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.paddle1.move_up(delta);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.paddle1.move_down(delta, self.screen_height);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Desenha as raquetes
        self.paddle1.draw(ctx)?;
        self.paddle2.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
