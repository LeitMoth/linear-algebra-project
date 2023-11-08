use std::{
    f32::consts::TAU,
    path::Path,
    time::{Duration, SystemTime},
};

use glam::{IVec2, Mat4, Vec3};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{lines, model::Model, wavefront_obj::load_obj};

/// Representation of the application state.
pub struct World {
    width: u32,
    height: u32,
    models: Vec<Model>,
    start_time: SystemTime,
    processed_time: Duration,
    ticks: u128,
}

impl World {
    // adds cube // triangles are not modified after this point
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            models: vec![load_obj(Path::new("./res/blender_monkey.obj"))],
            start_time: SystemTime::now(),
            processed_time: Duration::from_secs(0),
            ticks: 0,
        }
    }

    // updates model matrix
    pub fn update(&mut self, input: &WinitInputHelper) {
        while self.processed_time < self.start_time.elapsed().unwrap() {
            self.processed_time += Duration::from_secs_f32(1.0 / 60.0);
            if !input.key_held(VirtualKeyCode::Space) {
                self.ticks += 1;
            }
        }
        // takes in number in seconds
        let slider = |seconds: f32| {
            let ticks = seconds * 60.0;
            (self.ticks % ticks as u128) as f32 / ticks
        };

        // gets even circle for rotation
        let a = slider(10.0) * TAU;

        // moves back and forth from -6 to 6
        let x = f32::powi(f32::sin(slider(25.0) * TAU), 3) * 6.0;

        for model in &mut self.models {
            // rotation first (cube starts at center) and then translation // at z = 4
            // model.transform = Mat4::from_translation(Vec3::new(x, 0.0, 4.0))
            //     * Mat4::from_rotation_x(a)
            //     * Mat4::from_rotation_y(a + 2.0);

            model.transform = Mat4::from_translation(Vec3::new(0.0, 0.0, 5.0 + x))
                * Mat4::from_rotation_y(a);
        }
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

        // centers at zero // identity matrix
        let camera = Mat4::from_translation(Vec3::ZERO);

        // will center the camera from any position/rotation
        let view = camera.inverse();
        // left handed perspective; positive z is away from the camera
        let proj = Mat4::perspective_lh(70_f32.to_radians(), (self.width as f32) / (self.height as f32), 2.0, 10.0);

        let screenspace = Mat4::IDENTITY
            * Mat4::from_translation(Vec3::new(
                self.width as f32 / 2.0,
                self.height as f32 / 2.0,
                0.0,
            ))
            // scales everything up; 
            * Mat4::from_scale(Vec3::new(self.width as f32, -(self.height as f32), 1.0));


        for model in &self.models {
            // puts model in space, moves camera to model, projects onto clipspace, transforms to screenspace
            let mat = screenspace * proj * view * model.transform;

            for ind in &model.indices {
                // find each index and project each vertex
                let triangle = ind.map(|i| model.verts[i]).map(|v| mat.project_point3(v)); 

                //clipping near and far plane 
                if triangle.iter().any(|t| t.z < 0.0 || t.z > 1.0) {
                    continue;
                }

                let mut plotcolor = |v: IVec2| {
                    // assert!(
                    //     v.x >= 0 && v.y >= 0 && (v.x as u32) < self.width && (v.y as u32) < self.height,
                    //     "Tried to plot out of bounds."
                    // );
                    let pix = 4 * (v.y * self.width as i32 + v.x);
                    let pix = pix as usize;

                    if !(v.x >= 0
                        && v.y >= 0
                        && (v.x as u32) < self.width
                        && (v.y as u32) < self.height)
                        || pix + 4 >= frame.len()
                    {
                        for p in frame.chunks_exact_mut(4).take(20) {
                            p.copy_from_slice(&[0xFF, 0x00, 0xFF, 0xFF]);
                        }
                        return;
                    }

                    //average z value of the triangles
                    let depth = (triangle[0].z + triangle[1].z + triangle[2].z) / 3.0;

                    let depth_8bit = (depth.clamp(0.0,1.0) * 255.0) as u8;

                    // checks if we are behind
                    if depth_8bit > frame[pix + 3] { return; }

                    let inv_depth = depth;

                    let reds = 255.0 * inv_depth.clamp(0.0, 1.0);
                    // let greens = 128.0 * avg_z.clamp(0.0,1.0);
                    // let blues = 16.0 * avg_z.clamp(0.0,1.0);
                    let greens = 255.0 * inv_depth.clamp(0.0, 1.0);
                    let blues = 255.0 * inv_depth.clamp(0.0, 1.0);

                    frame[pix..pix + 4].copy_from_slice(&[
                        reds as u8,
                        greens as u8,
                        blues as u8,
                        depth_8bit,
                    ]);
                };

                // with all the maps in the transform functions, allows us to treat the triangles as a list
                lines::plot_line(
                    triangle[0].truncate().as_ivec2(),
                    triangle[1].truncate().as_ivec2(),
                    &mut plotcolor,
                );
                lines::plot_line(
                    triangle[1].truncate().as_ivec2(),
                    triangle[2].truncate().as_ivec2(),
                    &mut plotcolor,
                );
                lines::plot_line(
                    triangle[2].truncate().as_ivec2(),
                    triangle[0].truncate().as_ivec2(),
                    &mut plotcolor,
                );
            }
        }
    }
}
