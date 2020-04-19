extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::types;
use crate::escalator::{Escalator, Direction};
use crate::item::Item;
use std::borrow::BorrowMut;
use graphics::*;
use item::Rectangular;

mod escalator;
mod item;

pub struct App {
    gl: GlGraphics,
    escalator: Escalator,
    item: Item,
}

fn draw_step(color: [f32; 4], gl: &mut GlGraphics, step: &escalator::Step, transform: [[f64; 3]; 2]) {
    let rect = rectangle::rectangle_by_corners(step.left(), step.bottom(), step.right(), step.top());
    rectangle(color, rect, transform, gl);
}

fn draw_item(gl: &mut GlGraphics, item: &item::Item, transform: [[f64; 3]; 2]) {
    let rect = rectangle::rectangle_by_corners(item.left(), item.bottom(), item.right(), item.top());
    rectangle(item.color, rect, transform, gl);
}

impl App {
    fn render(&mut self, args: &RenderArgs) {


        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let item = &self.item;
        let steps = &self.escalator.steps;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let transform = c
                .scale(1., -1.)
                .trans(0., -args.window_size[1])
                .transform;

            for step in steps.iter() {
                draw_step(GREY, gl, step, transform);
            }

            draw_item(gl, item, transform);
        });
    }


    fn update(&mut self, args: &UpdateArgs) {
        self.item = self.item.update(args.dt);
        self.escalator.update(args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        escalator: Escalator::new(100., 100., 10, 0., 0.),
        item: Item::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}