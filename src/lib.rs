//! Meenle_Noonle is my software renderer demo, built to help me learn things I didn't previously know.
//! It is a small Rust library with no dependencies, targeting wasm. It exports functions to act on an
//! internal frame buffer ([render], [draw_line]), as well as a function to get a pointer to the frame buffer
//! itself ([get_buffer]).

use std::ops::Mul;
mod meshes;

// dimensions for the canvas
const WIDTH: usize = 500;
const HEIGHT: usize = 500;

/// Frame buffer.
static mut BUFFER: [[Pixel; WIDTH]; HEIGHT] = [[Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
}; WIDTH]; HEIGHT];

/// Frame buffer with the pretty background pattern. Used to clear the scene.
static mut BG_BUFFER: [[Pixel; WIDTH]; HEIGHT] = unsafe { BUFFER };

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Vertex = Vec3;
/// Pixel for the frame buffer. RGBA color, to match HTML canvas' buffer format.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    const fn _black() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    const fn white() -> Pixel {
        Pixel {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
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

/// Simple 3x3 Matrix for graphics maths.
pub struct Mat3x3 {
    pub mat: [[f64; 3]; 3],
}

impl Mat3x3 {
    /// This poorly named function actually gives the 3x3 identity matrix scaled by the parameter `val`.
    pub const fn identity(val: f64) -> Mat3x3 {
        Mat3x3 {
            mat: [
                [val, 0_f64, 0_f64],
                [0_f64, val, 0_f64],
                [0_f64, 0_f64, val],
            ],
        }
    }

    /// Gives a rotation matrix to rotate a vertex about the origin.
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

///Triangle.
#[derive(Debug, Copy, Clone)]
pub struct Tri {
    pub verts: [Vertex; 3],
}

impl Tri {
    /// Draws itself into the framebuffer.
    fn render(&self) {
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

    /// Roatates the mesh.
    pub fn rot(&mut self, axis: Axis, angle: f64) {
        for tri in &mut self.tris {
            for vert in &mut tri.verts {
                *vert = Mat3x3::rot(angle, axis) * *vert;
            }
        }
    }

    /// Draws itself into the framebuffer.
    fn render(&self) {
        for tri in &self.tris {
            tri.render();
        }
    }
}

/// Plots a single pixel into the frame buffer.
fn plot_pixel(x: usize, y: usize, pixel: &Pixel) {
    unsafe {
        *BUFFER
            .get_mut(y)
            .unwrap_or(&mut BUFFER[0])
            .get_mut(x)
            .unwrap_or(&mut BUFFER[0][0]) = *pixel;
    }
}

/// Uses Bresenham's algorithm to draw a line.
#[no_mangle]
pub extern "C" fn draw_line(x0: f64, y0: f64, x1: f64, y1: f64) {
    let mut x0 = x0 as i32 + (WIDTH / 2) as i32;
    let mut y0 = y0 as i32 + (WIDTH / 2) as i32;
    let mut x1 = x1 as i32 + (WIDTH / 2) as i32;
    let mut y1 = y1 as i32 + (WIDTH / 2) as i32;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();
    let mut error = dx / 2;
    let ystep = if y0 < y1 { 1 } else { -1 };
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            plot_pixel(y as usize, x as usize, &Pixel::white());
        } else {
            plot_pixel(x as usize, y as usize, &Pixel::white());
        }
        error -= dy;
        if error < 0 {
            y += ystep;
            error += dx;
        }
    }
}

/// Generates the pretty background pattern.
#[no_mangle]
pub extern "C" fn generate_background() {
    unsafe {
        for (idx_row, row) in BG_BUFFER.iter_mut().enumerate() {
            for (idx_col, pxl) in row.iter_mut().enumerate() {
                *pxl = Pixel {
                    r: ((255_f64 / HEIGHT as f64) * idx_row as f64) as u8,
                    g: ((255_f64 / WIDTH as f64) * idx_col as f64) as u8,
                    b: ((-(255_f64 / HEIGHT as f64) * idx_row as f64) + 255_f64) as u8,
                    a: 255,
                };
            }
        }
    }
}

/// Fills the frame buffer with a pretty pattern.
#[no_mangle]
pub extern "C" fn fill_buffer() {
    unsafe {
        BUFFER = BG_BUFFER;
    }
}

/// Gets a pointer to the frame buffer.
#[no_mangle]
pub extern "C" fn get_buffer() -> &'static [[Pixel; WIDTH]; HEIGHT] {
    unsafe { &BUFFER }
}

/// Renders the demo into the frame buffer.
#[no_mangle]
pub extern "C" fn render(scalar: f64, x_angle: f64, y_angle: f64, z_angle: f64) {
    let mut demo_mesh = meshes::monkey();
    demo_mesh.scale(50_f64);

    demo_mesh.scale(scalar);


    demo_mesh.rot(Axis::X, x_angle);
    demo_mesh.rot(Axis::Y, y_angle);
    demo_mesh.rot(Axis::Z, z_angle);

    fill_buffer();
    demo_mesh.render();
}
