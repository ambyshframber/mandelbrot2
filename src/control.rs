use winit::event_loop::EventLoopProxy;

use crate::utils::*;
use crate::pixelmapper::PixelMapper;

struct Controller {
    proxy: EventLoopProxy<ViewUpdate>,

    render_pm: PixelMapper,
    centre: Complex,
    radius: f32,
    angle: f32,

    iw: u32,
    ih: u32,

    scale: f32,
    vw: u32,
    vh: u32,
}

pub fn control_loop(p: EventLoopProxy<ViewUpdate>) {
    let mut controller = Controller::new(p);
    let c = rustyline::config::Config::builder().auto_add_history(true).build();

    let mut rl = rustyline::DefaultEditor::with_config(c).unwrap();
    loop {
        let s = rl.readline("> ").unwrap();
        if let Some(c) = parse_line(&s) {
            controller.do_command(c)
        }
    }
}

impl Controller {
    fn new(proxy: EventLoopProxy<ViewUpdate>) -> Self {
        Self {
            proxy,

            render_pm: PixelMapper::new_radx(Complex { real: -1.0, imag: 0.0 }, 1.0, 1.0, crate::STARTING_WIDTH, crate::STARTING_HEIGHT),
            centre: Complex { real: -0.5, imag: 0.0 },
            radius: 1.0,
            angle: 0.0,

            iw: crate::STARTING_WIDTH,
            ih: crate::STARTING_HEIGHT,

            scale: 1.0 / 3.0,
            vw: crate::STARTING_WINDOW_WIDTH,
            vh: crate::STARTING_WINDOW_HEIGHT,
        }
    }
    fn do_command(&mut self, c: Command) {

        use Command::*;
        match c {
            Render(name, max_iter, aa) => self.render(name, max_iter, aa),
            Resolution(x, y, sd) => {
                let pm = PixelMapper::new_radx(self.centre, self.radius, self.angle, x, y);
                let scale = 1.0 / sd;
                self.scale = scale;
                let vf_pm = pm.scale(scale);
                let vw = (x as f32 * scale) as u32;
                self.vw = vw;
                let vh = (y as f32 * scale) as u32;
                self.vh = vh;
                let _ = self.proxy.send_event(ViewUpdate { pm: vf_pm, wi: vw, hi: vh });
            }
            View(centre, radius, angle) => {
                //eprintln!("view changed");
                let pm = PixelMapper::new_radx(centre, radius, angle, self.iw, self.ih);
                self.render_pm = pm;
                let vf_pm = pm.scale(self.scale);
                let _ = self.proxy.send_event(ViewUpdate { pm: vf_pm, wi: self.vw, hi: self.vh });
            }
            _ => println!("beans")
        }

    }

    fn render(&self, name: &str, max_iter: usize, aa: usize) {
        use std::time::Instant;
        use crate::mandelbrot;
        use image::{Rgb, RgbImage};

        let start = Instant::now();

        let (g, h) = mandelbrot::generate_iteration_tables(&self.render_pm, self.iw as usize, self.ih as usize, 100);
        println!("tables took {}ms", start.elapsed().as_millis());
        let tables = Instant::now();
        let mut i = RgbImage::new(self.iw, self.ih);
        i.pixels_mut().zip(g.iter()).for_each(|(p, i)| {
            if let Some(count) = h.get(*i as usize) {
                let blue = (count * 256.0) as u8;
                *p = Rgb([0, 0, blue])
            }
            else {
                *p = Rgb([255, 255, 255])
            }
        });
        println!("colouring took {}ms", tables.elapsed().as_millis());
        if let Err(e) = i.save(name) {
            println!("failed to save: {}", e)
        }
    }
}

fn parse_line(l: &str) -> Option<Command> {
    let mut i = l.split_ascii_whitespace();
    Some(match i.next()? {
        "render" => {
            let name = i.next().unwrap_or("output.png");
            let max_iter = i.next().map(|v| v.parse().ok()).unwrap_or(Some(100))?;
            let aa = i.next().map(|v| v.parse().ok()).unwrap_or(Some(1))?;
            Command::Render(name, max_iter, aa)   
        }
        "res" => {
            let x = i.next().and_then(|v| v.parse().ok())?;
            let y = i.next().and_then(|v| v.parse().ok())?;
            let sd = i.next().map(|v| v.parse().ok()).unwrap_or(Some(3.0))?;
            Command::Resolution(x, y, sd)
        }
        "view" => {
            let real = i.next().and_then(|v| v.parse().ok())?;
            let imag = i.next().and_then(|v| v.parse().ok())?;
            let r = i.next().and_then(|v| v.parse().ok())?;
            let angle = i.next().and_then(|v| v.parse().ok())?;
            Command::View(Complex { real, imag }, r, angle)
        }
        "settings" => Command::Settings,
        _ => return None
    })
}

#[derive(Debug)]
pub struct ViewUpdate {
    pub pm: PixelMapper,
    pub wi: u32,
    pub hi: u32
}

enum Command<'a> {
    /// renders the current view to a file
    /// name, max iter, aa
    Render(&'a str, usize, usize),
    /// changes the resolution of the target view and viewfinder
    /// the float is scale divisor, ie. how many pixels of render per every pixel of viewfinder
    /// if it's 3, divide the resolution by 3 and send that to the viewfinder
    Resolution(u32, u32, f32),
    /// changes the position, radius and angle of the current view
    View(Complex, f32, f32),

    /// prints the current view information to the console
    Settings
}
