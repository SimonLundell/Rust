use ggez::graphics::{self, Canvas, DrawParam, Color, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use mint::{Point2};

const CAR_W: f32 = 30.0;
const HALF_CAR_W: f32 = CAR_W / 2.0;

pub struct Car {
    pos: Point2<f32>,
    vertices : Vec<Point2<f32>>,
    speed: f32,
    heading: f32,
    wheelbase: f32,
    rear_axle_from_front: f32,
    rear_overhang: f32
}

impl Car {
    pub fn new(pos: Point2<f32>, vertices: Vec<Point2<f32>>, speed: f32, heading: f32, wheelbase: f32, rear_axle_from_front: f32, rear_overhang: f32) -> Car {
        Car{pos: pos, vertices: vertices, speed: speed, heading: heading, wheelbase: wheelbase, rear_axle_from_front: rear_axle_from_front, rear_overhang: rear_overhang}   
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mb = &mut MeshBuilder::new();
        mb.line(&self.vertices, 2.0, Color::RED)?;
        let car = Mesh::from_data(ctx, mb.build());
        graphics::Canvas::draw(canvas, &car, DrawParam::default());
        Ok(())
    }
    
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed += speed;

        if self.speed > 2.5 {
            self.speed = 2.5;
        }
        else if self.speed < -1.25 {
            self.speed = -1.25;
        }
    }

    pub fn set_heading(&mut self, heading: f32) {
        self.heading += heading;
    }

    fn move_points(&mut self, speed: f32) {
        self.pos.x -= speed * f32::sin(-self.heading);
        self.pos.y -= speed * f32::cos(-self.heading);

        self.vertices[0] = Point2{x: self.pos.x - HALF_CAR_W, y: self.pos.y + self.rear_overhang};
        self.vertices[1] = Point2{x: self.pos.x + HALF_CAR_W, y: self.pos.y + self.rear_overhang};
        self.vertices[2] = Point2{x: self.pos.x + HALF_CAR_W, y: self.pos.y - self.rear_axle_from_front};
        self.vertices[3] = Point2{x: self.pos.x - HALF_CAR_W, y: self.pos.y - self.rear_axle_from_front};
        self.vertices[4] = self.vertices[0].clone();
    }

    fn rotate_points(&mut self, angle: f32) {
        for i in 0..self.vertices.len() - 1 {
            // Clone to not modify the value to early, causing rectangle to shrink
            let temp_x = self.vertices[i].x.clone();
            self.vertices[i].x = self.pos.x + (self.vertices[i].x - self.pos.x) * f32::cos(angle) - (self.vertices[i].y - self.pos.y) * f32::sin(angle);
            self.vertices[i].y = self.pos.y + (temp_x - self.pos.x) * f32::sin(angle) + (self.vertices[i].y - self.pos.y) * f32::cos(angle);
        }

        self.vertices[4] = self.vertices[0].clone();
    }

    pub fn update_position(&mut self) {
        self.move_points(self.speed);
        self.rotate_points(self.heading);
    }
}