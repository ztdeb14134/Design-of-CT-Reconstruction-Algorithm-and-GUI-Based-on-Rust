use egui::TextureHandle;
use ndarray::Axis;

use crate::{
    projection::{divide_circle, project_image},
    readct::{load_texture_from_file, readct, save_as_png, save_layer_as_image},
    rebuild_dbp::reconstruct_image_dbp,
    rebuild_dsp::reconstruct_image_dsp,
    sart::sart_reconstruction,
};
use std::time::{SystemTime, UNIX_EPOCH};
enum AppState {
    Home,
    OneProjection,
    Rebuild,
    Dbp,
    Dsp,
    Showimg,
    Sart,
}

pub struct MyApp {
    input_file_path: String,  //输入文件路径
    cols: String,             //长
    rows: String,             //宽
    frames: String,           //高
    bytes_per_pixel: String,  //单位像素占用字节
    appstate: AppState,       //app状态
    ct: Vec<Vec<Vec<f32>>>,   //ct数据
    sl: String,               //切片层数
    projectionangles: String, //投影角度个数
    pjimage: Vec<Vec<f32>>,   //投影数据
    texture1: Option<TextureHandle>,
    texture2: Option<TextureHandle>,
    reconstruction_method: String,
    use_time: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_file_path: String::from("src\\CT_600_600_10.bin"),
            cols: "600".to_string(),
            rows: "600".to_string(),
            frames: "10".to_string(),
            bytes_per_pixel: "4".to_string(),
            appstate: AppState::Home,
            ct: vec![vec![vec![]]],
            sl: "0".to_string(),
            projectionangles: "30".to_string(),
            pjimage: vec![],
            texture1: None,
            texture2: None,
            reconstruction_method: String::new(),
            use_time: 0.0,
        }
    }
}
impl MyApp {
    fn readtime(self: &mut Self) {
        self.use_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            - self.use_time;
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
                        // save_layer_as_image(&image_3d, 100, "src\\raw.png").unwrap();
                        // println!("{:?}", image_3d)
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
                    ui.horizontal(|ui| {
                        ui.label(format!("输入想要投影的数量"));
                        ui.text_edit_singleline(&mut self.projectionangles)
                    });
                    if ui.button("切片后进行投影").clicked() {
                        println!("正在切片保存");
                        let image: Vec<Vec<f32>> =
                            save_layer_as_image(&self.ct, self.sl.parse().unwrap(), "src\\raw.png")
                                .expect("切片保存失败");
                        println!("正在投影");
                        self.pjimage = project_image(
                            image,
                            divide_circle(self.projectionangles.parse().unwrap()),
                        );
                        println!("投影成功");
                        save_as_png(self.pjimage.clone(), "src\\project.png");
                        println!("{}", self.pjimage[0].len());
                        self.texture1 = Some(load_texture_from_file(ctx, "src/raw.png"));
                        self.texture2 = Some(load_texture_from_file(ctx, "src/project.png"));
                        self.appstate = AppState::Rebuild;
                    }
                });
            }
            AppState::Rebuild => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("请选择重建方式");
                    if ui.button("直接反投影").clicked() {
                        self.appstate = AppState::Dbp;
                    }
                    if ui.button("联合代数重建反投影").clicked() {
                        self.appstate = AppState::Sart;
                    }
                    if ui.button("滤波反投影").clicked() {
                        self.appstate = AppState::Dsp;
                    }
                });
            }
            AppState::Dbp => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.readtime();
                    ui.heading("正在进行直接反投影");
                    println!("正在进行直接反投影");
                    let rebuild_ct = reconstruct_image_dbp(self.pjimage.clone(), 600);
                    save_as_png(rebuild_ct, "src/Dbp.png");
                    println!("直接反投影成功,图片已保存到src/Dbp.png");
                    self.texture1 = Some(load_texture_from_file(ctx, "src/raw.png"));
                    self.texture2 = Some(load_texture_from_file(ctx, "src/Dbp.png"));
                    self.reconstruction_method = "直接反投影".to_string();
                    self.readtime();
                    self.appstate = AppState::Showimg;
                });
            }
            AppState::Dsp => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.readtime();
                    ui.heading("正在进行滤波反投影");
                    println!("正在进行滤波反投影");
                    let rebuild_ct = reconstruct_image_dsp(self.pjimage.clone(), 600);
                    save_as_png(rebuild_ct, "src/Dsp.png");
                    println!("滤波反投影成功,图片已保存到src/Dsp.png");
                    self.texture1 = Some(load_texture_from_file(ctx, "src/raw.png"));
                    self.texture2 = Some(load_texture_from_file(ctx, "src/Dsp.png"));
                    self.reconstruction_method = "滤波反投影".to_string();
                    self.readtime();
                    self.appstate = AppState::Showimg;
                });
            }
            AppState::Sart => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("正在进行代数重建");
                    println!("正在进行代数重建");
                    self.readtime();
                    let angles: Vec<f32> = (0..self.pjimage.len())
                        .map(|i| i as f32 * 180.0 / self.pjimage.len() as f32)
                        .collect();
                    let rebuild_ct = sart_reconstruction(self.pjimage.clone(), angles, None);
                    // 将 Array2<f32> 转换为 Vec<Vec<f32>>
                    let rebuild_ct_vec = rebuild_ct
                        .axis_iter(Axis(0))
                        .map(|row| row.to_vec())
                        .collect();
                    save_as_png(rebuild_ct_vec, "src/sart.png");
                    println!("代数重建成功,图片已保存到src/sart.png");
                    self.texture1 = Some(load_texture_from_file(ctx, "src/raw.png"));
                    self.texture2 = Some(load_texture_from_file(ctx, "src/sart.png"));
                    self.reconstruction_method = "代数重建反投影".to_string();
                    self.readtime();
                    self.appstate = AppState::Showimg;
                });
            }
            AppState::Showimg => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("投影数据与原数据对比展示");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if let Some(texture) = &self.texture1 {
                                ui.image((texture.id(), texture.size_vec2()));
                                ui.label(format!("第{}层切片的原图 ", self.sl));
                            }
                        });

                        ui.vertical(|ui| {
                            if let Some(texture) = &self.texture2 {
                                ui.image((texture.id(), texture.size_vec2()));
                                ui.label(format!(
                                    " 第{}层切片通过{}的重建图像, {} 个投影角度",
                                    self.sl, self.reconstruction_method, self.projectionangles
                                ));
                            }
                        });
                    });
                    // 重建时间显示
                    ui.label(format!(
                        "{} 重建耗时：{:.4} 秒",
                        self.reconstruction_method, self.use_time,
                    ));

                    if ui.button("回到主页").clicked() {
                        self.appstate = AppState::Home;
                    }
                    if ui.button("重新选择投影方式").clicked() {
                        self.appstate = AppState::OneProjection;
                    }
                });
            }
        }
    }
}
