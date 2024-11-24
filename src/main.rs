use ct_rebuild_project::ttf::setup_custom_fonts;
use ct_rebuild_project::ui::MyApp;
fn main() -> Result<(), eframe::Error> {
    // 启动 egui 框架
    eframe::run_native(
        "医学图像处理大作业-CT图像重建",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
