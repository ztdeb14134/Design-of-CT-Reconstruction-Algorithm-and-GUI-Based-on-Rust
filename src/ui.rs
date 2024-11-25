use std::time::{SystemTime, UNIX_EPOCH};

use crate::readct::{readct, save_layer_as_image};
enum AppState {
    Home,
    OneProjection,
    AllProjection,
}
pub struct MyApp {
    input_file_path: String,
    cols: String,
    rows: String,
    frames: String,
    bytes_per_pixel: String,
    nextstate: AppState,
    appstate: AppState,
    ct: Vec<Vec<Vec<f32>>>,
    sl: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file_path: String::from("src\\CT_3.bin"),
            cols: "600".to_string(),
            rows: "600".to_string(),
            frames: "246".to_string(),
            bytes_per_pixel: "4".to_string(),
            appstate: AppState::Home,
            nextstate: AppState::OneProjection,
            ct: vec![vec![vec![]]],
            sl: "0".to_string(),
        }
    }
}

// 实现 eframe::App trait
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.appstate {
            AppState::Home => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("CT 重建");
                    ui.horizontal(|ui| {
                        ui.label("输入路径：");
                        ui.text_edit_singleline(&mut self.input_file_path);
                    });
                    ui.horizontal(|ui| {
                        ui.label("输入长");
                        ui.text_edit_singleline(&mut self.cols)
                    });
                    ui.horizontal(|ui| {
                        ui.label("输入宽");
                        ui.text_edit_singleline(&mut self.rows)
                    });
                    ui.horizontal(|ui| {
                        ui.label("输入高");
                        ui.text_edit_singleline(&mut self.frames)
                    });
                    ui.horizontal(|ui| {
                        ui.label("输入每像素占储存数");
                        ui.text_edit_singleline(&mut self.bytes_per_pixel)
                    });

                    // 按钮和计数器
                    if ui.button("读取ct").clicked() {
                        println!("读取ct按键被按下");
                        self.ct = match readct(
                            self.cols.parse().unwrap(),
                            self.rows.parse().unwrap(),
                            self.frames.parse().unwrap(),
                            self.bytes_per_pixel.parse().unwrap(),
                            &self.input_file_path,
                        ) {
                            Ok(i) => {
                                println!("读取成功");
                                self.appstate = AppState::OneProjection;
                                i
                            }
                            Err(_) => {
                                println!("Error occurred");
                                vec![] // Provide a default value
                            }
                        };
                        // save_layer_as_image(&image_3d, 100, "src\\666.png").unwrap();
                        // println!("{:?}", image_3d)
                    }
                    if ui.button("切片反投影").clicked() {
                        self.nextstate = AppState::OneProjection
                    }
                    if ui.button("全部反投影").clicked() {
                        self.nextstate = AppState::AllProjection
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
            AppState::OneProjection => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("切片并投影");
                    ui.horizontal(|ui| {
                        ui.label(format!("输入想要切片的层(0-{})", self.frames));
                        ui.text_edit_singleline(&mut self.sl)
                    });
                    if ui.button("切片后进行投影").clicked() {
                        println!("正在切片保存");
                        let image: Vec<Vec<f32>> =
                            save_layer_as_image(&self.ct, self.sl.parse().unwrap(), "src\\666.png")
                                .expect("切片保存失败");
                        println!("正在投影");
                    }
                });
            }
            AppState::AllProjection => {}
        }
    }
}
