extern crate turtle;

use turtle::Turtle;
use turtle::Distance;

trait Fractal{
    fn koch(&mut self, length: f64, level: u32);
    fn op(&mut self, length: f64, level: u32) {
        if level == 0{
            self.forward(length);
        }else{
            self.koch(length,level);
        }
    }
    fn forward(&mut self, distance: Distance);
}

impl Fractal for Turtle{
    fn koch(&mut self, length: f64, level: u32){
        self.op(length/3.0,level-1);
        self.left(60.0);
        self.op(length/3.0,level-1);
        self.right(120.0);
        self.op(length/3.0,level-1);
        self.left(60.0);
        self.op(length/3.0,level-1);
    }
    fn forward(&mut self, distance: Distance){
        self.forward(distance);
    }
}

fn main() {
    let mut turtle = Turtle::new ();
    turtle.set_pen_size(2.0);
    turtle.pen_up();
    turtle.go_to([-200.0,100.0]);
    turtle.pen_down();
    turtle.set_speed(0);
    turtle.hide();
    turtle.right(90.0);
    turtle.koch(400.0,5);
    turtle.right(120.0);
    turtle.koch(400.0,5);
    turtle.right(120.0);
    turtle.koch(400.0,5);
}
