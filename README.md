# SZTU-HESS 医学成像原理 -- 2024 秋 -- 期末课程设计

## 选题: CT 图像重建算法与图形用户界面的设计

### 项目简介
通过 CT 的原始数据切片，主动生成 CT 中某一切片的投影数据。通过设计多种算法实现图像的重建，最终以图形用户界面 (GUI) 进行展示，并对比不同算法的优劣。

### 项目目标
1. **数据生成**: 解析 CT 原始切片数据，生成对应的投影数据。
2. **算法实现**: 设计并实现多种 CT 图像重建算法。
   - 算法对比包括计算效率、重建精度等。
3. **界面设计**: 开发一个图形用户界面 (GUI)，展示重建结果并提供交互功能。

---

### 项目组成员

| 姓名         | 学号          | 角色       |
|--------------|---------------|------------|
| **蔡瑞风**   | 202200502029  | 组长       |
| **潘柏伟**   | 202200502123  | 组员       |
| **钟扬东**   | 202200502006  | 组员       |
| **黄舒超**   | 202200502031  | 组员       |
| **周子驭**   | 202200502036  | 组员       |
| **李唐龙**   | 202201102040  | 组员       |

---

### 项目地址
[GitHub 项目仓库](https://github.com/ztdeb14134/Design-of-CT-Reconstruction-Algorithm-and-GUI-Based-on-Rust)

---

### 实现计划

#### 1. 数据准备
- 从 CT 原始数据提取切片并生成投影数据。
- 编写数据生成工具，支持不同分辨率及参数设置。

#### 2. 算法设计与实现
- 实现以下重建算法（可根据需要扩展）：
  - 直接反投影算法 
  - 代数重建反投影算法
  - 滤波反投影算法
- 优化算法性能，记录运行时间及结果精度。

#### 3. 图形用户界面 (GUI) 设计
- 使用常见的 rust-egui 开发框架。
- 功能包括：
  - 投影数据与重建结果的可视化。
  - 算法选择及参数设置。
  - 重建结果对比。

#### 4. 测试与优化
- 对不同分辨率和数据量的测试。
- 对比各算法在不同条件下的优劣，撰写报告。

---


### 项目成果
1. **功能性 GUI**:
   - 支持用户选择算法并展示对比结果。
2. **算法实现**:
   - 至少两种有效的 CT 图像重建算法。
3. **完整报告**:
   - 包括算法分析、性能对比及实验结果。

#### 环境要求
- **编程语言**: Rust (最新稳定版)  
- **构建工具**: Cargo  
- **依赖库**: 详见Cargo.toml
- **操作系统**: 支持 Windows、MacOS 和 Linux。

#### 编译、运行项目

1. **安装 Rust 环境**  
   确保已安装 [Rust](https://www.rust-lang.org/) 和 `cargo`。

2. **克隆项目仓库**
   使用下列命令克隆仓库
   ```bash
   git clone https://github.com/ztdeb14134/Design-of-CT-Reconstruction-Algorithm-and-GUI-Based-on-Rust.git
   cd Design-of-CT-Reconstruction-Algorithm-and-GUI-Based-on-Rust

3. **添加依赖**
   Rust 项目使用 Cargo 进行依赖管理，依赖会在构建时自动安装。如果需要手动添加依赖，请编辑 Cargo.toml 文件，添加以下内容：
   ```toml
   [dependencies]
   egui = "*"
   eframe = "*"
   image = "*"
   nalgebra = "*"
   nshare = "*"
   rand = "*"
   fundsp = "*"
   realfft = "*"
   rustfft = "*"
   egui_extras = "*"
   ndarray = "*"

4. **编译项目**
   在Cargo.toml所在路径终端中执行以下命令:

   ```bash
   cargo build --release
   ```
   构建完成后，生成的可执行文件位于：
   ```arduino
   target/release/

5. **运行项目**
   -可以选择用已经编译好的exe文件或者自己编译的exe文件启动(./target/release/ct_rebuild_project.exe)
   -可以通过在Cargo.toml所在路径终端中执行以下命令:
   ```bash
   cargo run --release
   ```
   !!!运行注意事项!!!
   如果使用相对路径,请务必在项目文件夹中通过终端打开并输入./target/release/ct_rebuild_project.exe
   如果相对路径运行不成功时请使用绝对路径尝试


