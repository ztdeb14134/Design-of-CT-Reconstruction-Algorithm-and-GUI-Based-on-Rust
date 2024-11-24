pub fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily;

    // 获取默认字体定义
    let mut fonts = FontDefinitions::default();

    // 添加中文字体（例如 `simhei.ttf` 或其他支持中文的字体）
    fonts.font_data.insert(
        "simhei".to_owned(),                              // 字体的标识名称
        FontData::from_static(include_bytes!("gbk.ttf")), // 替换为你的中文字体路径
    );

    // 将新字体加入到 `proportional` 和 `monospace` 中
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "simhei".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("simhei".to_owned());

    // 应用字体定义
    ctx.set_fonts(fonts);
}
