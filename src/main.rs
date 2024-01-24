use std::ffi::OsStr;
use std::path::Path;

use plotters::series::LineSeries;
use plotters::backend::SVGBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::style::{IntoFont, Color};
use rand::rngs::ThreadRng;
use rand::Rng;
use plotters::prelude::{WHITE,RED};

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
            .build_cartesian_2d(0..self.buf.len(), 0..10_000usize).map_err(|_| ())?;
        for (i,_) in self.enumerate() {
            if i == precition {
                break
            }
        }
        chart.configure_mesh().light_line_style(WHITE).draw().map_err(|_| ())?;
        chart.draw_series(
            LineSeries::new(
                    self.buf.iter()
                        .enumerate()
                        .map(|(i,x)| (i, *x)),
                    RED.mix(0.5).filled()
                    )).map_err(|_| ())?;
        Ok(())
    }
}

impl Iterator for Plane {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.r_thread.gen::<usize>();
        if n % 2 == 0 {
            self.ptr += 1;
            if self.ptr == self.buf.len() {
                self.ptr -= 2;
            }
        } else {
            if let Some(new_ptr) = self.ptr.checked_add_signed(-1) {
                self.ptr = new_ptr
            } else {
                self.ptr+=1
            }
        }
        self.buf[self.ptr] += 1;
        Some(self.buf[self.ptr])
    }
}

const OUT_FILE_NAME: &str = "markov.svg";

fn main() {
    let mut p = Plane::new(10001);
    p.draw(OUT_FILE_NAME,1000_000).unwrap();
}
