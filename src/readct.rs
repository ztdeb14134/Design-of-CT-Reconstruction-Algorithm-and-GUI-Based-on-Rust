use std::fs::File;
use std::io::Read;

pub fn readct(
    cols: usize,
    rows: usize,
    frames: usize,
    bytes_per_pixel: usize,
    path: &str,
) -> Result<Vec<Vec<Vec<u8>>>, std::io::Error> {
    // 1. 图像维度信息
    // let cols = 512; // 图像宽度
    // let rows = 512; // 图像高度
    // let frames = 1; // 帧数 (深度)
    // let bytes_per_pixel = 1; // 每像素字节数 (1 表示 8 位图像，2 表示 16 位图像)

    // // 2. 文件路径
    // let raw_file_path = "path/to/your/file.raw"; // 替换为你的 RAW 文件路径

    // 3. 计算数据总大小
    let total_size = (cols * rows * frames * bytes_per_pixel) as usize;
    let mut buffer:Vec<u8>;
    // 4. 读取文件内容
    match File::open(path) {
        Ok(mut file) => {
            buffer = vec![0u8; total_size];
            file.read_exact(&mut buffer)?;
        }
        Err(err) => {
            println!("打开文件失败");
            return Err(err);
        }
    }

    // 5. 将数据保存到三维向量
    let mut image_3d = vec![vec![vec![0u8; cols]; rows]; frames];

    for frame in 0..frames {
        for row in 0..rows {
            for col in 0..cols {
                let index = (frame * rows * cols + row * cols + col) as usize;
                image_3d[frame][row][col] = buffer[index];
            }
        }
    }

    println!("Image data successfully organized into a 3D vector.");

    Ok(image_3d)
}
