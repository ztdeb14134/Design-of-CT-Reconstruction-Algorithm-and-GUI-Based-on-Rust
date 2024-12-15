use ndarray::Array2;
use std::f32::consts::PI;

const IMAGE_SIZE: usize = 600;
const NUM_ITERATIONS: usize = 100; // 根据实际需求调整迭代次数
const LAMBDA: f32 = 0.25; // 更新步长系数

pub fn sart_reconstruction(
    projections: Vec<Vec<f32>>,
    angles: Vec<f32>, // 输入投影角度数组
    num_iterations: Option<usize>,
) -> Array2<f32> {
    let num_iterations = num_iterations.unwrap_or(NUM_ITERATIONS);
    let mut image = Array2::<f32>::zeros((IMAGE_SIZE, IMAGE_SIZE));
    let center = IMAGE_SIZE as f32 / 2.0;

    for _ in 0..num_iterations {
        for (proj_idx, projection) in projections.iter().enumerate() {
            let angle = angles[proj_idx] * PI / 180.0; // 将角度转换为弧度
            let cos_angle = angle.cos();
            let sin_angle = angle.sin();

            for x in 0..IMAGE_SIZE {
                for y in 0..IMAGE_SIZE {
                    // 将像素坐标中心化
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    let radon_x = dx * cos_angle - dy * sin_angle;

                    // 映射到投影索引，并确保索引有效
                    let projection_len = projection.len() as f32;
                    let scale = IMAGE_SIZE as f32 / projection_len;
                    let radon_idx = ((radon_x / scale) + (projection_len / 2.0)) as isize;

                    if radon_idx >= 0 && (radon_idx as usize) < projection.len() {
                        // 插值计算
                        let idx_low = radon_idx as usize;
                        let idx_high = (radon_idx + 1).min(projection.len() as isize - 1) as usize;
                        let weight_high = radon_x.fract().abs();
                        let weight_low = 1.0 - weight_high;

                        let interpolated_value =
                            weight_low * projection[idx_low] + weight_high * projection[idx_high];

                        // 更新公式
                        let estimate = image[[y, x]];
                        let update = (interpolated_value - estimate) / projections.len() as f32;
                        image[[y, x]] += LAMBDA * update;
                    }
                }
            }
        }
    }

    // 归一化图像
    let max_value = image.iter().cloned().fold(f32::MIN, f32::max);
    if max_value > 0.0 {
        image.mapv_inplace(|v| v / max_value);
    }

    // Gamma 校正
    image.mapv_inplace(|v| v.powf(1.2));

    image
}
