use egui::CentralPanel;

struct MyApp {
    img_bytes: Vec<u8>,
}

impl MyApp {
    pub fn new(img_bytes: Vec<u8>) -> Self {
        Self { img_bytes }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Image Bytes: {:?}", self.img_bytes));
        });
    }
}
