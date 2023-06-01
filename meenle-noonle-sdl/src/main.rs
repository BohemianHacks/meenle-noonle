use std::error::Error;

use sdl2::keyboard::Keycode;

const MN_PXL_FMT: sdl2::pixels::PixelFormatEnum = sdl2::pixels::PixelFormatEnum::ABGR8888; // RGBA

fn main() -> Result<(), Box<dyn Error>> {
    let mut mesh_idx = 0;
    meenle_noonle::generate_background();
    meenle_noonle::demo::set_mesh(mesh_idx);

    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let mut event_pump = sdl.event_pump()?;
    let window = video
        .window(
            "Meenle-Noonle",
            meenle_noonle::WIDTH as u32,
            meenle_noonle::HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;

    let mn_fb = meenle_noonle::get_buffer();
    // coerce mn_fb to slice from 1M char array
    let mn_fb: *mut [u8; 1_000_000] = unsafe { std::mem::transmute(mn_fb.as_ptr()) };
    let mn_fb = &unsafe { &mut *mn_fb }[..];

    let texture_creator = canvas.texture_creator();
    let mut mn_texture = texture_creator.create_texture_streaming(
        MN_PXL_FMT,
        meenle_noonle::WIDTH as u32,
        meenle_noonle::HEIGHT as u32,
    )?;

    // every frame
    'main_loop: loop {
        let epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        
        meenle_noonle::demo::render_spin(epoch, 5.0);
        mn_texture.with_lock(None, |buf, _| buf.copy_from_slice(mn_fb))?;
        canvas.copy(&mn_texture, None, None)?;
        
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Up => {
                        mesh_idx += 1;
                        meenle_noonle::demo::scale_mesh_to_screen(0.0);
                        meenle_noonle::demo::set_mesh(mesh_idx);
                    }
                    Keycode::Down => {
                        mesh_idx -= 1;
                        meenle_noonle::demo::scale_mesh_to_screen(0.0);
                        meenle_noonle::demo::set_mesh(mesh_idx);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        
        canvas.present();
    }

    Ok(())
}
