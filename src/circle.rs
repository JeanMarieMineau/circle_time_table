extern crate num_complex;

use num_complex::Complex;
use std::cmp::min;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

pub struct Circle{
    points: Vec<Point>,
    n: u32,
}

impl Circle{
    pub fn new(n: u32, xm: u32, ym: u32)->Circle{
        let r = (min(xm, ym)/2 -50) as f64;
        let pi = std::f64::consts::PI;
        let center = Complex::new((xm/2) as f64, (ym/2) as f64);

        let mut points = Vec::new();
        for i in 0..n{
            let theta = ((2*i) as f64)*pi / (n as f64);
            let c = Complex::from_polar(&r, &theta) + center;
            let point = Point::new(
                c.re.round() as i32,
                c.im.round() as i32
            );
            points.push(point);
            println!("({}, {})", point.x(), point.y());
        }
        Circle{points: points, n:n}
    }

    pub fn display<T>(&self, canvas: &mut Canvas<T>)
        where T: sdl2::render::RenderTarget{

        canvas.set_draw_color(Color::RGB(255, 210, 0));
        for i in 0..(self.n-1){
            let j = i as usize;
            canvas.draw_line(self.points[j], self.points[j+1]).expect("");
        }
        canvas.draw_line(self.points[(self.n-1) as usize], self.points[0]).expect("");
    }
}
