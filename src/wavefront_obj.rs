use std::{path::Path, io::{BufReader, BufRead}, fs::File};

use glam::Mat4;

#[derive(Debug)]
pub struct Model {
    name: String,
    verts: Vec<[f32;3]>,
    faces: Vec<[usize;3]>,
    transform: Mat4,
}

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
            let vert: Vec<f32> = line.split_ascii_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            verts.push([vert[0],vert[1],vert[2]]);
        }

        if line.starts_with('f') {
            let face: Vec<usize> = line.split_ascii_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            assert!(face.iter().all(|&x| 0 < x && x <= verts.len()));
            faces.push([face[0],face[1],face[2]]);
        }
    }

    Model {
        name,
        verts,
        faces,
        transform: Mat4::IDENTITY,
    }
}
