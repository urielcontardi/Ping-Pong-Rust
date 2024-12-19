use crate::paddle::Paddle;
use crate::ball::Ball;

pub struct Robot {
    pub paddle: Paddle,
}

impl Robot {
    // Construtor para criar o robô
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            paddle: Paddle::new(x, y, width, height, speed),
        }
    }

    // Lógica do robô para mover a raquete
    pub fn update(&mut self, delta: f32, ball: &Ball, screen_height: f32) {
        let ball_y = ball.y;
        let paddle_center = self.paddle.y + self.paddle.height / 2.0;

        // Apenas mova a raquete se a bola estiver na direção dela
        if ball.velocity_x > 0.0 {
            // Adiciona um fator de "erro" para simular imprecisão
            let error_margin = 10.0;

            if ball_y < paddle_center - error_margin {
                self.paddle.move_up(delta * 0.9); // 90% da velocidade original
            } else if ball_y > paddle_center + error_margin {
                self.paddle.move_down(delta * 0.9, screen_height);
            }
        }
    }
}
