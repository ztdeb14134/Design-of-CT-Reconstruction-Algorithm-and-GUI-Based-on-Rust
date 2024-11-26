use std::f32::consts::PI;

use nalgebra::ComplexField;

pub fn reconstruct_image(projections: Vec<Vec<f32>>, image_size: usize) -> Vec<Vec<f32>> {
    let n_angles = projections.len(); // 角度数
    let step_angle = PI / n_angles as f32; // 每个角度之间的间隔（弧度制）

    // 初始化重建图像为 0
    let mut reconstructed_image = vec![vec![1.0; image_size]; image_size];

    println!("重建角度");
    for (i, projection) in projections.iter().enumerate() {
        let angle = i as f32 * step_angle; // 当前角度
        print!("{} ", angle);
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        // 对每个投影值进行反向投影
        for (r, &value) in projection.iter().enumerate() {
            let r_f32 = r as f32 - image_size as f32 / 2.0; // 将索引转换为坐标

            for x in 0..image_size {
                for y in 0..image_size {
                    // 计算点 (x, y) 在当前投影方向上的坐标
                    let x_f32 = x as f32 - image_size as f32 / 2.0;
                    let y_f32 = y as f32 - image_size as f32 / 2.0;

                    let projection_coordinate = r_f32 - (x_f32 * cos_angle + y_f32 * sin_angle);

                    // 如果坐标落在当前投影值中，则进行叠加
                    if projection_coordinate.abs() < 0.5 {
                        reconstructed_image[599 - y][x] += value.powf(0.76)
                            + reconstructed_image[599 - y][x].powf(0.2) * value.powf(2.0);
                    }
                }
            }
        }
    }
    for i in 0..image_size {
        for j in 0..image_size {
            reconstructed_image[i][j] = (reconstructed_image[i][j].powf(1.7)).powf(1.77);
        }
    }
    // for i in reconstructed_image.iter() {
    //     for j in i.iter() {
    //         let _ = j.powf(0.1);
    //     }
    // }

    let max_value = reconstructed_image
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f32::MIN, f32::max);
    let min_value = reconstructed_image
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(f32::MAX, f32::min);

    for row in reconstructed_image.iter_mut() {
        for pixel in row.iter_mut() {
            *pixel = (*pixel - min_value) / (max_value - min_value); // 归一化到 [0, 1]
        }
    }

    reconstructed_image
}
// fn negative_image(image: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
//     // 动态计算图片的最大值
//     let max_value = image
//         .iter()
//         .flat_map(|row| row.iter())
//         .cloned()
//         .fold(f32::MIN, f32::max);

//     // 生成反片图片
//     image
//         .iter()
//         .map(|row| row.iter().map(|&pixel| max_value - pixel).collect())
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruction() {
        // 模拟投影数据，构造一个简单的例子
        let projections = vec![
            vec![1.0, 2.0, 3.0, 2.0, 1.0], // 0度投影
            vec![0.5, 1.0, 2.0, 1.0, 0.5], // 45度投影
            vec![1.0, 2.0, 3.0, 2.0, 1.0], // 90度投影
        ];

        let reconstructed_image = reconstruct_image(projections, 5);

        // 打印重建结果
        for row in reconstructed_image.iter() {
            println!("{:?}", row);
        }

        // 检查重建后的图像尺寸
        assert_eq!(reconstructed_image.len(), 5);
        assert_eq!(reconstructed_image[0].len(), 5);
    }
}
