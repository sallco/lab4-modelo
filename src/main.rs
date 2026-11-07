use lab4_modelo::{
    framebuffer::Framebuffer,
    obj::Obj,
    renderer::{HEIGHT, WIDTH, render},
    view::View,
};
use raylib::prelude::*;
use std::{io, path::PathBuf, process::ExitCode};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let screenshot = match args.next().as_deref() {
        None => None,
        Some("--screenshot") => Some(
            args.next()
                .ok_or("Uso: cargo run -- --screenshot ruta.png")?,
        ),
        _ => return Err("Uso: cargo run [-- --screenshot ruta.png]".into()),
    };
    if args.next().is_some() {
        return Err("Demasiados argumentos".into());
    }
    if let Some(path) = &screenshot {
        if !path.ends_with(".png") {
            return Err("La captura debe tener extensión .png".into());
        }
        if let Some(parent) = std::path::Path::new(path)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/nave1.obj");
    let obj = Obj::load(&path)
        .map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))?;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut view = View::default();
    let count = render(&mut framebuffer, &obj, &view);
    println!(
        "OBJ cargado: {} vértices, {} índices; {count} triángulos dibujados.",
        obj.vertices.len(),
        obj.indices.len()
    );

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Lab 4 | Flechas/Q/E: girar | +/-: zoom | WASD: mover | R: reiniciar | Esc: salir")
        .build();
    window.set_target_fps(60);
    let mut texture = window.load_texture_from_image(&thread, &framebuffer.image)?;
    while !window.window_should_close() {
        if view.update(window.get_frame_time(), |key| window.is_key_down(key)) {
            render(&mut framebuffer, &obj, &view);
            let pixels: Vec<u8> = framebuffer
                .image
                .get_image_data()
                .iter()
                .flat_map(|color| [color.r, color.g, color.b, color.a])
                .collect();
            texture.update_texture(&pixels)?;
        }
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture(&texture, 0, 0, Color::WHITE);
        if let Some(path) = &screenshot {
            // Enviar el lote antes de leer la pantalla y antes de intercambiar buffers.
            unsafe { raylib::ffi::rlDrawRenderBatchActive() };
            let image = drawing.load_image_from_screen(&thread);
            image.export_image(path);
            if !std::path::Path::new(path).is_file() {
                return Err("No se pudo guardar la captura".into());
            }
            println!("Captura guardada: {path}");
            break;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
