use std::fs::File;
use std::io::Read;

pub fn readct(
    cols: usize,
    rows: usize,
    frames: usize,
    bytes_per_pixel: usize,
    path: &str,
) -> Result<Vec<Vec<Vec<f32>>>, std::io::Error> {
    // 1. 图像维度信息
    // let cols = 512; // 图像宽度
    // let rows = 512; // 图像高度
    // let frames = 1; // 帧数 (深度)
    // let bytes_per_pixel = 1; // 每像素字节数 (1 表示 8 位图像，2 表示 16 位图像)

    println!("{},{},{},{},{}", cols, rows, frames, bytes_per_pixel, path);
    // 3. 计算数据总大小
    let total_size = (cols * rows * frames * bytes_per_pixel) as usize;
    let mut byte_buffer = vec![0u8; total_size];
    let mut file = File::open(path)?;
    file.read_exact(&mut byte_buffer)?;

    // 将字节缓冲区转为f32缓冲区
    let float_buffer: Vec<f32> = byte_buffer
        .chunks_exact(4) // 每4个字节为一个f32
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap())) // 转换为小端格式的f32
        .collect();

    // 5. 将数据保存到三维向量
    let mut image_3d = vec![vec![vec![0f32; cols]; rows]; frames];

    for frame in 0..frames {
        for row in 0..rows {
            for col in 0..cols {
                let index = (frame * rows * cols + row * cols + col) as usize;
                image_3d[frame][row][col] = float_buffer[index];
            }
        }
    }

    println!("");

    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;

    // 1. 找到最小值和最大值
    for frame in &image_3d {
        for row in frame {
            for &pixel in row {
                if pixel < min_val {
                    min_val = pixel;
                }
                if pixel > max_val {
                    max_val = pixel;
                }
            }
        }
    }

    // 3. 归一化
    let normalized_image_3d: Vec<Vec<Vec<f32>>> = image_3d
        .into_iter()
        .map(|frame| {
            frame
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|pixel| (pixel - min_val) / (max_val - min_val) * 255.0)
                        .collect()
                })
                .collect()
        })
        .collect();

    Ok(normalized_image_3d)
}
use image::{GrayImage, Luma};

pub fn save_layer_as_image(
    image_3d: &Vec<Vec<Vec<f32>>>,
    frame_index: usize,
    output_path: &str,
) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    // 校验帧索引是否合法
    if frame_index >= image_3d.len() {
        return Err(format!("帧索引超出范围: {}", frame_index).into());
    }

    let rows = image_3d[0].len();
    let cols = image_3d[0][0].len();

    // 获取指定帧的二维图像数据
    let frame: &Vec<Vec<f32>> = &image_3d[frame_index];

    // 构建灰度图
    let mut img = GrayImage::new(cols as u32, rows as u32);
    for (y, row) in frame.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            // 映射到 [0, 255]
            let pixel_value = pixel.round() as u8;
            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    // 保存为图像文件
    img.save(output_path)?;

    println!("图像已保存到: {}", output_path);
    Ok(frame.clone())
}
