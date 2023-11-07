use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use glam::Mat4;
use glam::Vec3;

use crate::model::Model;

pub fn load_obj(p: &Path) -> Model {
    let f = BufReader::new(File::open(p).unwrap());

    let mut verts = vec![];
    let mut faces = vec![];

    let mut name = "Unnamed Object".to_owned();

    for line in f.lines() {
        let line = line.unwrap();

        if line.starts_with("o") {
            name = line.split_ascii_whitespace().nth(1).unwrap().to_owned();
        }

        if line.starts_with('v') {
            let vert: Vec<f32> = line
                .split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect();
            verts.push(Vec3::new(vert[0], vert[1], vert[2]));
        }

        if line.starts_with('f') {
            let face: Vec<usize> = line
                .split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect();
            assert!(face.iter().all(|&x| 0 < x && x <= verts.len()));
            // We subtract one to switch to 0 indexing
            faces.push([face[0] - 1, face[1] - 1, face[2] - 1]);
        }
    }

    Model {
        name,
        verts,
        indices: faces,
        transform: Mat4::IDENTITY,
    }
}
