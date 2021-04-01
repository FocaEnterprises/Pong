extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::piston::ReleaseEvent;
use crate::entity::*;
use piston::Button;
use piston::PressEvent;
use piston::keyboard::Key;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod entity;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ball_x: f64,
    ball_y: f64,
    ball_size: f64,
    ball_vel_x: f64,
    ball_vel_y: f64,
    player_y1: f64,
    player_y2: f64,
    player_down1: bool,
    player_up1: bool,
    player_down2: bool,
    player_up2: bool,
}

impl App {
    fn update(&mut self, _: &UpdateArgs) {
        self.ball_x += self.ball_vel_x;
        self.ball_y += self.ball_vel_y;

        if self.ball_x + self.ball_size > self.player_x2()
        && self.ball_y < self.player_y2 + self.player_height()
        && self.ball_y + self.ball_size > self.player_y2 {
            self.ball_vel_x = -self.ball_vel_x;
        }

        if self.ball_x < self.player_x1() + self.player_width()
        && self.ball_y < self.player_y1 + self.player_height()
        && self.ball_y + self.ball_size > self.player_y1 {
            self.ball_vel_x = -self.ball_vel_x;
        }

        if self.ball_y + self.ball_size > HEIGHT.into() {
            self.ball_vel_y *= -1.0;
        }

        if self.ball_y < 0.0 {
            self.ball_vel_y *= -1.0;
        }

        if self.ball_x < 0.0 || self.ball_x > WIDTH.into() {
            self.ball_x = WIDTH as f64 / 2.0 - self.ball_size / 2.0;
            self.ball_y = HEIGHT as f64 / 2.0 - self.ball_size / 2.0;
        }

        if self.player_up1 {
            self.player_y1 -= 5.0;
        }
        else if self.player_down1 {
            self.player_y1 += 5.0;
        }
        if self.player_up2 {
            self.player_y2 -= 5.0;
        }
        else if self.player_down2 {
            self.player_y2 += 5.0;
        }

        if self.player_y1 < 0.0 {
            self.player_y1 = 0.0;
        }
        else if self.player_y1 > HEIGHT as f64 - self.player_height() {
            self.player_y1 = HEIGHT as f64 - self.player_height();
        }
        if self.player_y2 < 0.0 {
            self.player_y2 = 0.0;
        }
        else if self.player_y2 > HEIGHT as f64 - self.player_height() {
            self.player_y2 = HEIGHT as f64 - self.player_height();
        }
    }
    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let ball = rectangle::square(self.ball_x, self.ball_y, self.ball_size);
        let player1 = [self.player_x1(), self.player_y1, self.player_width(), self.player_height()];
        let player2 = [self.player_x2(), self.player_y2, self.player_width(), self.player_height()];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(color::BLACK, gl);

            let transform = c
                .transform;

            // Draw a box rotating around the middle of the screen.
            rectangle([1.0, 1.0, 0.0, 1.0], ball, transform, gl);
            rectangle([1.0, 0.0, 0.0, 1.0], player1, transform, gl);
            rectangle([0.0, 0.0, 1.0, 1.0], player2, transform, gl);
        });
    }

    fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::W) => {
                self.player_up1 = true;
            },
            Button::Keyboard(Key::S) => {
                self.player_down1 = true;
            },
            Button::Keyboard(Key::Up) => {
                self.player_up2 = true;
            },
            Button::Keyboard(Key::Down) => {
                self.player_down2 = true;
            }
            _ => {}
        }
    }

    fn release(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::W) => {
                self.player_up1 = false;
            },
            Button::Keyboard(Key::S) => {
                self.player_down1 = false;
            },
            Button::Keyboard(Key::Up) => {
                self.player_up2 = false;
            },
            Button::Keyboard(Key::Down) => {
                self.player_down2 = false;
            }
            _ => {}
        }
    }

    fn player_x1(&self) -> f64 {
        0.0
    }
    
    fn player_x2(&self) -> f64 {
        (WIDTH as f64) - self.player_width()
    }
    
    fn player_width(&self) -> f64 {
        self.ball_size
    }

    fn player_height(&self) -> f64 {
        self.ball_size * 3.0
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Pong", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball_x: WIDTH as f64 / 2.0 - 20.0 / 2.0,
        ball_y: HEIGHT as f64 / 2.0 - 20.0 / 2.0,
        ball_size: 20.0,
        ball_vel_x: 3.0,
        ball_vel_y: 3.0,
        player_y1: HEIGHT as f64 / 2.0 - (20.0 * 3.0) / 2.0,
        player_y2: HEIGHT as f64 / 2.0 - (20.0 * 3.0) / 2.0,
        player_down1: false,
        player_up1: false,
        player_down2: false,
        player_up2: false,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(button) = e.press_args() {
            app.press(button);
        }

        if let Some(button) = e.release_args() {
            app.release(button);
        }
    }
}
