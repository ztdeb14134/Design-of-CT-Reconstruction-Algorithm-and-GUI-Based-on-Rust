use nalgebra::{Matrix2, Vector2};
use std::f32::consts::PI;

pub fn project_image(image: Vec<Vec<f32>>, angles: Vec<f32>) -> Vec<Vec<f32>> {
    let rows = image.len();
    let cols = image[0].len();

    // 将图像转为矩阵形式
    let mut img_matrix = Vec::new();
    for row in image.iter() {
        img_matrix.push(row.clone());
    }

    // 结果存储投影值
    let mut projections: Vec<Vec<f32>> = Vec::new();
    println!("投影角度:");

    for &angle in angles.iter() {
        // 将角度转为弧度
        let angle_in_radians = (angle as f32) * PI / 180.0;
        print!("{}  ", angle_in_radians);
        // 计算旋转矩阵，使用 2x2 矩阵
        let cos_angle = angle_in_radians.cos();
        let sin_angle = angle_in_radians.sin();
        let rotation_matrix = Matrix2::new(cos_angle, -sin_angle, sin_angle, cos_angle);

        // 计算投影
        let mut projection = vec![0.0; rows];

        for r in 0..rows {
            for c in 0..cols {
                // 使用 Vector2 来表示二维点，旋转时只考虑 x 和 y 坐标
                let point =
                    Vector2::new(c as f32 - cols as f32 / 2.0, r as f32 - rows as f32 / 2.0);

                // 旋转点
                let rotated_point = rotation_matrix * point;

                // 投影到某一方向（x 或 y）上
                let projection_value = rotated_point.x + cols as f32 / 2.0; // 投影到x轴
                if projection_value >= 0.0 && projection_value < cols as f32 {
                    projection[r] += image[r][projection_value as usize];
                }
            }
        }
        let max_value = projection.iter().cloned().fold(f32::MIN, f32::max);
        let min_value = projection.iter().cloned().fold(f32::MAX, f32::min);

        if max_value > min_value {
            for value in projection.iter_mut() {
                *value = (*value - min_value) / (max_value - min_value); // 归一化到 [0, 1]
            }
        }
        projections.push(projection);
    }
    projections
}

pub fn divide_circle(n: usize) -> Vec<f32> {
    let step = 180.0 / n as f32;
    (0..n as usize).map(|i| i as f32 * step).collect()
}
