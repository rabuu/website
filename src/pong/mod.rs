mod math;
use math::Vec2;

#[derive(Debug, Clone)]
pub struct Pong {
    pub width: f64,
    pub height: f64,
    pub line_width: f64,

    pub paddle_width: f64,
    pub paddle_height: f64,

    pub paddle_left: Vec2,
    pub paddle_right: Vec2,

    pub paddle_speed: f64,
    pub paddle_right_speed_factor: f64,

    pub ball_size: f64,

    pub ball: Vec2,
    pub ball_vel: Vec2,
}

impl Pong {
    pub fn new(width: f64, ball_vel_x: f64, ball_vel_y: f64) -> Self {
        let height = width / 2.0;
        let line_width = width / 100.0;

        let paddle_width = 1.3 * line_width;
        let paddle_height = width / 7.0;

        let paddle_left = Vec2 {
            x: 2.0 * line_width,
            y: (height - paddle_height) / 2.0,
        };
        let paddle_right = Vec2 {
            x: width - paddle_width - 2.0 * line_width,
            y: (height - paddle_height) / 2.0,
        };

        let paddle_speed = ball_vel_y.abs() * 0.9;
        let paddle_right_speed_factor = 0.9;

        let ball_size = 2.0 * line_width;

        let ball = Vec2 {
            x: (width + ball_size) / 2.0,
            y: (height + ball_size) / 2.0,
        };

        let ball_vel = Vec2 {
            x: ball_vel_x,
            y: ball_vel_y,
        };

        Self {
            width,
            height,
            line_width,
            paddle_width,
            paddle_height,
            paddle_left,
            paddle_right,
            paddle_speed,
            paddle_right_speed_factor,
            ball_size,
            ball,
            ball_vel,
        }
    }

    pub fn tick(&mut self) {
        self.ball += self.ball_vel;

        // master of AI
        let paddle_center = self.paddle_right.y + (self.paddle_height / 2.0);
        if self.ball.y < paddle_center {
            self.scroll_right(true);
        } else if self.ball.y > paddle_center {
            self.scroll_right(false);
        }

        // walls left & right
        if self.ball.x >= self.width - self.line_width - self.ball_size {
            self.ball_vel.x = -self.ball_vel.x;
        } else if self.ball.x <= self.line_width {
            self.ball_vel.x = -self.ball_vel.x;
        }

        // walls top & bottom
        if self.ball.y >= self.height - self.line_width - self.ball_size {
            self.ball_vel.y = -self.ball_vel.y;
        } else if self.ball.y <= self.line_width {
            self.ball_vel.y = -self.ball_vel.y;
        }

        // left paddle
        if self.ball.x <= self.paddle_left.x + self.paddle_width
            && self.ball.x >= self.paddle_left.x
            && self.ball.y + self.ball_size > self.paddle_left.y
            && self.ball.y <= self.paddle_left.y + self.paddle_height
        {
            self.ball_vel.x = -self.ball_vel.x;
        }

        // right paddle
        if self.ball.x + self.ball_size >= self.paddle_right.x
            && self.ball.x <= self.paddle_right.x + self.paddle_width
            && self.ball.y + self.ball_size > self.paddle_right.y
            && self.ball.y <= self.paddle_right.y + self.paddle_height
        {
            self.ball_vel.x = -self.ball_vel.x;
        }
    }

    pub fn scroll_left(&mut self, up: bool) {
        let val = if up {
            -self.paddle_speed
        } else {
            self.paddle_speed
        };
        self.paddle_left.y = (self.paddle_left.y + val).clamp(
            self.line_width,
            self.height - self.line_width - self.paddle_height,
        );
    }

    pub fn scroll_right(&mut self, up: bool) {
        let val = if up {
            -self.paddle_speed * self.paddle_right_speed_factor
        } else {
            self.paddle_speed * self.paddle_right_speed_factor
        };
        self.paddle_right.y = (self.paddle_right.y + val).clamp(
            self.line_width,
            self.height - self.line_width - self.paddle_height,
        );
    }
}
