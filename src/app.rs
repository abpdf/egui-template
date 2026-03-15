use rand::Rng;
use egui::RichText;
use egui::{FontDefinitions, FontData, FontFamily};
const CUSTOM_FONT_DATA: &[u8] = include_bytes!("../assets/SourceHanSansSC-Light.otf");

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 将嵌入的字体数据插入到字体定义中，并命名为 "MyFont"
    fonts.font_data.insert(
        "MyFont".to_owned(),
        FontData::from_static(CUSTOM_FONT_DATA), // 使用 'static 字节切片
    );

    // 设置比例字体族（用于大多数文本）为刚刚添加的字体
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "MyFont".to_owned()); // 插入到最前面，优先使用

    // 如果需要等宽字体也使用同一字体，可类似设置 FontFamily::Monospace

    ctx.set_fonts(fonts);
}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_string(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(
                RichText::new(format!("{}", self.label))
                    .size(72.0)   // 字号设置为 72 像素
            );

            if ui.button(RichText::new("一一一").size(72.0)).clicked() {
                self.label = format!("{}", rand::thread_rng().gen_range(1..=50));
            }

        });
    }
}
