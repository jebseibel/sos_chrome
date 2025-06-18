use eframe::egui::{self, ColorImage, TextureHandle};
use crate::load_config::Config;
use crate::features::profile_utils::ProfileMap;
use crate::features;

pub struct MyApp {
    pub logo: Option<TextureHandle>,
    pub config: Config,
    pub profile_map: ProfileMap,
}

impl eframe::App for crate::MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_visuals(egui::Visuals::light());

        ctx.style_mut(|style| {
            style.visuals.window_fill = egui::Color32::from_rgb(200, 200, 200);
        });

        ctx.style_mut(|style| {
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(18.0), // 18-point font for buttons
            );
        });

        // Lazy load logo only once
        if self.logo.is_none() {
            if let Ok(image) = load_image_from_path(&self.config.logo.name) {
                self.logo = Some(ctx.load_texture("logo", image, Default::default()));
            }
        }

        // Top bar with logo on the left and title centered
        // Top bar with logo on the left and title centered
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let app_title = &self.config.app_title;
            let title_width = self.config.title_width;
            let logo_width = self.config.logo.width as f32;
            let logo_height = self.config.logo.height as f32;

            ui.set_height(40.0);

            ui.horizontal(|ui| {
                let available_width = ui.available_width();
                let spacer_width = (available_width - logo_width - title_width).max(0.0) / 2.0;

                // Left: Logo
                if let Some(logo) = &self.logo {
                    ui.add(
                        egui::Image::new(logo)
                            .fit_to_exact_size(egui::Vec2::new(logo_width, logo_height)),
                    );
                }

                // Spacer to center title
                ui.add_space(spacer_width);

                // Center: Title
                ui.label(
                    egui::RichText::new(app_title)
                        .heading()
                        .strong(),
                );
            });
        });


        // Left panel with vertical buttons
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add_space(20.0); // Padding at the top

            let button_1_label = &self.config.button_labels.button_1;
            if ui.add(egui::Button::new(button_1_label).min_size(egui::Vec2::new(200.0, 40.0))).clicked() {
                println!("{} clicked", button_1_label);
                features::handle_extensions::run(&self.profile_map);
            }

            ui.add_space(20.0);

            if ui.add(egui::Button::new(&self.config.button_labels.button_2).min_size(egui::Vec2::new(200.0, 40.0))).clicked() {
                println!("{} clicked", self.config.button_labels.button_2);
                features::remove_hijackers::run();
            }

            ui.add_space(20.0);

            if ui.add(egui::Button::new(&self.config.button_labels.button_3).min_size(egui::Vec2::new(200.0, 40.0))).clicked() {
                println!("{} clicked", self.config.button_labels.button_3);
                features::block_notifications::run();
            }

            ui.add_space(20.0);

            if ui.add(egui::Button::new(&self.config.button_labels.button_4).min_size(egui::Vec2::new(200.0, 40.0))).clicked() {
                println!("{} clicked", self.config.button_labels.button_4);
                features::flush_history::run();
            }
        });
        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Display Area");
        });
    }
}

fn load_image_from_path(path: &str) -> Result<ColorImage, String> {

    let image = image::open(path).map_err(|e| e.to_string())?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels: Vec<_> = image_buffer.pixels().flat_map(|p| p.0).collect();

    Ok(ColorImage::from_rgba_unmultiplied(size, &pixels))
}