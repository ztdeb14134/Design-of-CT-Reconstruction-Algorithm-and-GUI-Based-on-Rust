use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

pub fn reconstruct_image_dsp(projections: Vec<Vec<f32>>, image_size: usize) -> Vec<Vec<f32>> {
    let n_projections = projections.len();
    let n_samples = projections[0].len();
    let center = (image_size / 2) as f32;

    // 初始化重建图像
    let mut image = vec![vec![0.0; image_size]; image_size];

    // 创建FFT计划
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_samples);
    let ifft = planner.plan_fft_inverse(n_samples);

    // 构建Ram-Lak滤波器
    let mut filter = vec![0.0; n_samples];
    for i in 0..n_samples {
        let freq = if i < n_samples / 2 {
            i as f32
        } else {
            n_samples as f32 - i as f32
        };
        filter[i] = if freq == 0.0 { 0.0 } else { freq };
    }

    for (p, projection) in projections.iter().enumerate() {
        // 傅里叶变换
        let mut freq_data: Vec<Complex<f32>> =
            projection.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut freq_data);

        // 应用滤波器
        for (i, value) in freq_data.iter_mut().enumerate() {
            *value *= Complex::new(filter[i], 0.0);
        }

        // 逆傅里叶变换
        ifft.process(&mut freq_data);

        // 提取滤波后的投影
        let filtered_projection: Vec<f32> = freq_data.iter().map(|c| c.re).collect();

        // 将投影反投影到图像
        let angle = (p as f32 / n_projections as f32) * PI;
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        for x in 0..image_size {
            for y in 0..image_size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let r = dx * cos_angle + dy * sin_angle + (n_samples as f32 / 2.0);
                if r >= 0.0 && r < n_samples as f32 {
                    let r_idx = r.floor() as usize;
                    let value = if r_idx + 1 < n_samples {
                        let alpha = r - r_idx as f32;
                        (1.0 - alpha) * filtered_projection[r_idx]
                            + alpha * filtered_projection[r_idx + 1]
                    } else {
                        filtered_projection[r_idx]
                    };
                    image[y][x] += value;
                }
            }
        }
    }

    // 归一化图像
    let max_value = image
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(0.0, f32::max);
    if max_value > 0.0 {
        for row in &mut image {
            for pixel in row {
                *pixel /= max_value;
            }
        }
    }
    for i in 0..image_size {
        for j in 0..image_size {
            image[i][j] = (image[i][j].powf(1.37)).powf(1.27);
        }
    }
    image
}
