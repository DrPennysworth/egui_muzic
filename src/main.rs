use nfd::Response;
use lazy_static::{lazy_static};
use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, egui::{self, mutex::Mutex}};


pub struct UISave{
    _lbl_test: String,
    slider_test: f32,
    _clip: String,
}
impl UISave {
    pub fn new(_name: String) -> Self {
        Self {
            _lbl_test: String::new(),
            slider_test: 0.0,
            _clip: "<path to file?>".to_string(),
        }
    }
}
lazy_static! {
    static ref SAVED_VARS: Mutex<UISave> = Mutex::new(UISave::new("main".to_owned()));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example.system())
        .run();
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(egui_context: ResMut<EguiContext>) {
    

    egui::Window::new("Properties").show(egui_context.ctx(), |ui| {
        let mut guard = SAVED_VARS.lock();
        
        ui.label(guard._clip.to_string());
        if ui.button("Open Sound Clip").clicked(){
            let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                panic!("{}", e);
            });          
            match result {
                Response::Okay(file_path) => {guard._clip = String::from(file_path)},
                Response::OkayMultiple(files) => println!("Files {:?}", files),
                Response::Cancel => println!("User canceled"),
            }
        }

        ui.vertical(|ui|
            ui.add(egui::Slider::new(&mut guard.slider_test, 0.0..=1.0)),
        );
    });
}