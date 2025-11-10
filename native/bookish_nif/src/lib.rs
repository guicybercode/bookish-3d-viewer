use rustler::{Env, Term, Error};
use rustler::resource::ResourceArc;
use std::sync::Mutex;

rustler::init!("Elixir.Bookish3dViewer.Native", [
    load_model,
    rotate_camera,
    zoom_camera,
    pan_camera,
    reset_camera,
    get_model_info,
]);

struct ViewerResource {
    app: Mutex<bookish_3d_viewer::App>,
}

#[rustler::nif]
fn load_model(path: String) -> Result<Term, Error> {
    let mut app = bookish_3d_viewer::App::new();
    
    match std::fs::read(&path) {
        Ok(data) => {
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            
            match app.load_file_from_bytes(&data, ext) {
                Ok(_) => {
                    let resource = ResourceArc::new(ViewerResource {
                        app: Mutex::new(app),
                    });
                    Ok(rustler::types::atom::ok().encode(Env::current_env()))
                }
                Err(e) => Err(Error::Term(Box::new(format!("Failed to load model: {}", e))))
            }
        }
        Err(e) => Err(Error::Term(Box::new(format!("Failed to read file: {}", e))))
    }
}

#[rustler::nif]
fn rotate_camera(resource: ResourceArc<ViewerResource>, delta_x: f64, delta_y: f64) -> Result<Term, Error> {
    let mut app = resource.app.lock().unwrap();
    app.camera.rotate(delta_x as f32, delta_y as f32);
    Ok(rustler::types::atom::ok().encode(Env::current_env()))
}

#[rustler::nif]
fn zoom_camera(resource: ResourceArc<ViewerResource>, delta: f64) -> Result<Term, Error> {
    let mut app = resource.app.lock().unwrap();
    app.camera.zoom(delta as f32);
    Ok(rustler::types::atom::ok().encode(Env::current_env()))
}

#[rustler::nif]
fn pan_camera(resource: ResourceArc<ViewerResource>, delta_x: f64, delta_y: f64) -> Result<Term, Error> {
    let mut app = resource.app.lock().unwrap();
    app.camera.pan(delta_x as f32, delta_y as f32);
    Ok(rustler::types::atom::ok().encode(Env::current_env()))
}

#[rustler::nif]
fn reset_camera(resource: ResourceArc<ViewerResource>) -> Result<Term, Error> {
    let mut app = resource.app.lock().unwrap();
    app.camera.reset();
    Ok(rustler::types::atom::ok().encode(Env::current_env()))
}

#[rustler::nif]
fn get_model_info(resource: ResourceArc<ViewerResource>) -> Result<Term, Error> {
    let app = resource.app.lock().unwrap();
    match &app.model_info {
        Some(info) => Ok(info.format_info().encode(Env::current_env())),
        None => Ok("No model loaded".encode(Env::current_env()))
    }
}

