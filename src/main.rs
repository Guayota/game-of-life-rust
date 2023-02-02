extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::io::{Read, Write};

use glutin_window::GlutinWindow;
use graphics::Graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

macro_rules! clear_term {
    () => {
        // Clear screen and render field at the top
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

macro_rules! print_flush {
    ($($t:tt)*) => {
        {
            write!(std::io::stdout(), $($t)*).unwrap();
            std::io::stdout().flush().unwrap();
        }
    }
}

macro_rules! println_flush {
    () => {
        println!();
        std::io::stdout().flush().unwrap();
    };
    ($($t:tt)*) => {
        {
            write!(std::io::stdout(), $($t)*).unwrap();
            println!();
            std::io::stdout().flush().unwrap();
        }
    }
}

const WIN_TITLE: &str  = "Title";
const WIN_WIDTH:  u32  = 480;
const WIN_HEIGHT: u32  = 480;

// Colors constants
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

struct Canvas {
    gl: GlGraphics,
    cells: Vec<Vec<bool>>,
    cell_sizes: (u32, u32)
}

impl Canvas {
    fn render(&mut self, args: &RenderArgs) {
        // Background is cleared
        self.gl
            .draw(args.viewport(), |_c, gl| graphics::clear(BLACK, gl));

        // Render cells
        let cell_window_ratio = (
            WIN_WIDTH  / self.cell_sizes.0,
            WIN_HEIGHT / self.cell_sizes.1,
        );
        
        for row in self.cells.iter() {
        for (i, cell) in row.iter().enumerate() {
            if !*cell { return; }
            
            let rect = graphics::rectangle::rectangle_by_corners(
                i as f64 * cell_window_ratio.0 as f64,
                i as f64 * cell_window_ratio.0 as f64,
                i as f64 * cell_window_ratio.1 as f64 + self.cell_sizes.0 as f64,
                i as f64 * cell_window_ratio.1 as f64 + self.cell_sizes.1 as f64
                // 0.0, 0.0, 20.0, 20.0
            );
            
            self.gl.draw(args.viewport(), |c, gl| {
                graphics::rectangle(WHITE, rect, c.transform, gl)
            });
        }}
    }

    fn update(&mut self) {
        for row in 0..self.cell_sizes.0 as isize {
        for col in 0..self.cell_sizes.1 as isize {
            let mut neighbors = 0;
            for i in -1..1 {
            for j in -1..1 {
                if let Some(v) = self.cells.get((row+i) as usize) {
                    if let Some(_) = v.get((col+j) as usize) {
                        neighbors += 1;
                    }
                }
            }}

            let (i, j) = (row as usize, col as usize);
            match neighbors {
                2..=3 => self.cells[i][j] = true,
                _ => self.cells[i][j] = false
            }
        } }
    }

    fn pressed(&mut self, btn_arg: &ButtonArgs) {
        match &btn_arg.button {
            &Button::Keyboard(Key::Space) => println_flush!("Space pressed!"),
            _ => ()
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow =
        WindowSettings::new(WIN_TITLE, [WIN_WIDTH as u32, WIN_HEIGHT as u32])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .vsync(true)
            .resizable(false) // Makes it so i3wm won't resize it
            .build()
            .unwrap();

    let cell_sizes = (20u32, 20u32);
    let mut cells = vec![vec![true; (cell_sizes.1) as usize]; cell_sizes.0 as usize];

    let mut content = Canvas {
        gl: GlGraphics::new(opengl),
        cells,
        cell_sizes
    };

    let mut events = Events::new(EventSettings::new()).ups(60); // 60 FPS
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            content.render(&r);
        }

        if let Some(u) = e.update_args() {
            content.update();
        }

        if let Some(key) = e.button_args() {
            content.pressed(&key);
        }
    }
}
