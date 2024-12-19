use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::{Context, GameResult};

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Ball {
    // Construtor para criar uma nova bola
    pub fn new(x: f32, y: f32, radius: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            x,
            y,
            radius,
            velocity_x,
            velocity_y,
        }
    }

    // Atualiza a posição da bola
    pub fn update(&mut self, delta: f32) {
        self.x += self.velocity_x * delta;
        self.y += self.velocity_y * delta;
    }

    // Detecta colisão com as bordas e inverte a direção
    pub fn check_collision(&mut self, screen_width: f32, screen_height: f32) {
        // Colisão com as bordas superior e inferior
        if self.y - self.radius <= 0.0 || self.y + self.radius >= screen_height {
            self.velocity_y = -self.velocity_y;
        }

        // Colisão com as bordas esquerda e direita
        if self.x - self.radius <= 0.0 || self.x + self.radius >= screen_width {
            self.velocity_x = -self.velocity_x;
        }
    }

    // Desenha a bola na tela
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [self.x, self.y],
            self.radius,
            0.1,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, DrawParam::default())
    }
}
