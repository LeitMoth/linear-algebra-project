// Bresenham's line algorithm, adapted from:
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

use glam::IVec2;

pub fn plot_line(mut p1: IVec2, p2: IVec2, mut plot: impl FnMut(IVec2) -> ()) {
    let dx = i32::abs(p2.x - p1.x);
    let dy = -i32::abs(p2.y - p1.y);
    let sx = if p1.x < p2.x { 1 } else { -1 };
    let sy = if p1.y < p2.y { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        plot(p1);
        if p1 == p2 { break }
        let e2 = 2 * error;
        if e2 >= dy {
            if p1.x == p2.x { break }
            error += dy;
            p1.x += sx;
        }
        if e2 <= dx {
            if p1.y == p2.y { break }
            error += dx;
            p1.y += sy;
        }
    }
}