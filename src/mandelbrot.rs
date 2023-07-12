use crate::utils::*;
use crate::pixelmapper::PixelMapper;
use crate::grid::Grid;

fn iter(z: Complex, c: Complex) -> Complex {
    z.square() + c
}
pub fn do_point(c: Complex, max_iter: usize) -> Option<usize> {
    // cardioid/bulb checking
    let p = ((c.real - 0.25).powi(2) + c.imag.powi(2)).sqrt();
    if c.real <= p - (2.0 * p.powi(2)) + 0.25 {
        return None
    }
    if (c.real + 1.0).powi(2) + c.imag.powi(2) <= 1.0 / 16.0 {
        return None
    }

    let mut z = c;
    let mut old = z;

    for i in 0..max_iter {
        if i % 4 == 0 {
            old = z
        }
        z = iter(z, c);
        if z.fuzzy_eq(old) {
            return None
        }
        if z.magnitude_squared() > 4.0 {
            return Some(i)
        }
    }
    None
}
pub fn generate_iteration_tables(pm: &PixelMapper, width: usize, height: usize, max_iter: usize) -> (Grid<u16>, Vec<f32>) {
    let mut g = Grid::new(width, height, 0u16);
    let mut h = Vec::new(); h.resize(max_iter, 0usize);
    let mut total = 0usize;

    for (x, y, v) in g.iter_coords_mut() {
        match do_point(pm.map(x, y), max_iter) {
            Some(i) => {
                *v = i as u16;
                h[i] += 1;
                total += 1
            }
            None => {
                *v = max_iter as u16
            }
        }
    }

    let h = accumulate_normalise_iterations(&h, total);

    (g, h)
}
/// returns a vec v where v[i] = sum of h[0..=i] / total
fn accumulate_normalise_iterations(h: &Vec<usize>, total: usize) -> Vec<f32> {
    let mut v = Vec::new(); v.reserve(h.len());
    let mut acc = 0;
    v.extend(h.iter().map(|i| {
        acc += i;
        acc as f32 / total as f32
    }));
    v
}

pub fn draw_into_buffer(pm: &PixelMapper, width: usize, height: usize, buffer: &mut [u32], max_iter: usize) {
    let (g, h) = generate_iteration_tables(pm, width, height, max_iter);

    g.iter().zip(buffer.iter_mut()).for_each(|(i, p)| {
        if let Some(cf) = h.get(*i as usize) {
            let blue = (cf * 256.0) as u8;
            *p = blue as u32;
        }
        else {
            *p = 0x00ff_ffff
        }
    });
}
