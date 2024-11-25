pub fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily;
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "simhei".to_owned(),                              // 字体的标识名称
        FontData::from_static(include_bytes!("gbk.ttf")), // 替换为你的中文字体路径
    );

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

    ctx.set_fonts(fonts);
}
