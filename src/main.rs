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
    #[test]
    fn process_ct() {
        use std::fs::File;
        use std::io::{Read, Write};
        use std::path::Path;
        let input_path = Path::new("src/CT_3.bin");
        let output_path = Path::new("src/CT_600_600_10.bin");

        // 原始尺寸
        const WIDTH: usize = 600;
        const HEIGHT: usize = 600;
        const DEPTH: usize = 246;

        // 目标层数
        const TARGET_LAYERS: usize = 10;

        // 读取原始文件
        let mut file = File::open(input_path).unwrap();
        let mut buffer = vec![0u8; WIDTH * HEIGHT * DEPTH * 4]; // f32 = 4 bytes
        file.read_exact(&mut buffer).unwrap();

        // 将字节转换为f32
        let mut data = Vec::with_capacity(WIDTH * HEIGHT * DEPTH);
        for chunk in buffer.chunks_exact(4) {
            let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            data.push(value);
        }

        // 计算采样间隔
        let step = (DEPTH as f32 / TARGET_LAYERS as f32).ceil() as usize;

        // 创建输出缓冲区
        let mut output_data = Vec::with_capacity(WIDTH * HEIGHT * TARGET_LAYERS);

        // 提取目标层
        for layer in 0..TARGET_LAYERS {
            let source_layer = (layer * step).min(DEPTH - 1);
            let start = source_layer * WIDTH * HEIGHT;
            let end = start + WIDTH * HEIGHT;
            output_data.extend_from_slice(&data[start..end]);
        }

        // 将f32转回字节
        let mut output_buffer = Vec::with_capacity(WIDTH * HEIGHT * TARGET_LAYERS * 4);
        for value in output_data {
            output_buffer.extend_from_slice(&value.to_le_bytes());
        }

        // 写入输出文件
        let mut output_file = File::create(output_path).unwrap();
        output_file.write_all(&output_buffer).unwrap();
    }
}
