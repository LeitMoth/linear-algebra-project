use glam::{Vec3, IVec2};

use crate::lines;

/// Representation of the application state.
pub struct World {
    width: u32,
    height: u32,
    triangles: Vec<Triangle>
}

type Triangle = [Vec3; 3];

impl World {

    pub fn new(width: u32, height: u32) -> Self {
        let triangles = vec![
            [
            Vec3::new(10.0,20.0,0.0),
            Vec3::new(30.0,20.0,0.0),
            Vec3::new(50.0,60.0,0.0),
            ],
            [
            Vec3::new(100.0,50.0,0.0),
            Vec3::new(70.0,40.0,0.0),
            Vec3::new(10.0,60.0,0.0),
            ],
        ];

        Self {
            width,
            height,
            triangles,
        }
    }

    pub fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        // clear buffer with blue
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff])
        }

        let mut plot = |v: IVec2| {
            assert!(v.x >= 0 && v.y >= 0 && (v.x as u32) < self.width && (v.y as u32) < self.height, "Tried to plot out of bounds.");
            let pix = 4 * (v.y * self.width as i32 + v.x);
            let pix = pix as usize;
            frame[pix..pix+4].copy_from_slice(&[0xFF, 0x80, 0x10, 0xFF]);
        };

        for triangle in &self.triangles {
            lines::plot_line(triangle[0].truncate().as_ivec2(), triangle[1].truncate().as_ivec2(), &mut plot);
            lines::plot_line(triangle[1].truncate().as_ivec2(), triangle[2].truncate().as_ivec2(), &mut plot);
            lines::plot_line(triangle[2].truncate().as_ivec2(), triangle[0].truncate().as_ivec2(), &mut plot);
        }
    }

}
