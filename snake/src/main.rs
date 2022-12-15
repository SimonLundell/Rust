// Remind rust of the dependencies
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Use the crate
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;

const WINDOW_X: i32 = 500;
const WINDOW_Y: i32 = 500;
const BLOCKS: i32 = 20;

#[derive(Clone, PartialEq)]
enum Direction
{
    Right, Left, Up, Down
}

struct Game
{
    gl: GlGraphics,
    snake: Snake,
    candy: Candy,
}

impl Game
{
    fn render(&mut self, args: &RenderArgs)
    {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl|
        {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.candy.render(&mut self.gl, args);
    }

    fn update(&mut self)
    {
        self.snake.update();
        
        let head = (*self.snake.body.front().expect("Snake has no body")).clone();

        let head_in_body = self.check_coordinate(head);

        if head.0 < 0 || head.0 > WINDOW_X / BLOCKS - 1 || head.1 < 0 || head.1 > WINDOW_Y / BLOCKS - 1 || head_in_body
        {
            self.reset();
        }

        if head.0 == self.candy.position.0 && head.1 == self.candy.position.1
        {
            loop 
            {
                self.candy.randomize_position();
                if !self.check_coordinate(self.candy.position)
                {
                    break;
                }
            }
            let mut tail = (*self.snake.body.back().expect("Snake has no body")).clone();
        
            match self.snake.dir
            {
                Direction::Left => tail.0 -= 1,
                Direction::Right => tail.0 += 1,
                Direction::Up => tail.1 -= 1,
                Direction::Down => tail.1 += 1,
            }
            
            self.snake.body.push_back(tail);
        }
    }

    fn reset(&mut self)
    {
        self.snake.body = LinkedList::from_iter((vec![(0,0), (1,0)]).into_iter());
        self.snake.dir = Direction::Right;
    }

    fn pressed(&mut self, btn: &Button)
    {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn
        {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
            
        }
    }

    fn check_coordinate(&mut self, control_point: (i32, i32)) -> bool
    {
        let mut snake_copy = (self.snake.body).clone();
        snake_copy.pop_front();

        let mut snake_iter = snake_copy.iter();
        while let Some(val) = snake_iter.next()
        {
            if control_point == *val
            {
                return true;
            }
        };
        
        return false;
    }
}

struct Snake
{
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake
{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs)
    {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body.iter().map(|&(x, y)|
        {
            graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64)
        }).collect();
        
        gl.draw(args.viewport(), |_c, gl|
        {
            let transform = _c.transform;

            squares.into_iter().for_each(|square|graphics::rectangle(red, square, transform, gl));
        })
    }

    fn update(&mut self)
    {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir{
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }


        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

struct Candy
{
    position: (i32, i32),
}

impl Candy
{
    pub fn candy() -> Candy
    {
        let mut rng = rand::thread_rng();
        let mut pos = (0, 0);

        pos.0 = rng.gen_range(0..24);
        pos.1 = rng.gen_range(0..24);

        return Candy{position: pos}
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs)
    {
        let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square: graphics::types::Rectangle = graphics::rectangle::square((self.position.0 * 20) as f64, (self.position.1 * 20) as f64, 20_f64);
        
        gl.draw(args.viewport(), |_c, gl|
        {
            let transform = _c.transform;

            graphics::rectangle(blue, square, transform, gl);
        })
    }

    fn randomize_position(&mut self)
    {
        let mut rng = rand::thread_rng();

        self.position.0 = rng.gen_range(0..24);
        self.position.1 = rng.gen_range(0..24);
    }

}

fn main() 
{
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [WINDOW_X as f64, WINDOW_Y as f64]).graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game {gl: GlGraphics::new(opengl), snake: Snake {body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()), dir: Direction::Right }, candy: Candy::candy() };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) 
    {
        if let Some(r) = e.render_args() 
        {
            game.render(&r);
        }

        if let Some(_u) = e.update_args()
        {
            game.update();
        }

        if let Some(k) = e.button_args()
        {
            if k.state == ButtonState::Press
            {
                game.pressed(&k.button);
            }
        }
    }
}
