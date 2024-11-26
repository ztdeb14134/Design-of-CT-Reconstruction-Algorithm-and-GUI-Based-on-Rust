use std::f32::consts::PI;

pub fn project_image(image: Vec<Vec<f32>>, angles: Vec<f32>) -> Vec<Vec<f32>> {
    println!("{:?}", angles);
    let rows = image.len();
    let cols = image[0].len();

    // 确保图片不为空
    if rows == 0 || cols == 0 {
        return vec![];
    }

    let max_dim = rows.max(cols);
    let center_x = (cols as f32) / 2.0;
    let center_y = (rows as f32) / 2.0;

    angles
        .iter()
        .map(|&angle| {
            let radians = angle * PI / 180.0;

            // 投影结果
            let mut projection = vec![0.0; max_dim];

            // 遍历每个像素
            for y in 0..rows {
                for x in 0..cols {
                    // 原始坐标
                    let x_rel = x as f32 - center_x;
                    let y_rel = y as f32 - center_y;

                    // 旋转坐标
                    let x_rot = x_rel * radians.cos() - y_rel * radians.sin() + center_x;
                    let y_rot = x_rel * radians.sin() + y_rel * radians.cos() + center_y;

                    // 检查旋转后的坐标是否在边界内
                    if x_rot >= 0.0
                        && x_rot < (cols - 1) as f32
                        && y_rot >= 0.0
                        && (y_rot - 1.0) < rows as f32
                    {
                        // 插值（简单近邻采样，优化时可用双线性插值）
                        let x_nearest = x_rot.round() as usize;
                        // let y_nearest = y_rot.round() as usize;

                        // 累加到投影结果中（根据旋转后的x坐标）
                        projection[x_nearest] += image[y][x];
                    }
                }
            }

            projection
        })
        .collect()
}

pub fn divide_circle(n: usize) -> Vec<f32> {
    let step = 180.0 / n as f32;
    (0..n as usize).map(|i| i as f32 * step).collect()
}
