use lab4_modelo::{
    framebuffer::Framebuffer,
    obj::Obj,
    renderer::{HEIGHT, WIDTH, render},
};
use raylib::prelude::*;
use std::{io, path::PathBuf, process::ExitCode};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/nave1.obj");
    let obj = Obj::load(&path)
        .map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))?;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let count = render(&mut framebuffer, &obj);
    println!(
        "OBJ cargado: {} vértices, {} índices; {count} triángulos dibujados.",
        obj.vertices.len(),
        obj.indices.len()
    );

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Lab 4: Modelo OBJ | Nave | Esc: salir")
        .build();
    window.set_target_fps(60);
    let texture = window.load_texture_from_image(&thread, &framebuffer.image)?;
    while !window.window_should_close() {
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture(&texture, 0, 0, Color::WHITE);
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
