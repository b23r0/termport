# termport
A terminal GUI port based on the blazing fast GPU-accelerated Alacritty core.

# Get Started

```toml
[dependencies]
termport = {git = "https://github.com/b23r0/termport.git"}
```

# Example

Download and compile alacritty-driver

https://github.com/b23r0/alacritty-driver

The following is an example of use

![image]( https://github.com/b23r0/termport/blob/main/example/egui_example/termport_egui.gif)

```rs
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
```