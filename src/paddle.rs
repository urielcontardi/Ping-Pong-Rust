use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::{Context, GameResult};

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32, // Velocidade de movimento
}

impl Paddle {
    // Construtor da raquete
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    // Move a raquete para cima
    pub fn move_up(&mut self, delta: f32) {
        self.y -= self.speed * delta;
        if self.y < 0.0 {
            self.y = 0.0; // Impede que a raquete saia da tela
        }
    }

    // Move a raquete para baixo
    pub fn move_down(&mut self, delta: f32, screen_height: f32) {
        self.y += self.speed * delta;
        if self.y + self.height > screen_height {
            self.y = screen_height - self.height; // Impede que a raquete saia da tela
        }
    }

    // Desenha a raquete na tela
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        let paddle = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::WHITE)?;
        graphics::draw(ctx, &paddle, DrawParam::default())
    }
}
