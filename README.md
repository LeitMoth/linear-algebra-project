# Software Renderer (MATH270 - Linear Algebra)

## About

A bare-bones software renderer that loads a simple .obj file and displays its wireframe.
To change which file is loaded you have to edit the code, this will be changed in the future.

In order to showcase perspective, the model moves towards and away from the camera while rotating.
The brightness of triangle outlines changes with depth, and basic clipping is implemented for the near and far planes.
You can pause the movement by holding space.

## The Involved Linear Algebra

Triangles are projected from their native 3D world space to 2D screen space, for our viewing.
This involves extensive use of matrix multiplication and affine transformations,
demonstrating applications of the various linear algebra concepts we have learned inside and outside of class.
Also, the more interesting transformations like translation and perspective projection require us to work in homogeneous coordinates.

## Installation

Rust and cargo are needed to build this project.
Instructions on how to grab both at once can be found here: https://doc.rust-lang.org/cargo/getting-started/installation.html

Once you have those, open a terminal and run `cargo run` in the directory containing this README file.
This may take a while the first time, cargo is downloading dependencies for you.

If you want to play with the code, any text editor will do, but many people like Visual Studio Code (https://code.visualstudio.com/).

If you are someone at Hendrix and need help or have questions, don't hesitate to message Colin on teams.

## Authors

- Colin Phillips - Hendrix College
- Harry Lance - Hendrix College
