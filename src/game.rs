use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;
use ggez::input::keyboard::{self, KeyCode}; // Adicionado aqui
use crate::paddle::Paddle;

pub struct Game {
    paddle1: Paddle,
    paddle2: Paddle,
    screen_width: f32,
    screen_height: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        Self {
            paddle1: Paddle::new(30.0, screen_height / 2.0 - 50.0, 10.0, 100.0, 300.0),
            paddle2: Paddle::new(screen_width - 40.0, screen_height / 2.0 - 50.0, 10.0, 100.0, 300.0),
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
