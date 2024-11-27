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





#[cfg(test)]
mod printbin {
    #[test]
    fn build_test() {
        use rand::Rng;
        use std::fs::File;
        use std::io::{BufWriter, Write};
        const WIDTH: usize = 600;
        const HEIGHT: usize = 600;
        const DEPTH: usize = 246;

        // 创建三维数据数组
        let mut data = vec![0.0f32; WIDTH * HEIGHT * DEPTH];

        // 随机生成两个明亮区块在每个断层中
        let mut rng = rand::thread_rng();
        for z in 0..DEPTH {
            for _ in 0..2 {
                // 明亮区块的中心和范围
                let center_x = rng.gen_range(50..(WIDTH - 50)) as usize;
                let center_y = rng.gen_range(50..(HEIGHT - 50)) as usize;
                let size = rng.gen_range(20..50); // 区块大小

                for x in (center_x - size)..=(center_x + size) {
                    for y in (center_y - size)..=(center_y + size) {
                        if x < WIDTH && y < HEIGHT {
                            let index = z * WIDTH * HEIGHT + y * WIDTH + x;
                            data[index] = 255.0; // 明亮值
                        }
                    }
                }
            }
        }

        // 将数据写入二进制文件
        let file = File::create("src\\ct_data.bin").expect("msg");
        let mut writer = BufWriter::new(file);

        for &value in data.iter() {
            writer.write_all(&value.to_le_bytes()).expect("msg");
        }

        println!("CT数据已成功生成到 ct_data.bin");
    }
}
