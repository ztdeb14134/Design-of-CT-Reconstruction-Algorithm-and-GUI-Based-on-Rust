use std::time::{SystemTime, UNIX_EPOCH};
pub struct MyApp {
    input_file_route: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file_route: String::from("请输入带重建的CT路径"),
        }
    }
}

// 实现 eframe::App trait
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CT 重建");

            // 输入框
            ui.horizontal(|ui| {
                ui.label("输入：");
                ui.text_edit_singleline(&mut self.input_file_route);
            });

            // 按钮和计数器
            if ui.button("按键1").clicked() {
                println!("按键1被按下");
            }
            if ui.button("按键2").clicked() {
                println!("按键2被按下");
            }
            ui.label(format!(
                "当前时间{}",
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => n.as_secs().to_string(),
                    Err(_) => String::from("UNKNOW"),
                }
            ));

            // 显示用户输入的内容
            ui.label(format!("输入的路径为: {}", self.input_file_route));
        });
    }
}
