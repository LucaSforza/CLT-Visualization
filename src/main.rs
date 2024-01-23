use std::ffi::OsStr;
use std::path::Path;

use plotters::backend::SVGBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::CandleStick;
use plotters::style::IntoFont;
use rand::rngs::ThreadRng;
use rand::Rng;
use plotters::prelude::WHITE;

struct Plane {
    buf: Vec<usize>,
    ptr: usize,
    r_thread: ThreadRng
}

impl Plane {
    fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("the buf must be an odd size")
        }
        Self {
            buf: vec![0;size],
            ptr: size/2 + 1,
            r_thread: rand::thread_rng(),
        }
    }

    //TODO: change error type
    fn draw<S: AsRef<OsStr> + ?Sized>(&mut self, path: &S,precition: usize) -> Result<(),()> {
        let path = Path::new(path);
        let root = SVGBackend::new(path, (1400, 800)).into_drawing_area();
        root.fill(&WHITE).map_err(|_| ())?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption("Markov visualitation of Central Limit Theorem", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(0..self.buf.len(), 0..precition/2).map_err(|_| ())?;
        for (i,_) in self.enumerate() {
            if i == precition {
                break
            }
        }
        chart.configure_mesh().light_line_style(WHITE).draw().map_err(|_| ())?;
        chart.draw_series(self.buf.iter().enumerate().map(|i,e| CandleStick::new(Local::now(), 0, e, low, close, gain_style, loss_style, width)));
        Ok(())
    }

    fn get_median(&self) -> usize {
        self.buf.len()/2
    }
}

impl Iterator for Plane {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.r_thread.gen::<usize>();
        if n % 2 == 0 {
            if self.ptr.checked_add_signed(1).is_none() {
                self.ptr-=1
            }
        } else {
            if self.ptr.checked_add_signed(-1).is_none() {
                self.ptr+=1
            }
        }
        self.buf[self.ptr] += 1;
        Some(self.buf[self.ptr])
    }
}

const OUT_FILE_NAME: &str = "markov.svg";

fn main() {
    Plane::new(101).draw(OUT_FILE_NAME,10_000).unwrap();
}
