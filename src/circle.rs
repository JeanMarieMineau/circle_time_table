extern crate num_complex;

use num_complex::Complex;
use std::cmp::min;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

pub struct Circle{
    points: Vec<Point>,
    n: usize,
    xm: u32,
    ym: u32,
}

impl Circle{
    pub fn new(n: usize, xm: u32, ym: u32)->Circle{
        let mut circle = Circle{
            points: Vec::new(),
            n:0,
            xm: xm,
            ym: ym
        };
        circle.set_n(n);
        circle
    }

    pub fn set_n(&mut self, n: usize){
        let r = (min(self.xm, self.ym)/2 -50) as f64;
        let pi = std::f64::consts::PI;
        let center = Complex::new((self.xm/2) as f64, (self.ym/2) as f64);

        self.points.clear();
        let quarter = (2 as f64)*pi / (n as f64);
        for i in 0..n{
            let theta = (i as f64) * quarter;
            let c = Complex::from_polar(&r, &theta) + center;
            let point = Point::new(
                c.re.round() as i32,
                c.im.round() as i32
            );
            self.points.push(point);
        }
        self.n = n;
    }

    fn draw_line<T>(&self, canvas: &mut Canvas<T>, i: usize, j: usize)
        where T: sdl2::render::RenderTarget{

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(self.points[i%self.n], self.points[j%self.n]).expect("");
    }

    pub fn draw_table<T>(&self, canvas: &mut Canvas<T>, m: usize)
        where T: sdl2::render::RenderTarget{
        for j in 0..self.n{
            self.draw_line(canvas, j, j*m);
        }
    }
}
