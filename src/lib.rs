//! Simple software renderer that is currently void of useful features. 
//! This is a hobby project and is not recommended for use, there are much better libraries/crates out there.

use std::ops::Mul;

#[derive(Clone, Copy)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

/// Three element vector. Used internally for colors (soon) and vertices.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from([self * rhs.x, self * rhs.y, self * rhs.z])
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(nums: [f64; 3]) -> Self {
        Vec3 {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

pub type Vertex = Vec3;
pub type _Color = Vec3;

/// Simple 3x3 Matrix utility struct for graphics maths.
pub struct Mat3x3 {
    pub mat: [[f64; 3]; 3],
}

impl Mat3x3 {
    /// This poorly named function actually gives the 3x3 identity matrix scaled by the parameter `val`.
    pub fn identity(val: f64) -> Mat3x3 {
        Mat3x3 {
            mat: [
                [val, 0_f64, 0_f64],
                [0_f64, val, 0_f64],
                [0_f64, 0_f64, val],
            ],
        }
    }

    /// Gives a rotation matrix to rotate a vertex about the origin by the given angle along the given axis.
    pub fn rot(angle: f64, axis: Axis) -> Mat3x3 {
        match axis {
            Axis::X => Mat3x3 {
                mat: [
                    [1_f64, 0_f64, 0_f64],
                    [0_f64, angle.cos(), angle.sin()],
                    [0_f64, -angle.sin(), angle.cos()],
                ],
            },
            Axis::Y => Mat3x3 {
                mat: [
                    [angle.cos(), 0_f64, -angle.sin()],
                    [0_f64, 1_f64, 0_f64],
                    [angle.sin(), 0_f64, angle.cos()],
                ],
            },
            Axis::Z => Mat3x3 {
                mat: [
                    [angle.cos(), -angle.sin(), 0_f64],
                    [angle.sin(), angle.cos(), 0_f64],
                    [0_f64, 0_f64, 1_f64],
                ],
            },
        }
    }
}

impl Mul<Vec3> for Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from([
            rhs.x * self.mat[0][0] + rhs.y * self.mat[0][1] + rhs.z * self.mat[0][2],
            rhs.x * self.mat[1][0] + rhs.y * self.mat[1][1] + rhs.z * self.mat[1][2],
            rhs.x * self.mat[2][0] + rhs.y * self.mat[2][1] + rhs.z * self.mat[2][2],
        ])
    }
}

///Triangle with three verticies.
#[derive(Debug, Copy, Clone)]
pub struct Tri {
    pub verts: [Vertex; 3],
}

impl Tri {
    fn render(&self, draw_line: fn(f64, f64, f64, f64)) {
        draw_line(
            self.verts[0].x,
            self.verts[0].y,
            self.verts[1].x,
            self.verts[1].y,
        );
        draw_line(
            self.verts[1].x,
            self.verts[1].y,
            self.verts[2].x,
            self.verts[2].y,
        );
        draw_line(
            self.verts[2].x,
            self.verts[2].y,
            self.verts[0].x,
            self.verts[0].y,
        );
    }
}

/// Mesh of triangles.
#[derive(Debug, Clone)]
pub struct Mesh {
    pub tris: Vec<Tri>,
}

impl Mesh {
    /// Creates a cube given two opposing vertices.
    #[rustfmt::skip]
    pub fn cube(v1: Vertex, v2: Vertex) -> Mesh {
        // Adapted from [javidx9's](https://www.youtube.com/c/javidx9/) olc 3d render engine
        Mesh {
            tris: vec![
                // South
                Tri { verts: [ Vec3::from([ v1.x, v1.y, v1.z,]), Vec3::from([ v1.x, v2.y, v1.z,]), Vec3::from([ v2.x, v2.y, v1.z,])]},
                Tri { verts: [ Vec3::from([ v1.x, v1.y, v1.z,]), Vec3::from([ v2.x, v2.y, v1.z,]), Vec3::from([ v2.x, v1.y, v1.z,])]},

                // East
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v1.z,]), Vec3::from([ v2.x, v2.y, v1.z,]), Vec3::from([ v2.x, v2.y, v2.z,])]},
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v1.z,]), Vec3::from([ v2.x, v2.y, v2.z,]), Vec3::from([ v2.x, v1.y, v2.z,])]},

                // North
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v2.z,]), Vec3::from([ v2.x, v2.y, v2.z,]), Vec3::from([ v1.x, v2.y, v2.z,])]},
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v2.z,]), Vec3::from([ v1.x, v2.y, v2.z,]), Vec3::from([ v1.x, v1.y, v2.z,])]},

                // West
                Tri { verts: [ Vec3::from([ v1.x, v1.y, v2.z,]), Vec3::from([ v1.x, v2.y, v2.z,]), Vec3::from([ v1.x, v2.y, v1.z,])]},
                Tri { verts: [ Vec3::from([ v1.x, v1.y, v2.z,]), Vec3::from([ v1.x, v2.y, v1.z,]), Vec3::from([ v1.x, v1.y, v1.z,])]},

                // Top
                Tri { verts: [ Vec3::from([ v1.x, v2.y, v1.z,]), Vec3::from([ v1.x, v2.y, v2.z,]), Vec3::from([ v2.x, v2.y, v2.z ])]},
                Tri { verts: [ Vec3::from([ v1.x, v2.y, v1.z,]), Vec3::from([ v2.x, v2.y, v2.z,]), Vec3::from([ v2.x, v2.y, v1.z ])]},

                // Bottom
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v2.z,]), Vec3::from([ v1.x, v1.y, v2.z,]), Vec3::from([ v1.x, v1.y, v1.z ])]},
                Tri { verts: [ Vec3::from([ v2.x, v1.y, v2.z,]), Vec3::from([ v1.x, v1.y, v1.z,]), Vec3::from([ v2.x, v1.y, v1.z ])]},
            ],
        }
    }

    /// Scales the mesh by the given scalar.
    pub fn scale(&mut self, scalar: f64) {
        for tri in &mut self.tris {
            for vert in &mut tri.verts {
                *vert = Mat3x3::identity(scalar) * *vert
            }
        }
    }

    /// Roatates the mesh about the given axis by the given angle.
    pub fn rot(&mut self, axis: Axis, angle: f64) {
        for tri in &mut self.tris {
            for vert in &mut tri.verts {
                *vert = Mat3x3::rot(angle, axis) * *vert
            }
        }
    }

    fn render(&self, draw_line: fn(f64, f64, f64, f64)) {
        for tri in &self.tris {
            tri.render(draw_line);
        }
    }
}

extern "C" {
    fn drawLine(x1: f64, y1: f64, x2: f64, y2: f64);
    fn clearCanvas();
}

fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64) {
    unsafe {
        drawLine(x1 + 250_f64, y1 + 250_f64, x2 + 250_f64, y2 + 250_f64);
    }
}

#[rustfmt::skip]
fn clear_canvas() {
    unsafe{ clearCanvas(); }
}

#[no_mangle]
pub extern "C" fn render(scalar: f64, x_angle: f64, y_angle: f64, z_angle: f64) -> f64 {
    let mut test_cube = Mesh::cube(
        Vertex {
            x: -50_f64,
            y: -50_f64,
            z: -50_f64,
        },
        Vertex {
            x: 50_f64,
            y: 50_f64,
            z: 50_f64,
        },
    );

    clear_canvas();

    test_cube.scale(scalar);

    test_cube.rot(Axis::X, x_angle);
    test_cube.rot(Axis::Y, y_angle);
    test_cube.rot(Axis::Z, z_angle);

    test_cube.render(draw_line);

    return 42_f64;
}
