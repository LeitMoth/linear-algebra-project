use std::{f32::consts::{PI, TAU}, time::SystemTime};

use glam::{IVec2, Mat4, Vec3};

use crate::{lines, mesh};

/// Representation of the application state.
pub struct World {
    width: u32,
    height: u32,
    triangles: Vec<Triangle>,
    model: Mat4,
    start_time: SystemTime,
}

type Triangle = [Vec3; 3];

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let mut triangles = vec![];

        triangles.extend(mesh::cube());

        Self {
            width,
            height,
            triangles,
            model: Mat4::IDENTITY,
            start_time: SystemTime::now(),
        }
    }

    pub fn update(&mut self) {
        // let mat = Mat4::from_translation(Vec3::new(0.0,0.0,0.1));

        // self.triangles.iter_mut().flat_map(|tri| tri.iter_mut()).for_each(|v| {
        //         let hom = v.extend(1.0);
        //         let result = mat * hom;
        //         *v = result.truncate() / result.w;
        // });

        let slider = |milis: u128| (self.start_time.elapsed().unwrap().as_millis() % milis) as f32 / milis as f32;

        let a = slider(3000) * TAU;

        let x = f32::powi(f32::sin(slider(8000) * TAU), 3) * 6.0;

        self.model = Mat4::from_translation(Vec3::new(x,0.0,4.0)) * Mat4::from_rotation_x(a) * Mat4::from_rotation_y(a + 2.0);
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        // clear buffer with blue
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff])
        }



        // setup matrix transforms
        let camera = Mat4::from_translation(Vec3::ZERO);

        let view = camera.inverse();
        let proj = Mat4::perspective_lh(360.0/(2.0*PI)*70.0, 1.0, 0.1, 100.0);

        let screenspace = Mat4::IDENTITY
            * Mat4::from_translation(Vec3::new(self.width as f32 / 2.0, self.height as f32 / 2.0, 0.0))
            * Mat4::from_scale(Vec3::splat(u32::min(self.width, self.height) as f32));

        let mat = screenspace * proj * view * self.model;

        // transform points
        let transform_point = |v| mat.project_point3(v);

        let transformed = self.triangles.iter().map(|triangle| {
            triangle.map(transform_point)
        });



        // get ready to draw
        let mut plot = |v: IVec2| {
            // assert!(
            //     v.x >= 0 && v.y >= 0 && (v.x as u32) < self.width && (v.y as u32) < self.height,
            //     "Tried to plot out of bounds."
            // );
            let pix = 4 * (v.y * self.width as i32 + v.x);
            let pix = pix as usize;

            if
                !(v.x >= 0 && v.y >= 0 && (v.x as u32) < self.width && (v.y as u32) < self.height) ||
                pix + 4 >= frame.len()
            {
                for p in frame.chunks_exact_mut(4).take(20) {
                    p.copy_from_slice(&[0xFF,0x00,0xFF,0xFF]);
                }
                return;
            }

            frame[pix..pix + 4].copy_from_slice(&[0xFF, 0x80, 0x10, 0xFF]);
        };

        // iterate over all transformed triangles, and draw them to the screen
        for triangle in transformed {
            lines::plot_line(
                triangle[0].truncate().as_ivec2(),
                triangle[1].truncate().as_ivec2(),
                &mut plot,
            );
            lines::plot_line(
                triangle[1].truncate().as_ivec2(),
                triangle[2].truncate().as_ivec2(),
                &mut plot,
            );
            lines::plot_line(
                triangle[2].truncate().as_ivec2(),
                triangle[0].truncate().as_ivec2(),
                &mut plot,
            );
        }
    }
}
