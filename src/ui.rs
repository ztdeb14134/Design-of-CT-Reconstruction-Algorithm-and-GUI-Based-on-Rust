use std::time::{SystemTime, UNIX_EPOCH};

use crate::readct::readct;
pub struct MyApp {
    input_file_path: String,
    cols: usize,
    rows: usize,
    frames: usize,
    bytes_per_pixel: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file_path: String::from("请输入带重建的CT路径"),
            cols: 600,
            rows: 600,
            frames: 600,
            bytes_per_pixel: 16,
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
                ui.label("输入路径：");
                ui.text_edit_singleline(&mut self.input_file_path);
            });
            ui.horizontal(|ui| {
                ui.label("输入长");
                ui.text_edit_singleline(&mut self.cols.to_string())
            });
            ui.horizontal(|ui| {
                ui.label("输入宽");
                ui.text_edit_singleline(&mut self.rows.to_string())
            });
            ui.horizontal(|ui| {
                ui.label("输入高");
                ui.text_edit_singleline(&mut self.frames.to_string())
            });
            ui.horizontal(|ui| {
                ui.label("输入每像素占储存数");
                ui.text_edit_singleline(&mut self.bytes_per_pixel.to_string())
            });
            
            // 按钮和计数器
            if ui.button("读取ct").clicked() {
                println!("读取ct按键被按下");
                let image_3d: Vec<Vec<Vec<u8>>> = match readct(
                    self.cols,
                    self.rows,
                    self.frames,
                    self.bytes_per_pixel,
                    &self.input_file_path,
                ) {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Error occurred");
                        vec![] // Provide a default value
                    }
                };
                println!("{:?}", image_3d)
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
            ui.label(format!("输入的路径为: {}", self.input_file_path));
        });
    }
}
