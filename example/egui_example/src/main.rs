use eframe::egui;
use termport::*;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(300.0,100.0));
    eframe::run_native(
        "Terminal App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("start terminal").clicked() {
                let mut term = new_term(&"alacritty_driver.exe".to_string()).unwrap();
                let buf = "\x1b[0;31mHello Termport\x1b[0m".as_bytes();
                term.write(buf).unwrap();
                std::thread::spawn(move || {
                    term.wait_for_exit().unwrap();
                });
            }
        });
    }
}