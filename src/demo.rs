//! Demonstration of the meenle-noonle library. To use, first call [generate_background], then pick your mesh with
//! [set_mesh], and use [get_buffer] to get the frame buffer where the output will be rendered. Call [render_spin]
//! every frame to update the frame buffer. NOTE: The demo is not thread safe. You must wait for [set_mesh] to finish
//! execution before rendering.
//!
//! Example usage:
//! ```
//! meenle_noonle::generate_background();
//! meenle_noonle::demo::set_mesh(0);
//! pFFB = meenle_noonle::get_buffer();
//!
//! loop {
//!     meenle_noonle::demo::render_spin(epoch, 5.0);
//!     sleep(1.0 / 60.0);
//! }
//! ```

use crate::meshes;
use crate::*;
use core::f64::consts::TAU;

static mut DEMO_MESH: Option<Mesh> = None;

#[no_mangle]
pub extern "C" fn set_mesh(id: u32) {
    unsafe {
        match id {
            0 => {
                DEMO_MESH = Some(meshes::monkey());
                if let Some(ref mut demo_mesh) = DEMO_MESH {
                    demo_mesh.scale(100.0);
                    demo_mesh.rot(Axis::X, TAU / 2.0);
                }
            }
            1 => {
                DEMO_MESH = Some(meshes::icosphere());
                if let Some(ref mut demo_mesh) = DEMO_MESH {
                    demo_mesh.scale(50.0);
                    demo_mesh.rot(Axis::X, TAU / 2.0);
                }
            }
            2 => {
                DEMO_MESH = Some(
                    Mesh::cube(
                        Vec3::from([-50.0, -50.0, -50.0]),
                        Vec3::from([50.0, 50.0, 50.0]),
                    )
                    .into(),
                )
                .into()
            }
            _ => (),
        }
    }
}

#[no_mangle]
pub extern "C" fn scale_mesh_to_screen(_proportion: f64) {
    unsafe {
        if let Some(ref mut demo_mesh) = DEMO_MESH {
            let oba = demo_mesh
                .tris
                .iter()
                .flat_map(|tri| tri.verts)
                .flat_map(|vert| [vert.x, vert.y, vert.z])
                .reduce(f64::max);
            dbg!(oba);
        }
    }
}

/// Render the spinning mesh animation into the frame buffer.
#[no_mangle]
pub extern "C" fn render_spin(time_seconds: f64, rotrate: f64) {
    unsafe {
        if let Some(ref mut demo_mesh) = DEMO_MESH {
            let mut opa = demo_mesh.clone();
            opa.rot(Axis::Y, (time_seconds * TAU / rotrate) % TAU);
            fill_buffer();
            opa.render();
        }
    }
}
