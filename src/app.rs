// use crate::trade::order;

use reqwest::blocking;
use reqwest::header::{self, HeaderName, HeaderValue};

use eframe::{
    egui::{self, RichText},
    epaint::{Color32, FontId},
};
use egui::TextStyle;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct StockTradeApp {
    // Example stuff:
    label: String,
    response: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for StockTradeApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Stock Tradi ng".to_owned(),
            response: "Empty".to_owned(),
            value: 2.7,
        }
    }
}

impl StockTradeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for StockTradeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            label,
            response,
            value,
        } = self;
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
                if ui.button("Button1").clicked() {
                    println!("Button1 clicked")
                }
                ui.separator();
                if ui.button("Button2").clicked() {
                    println!("Button2 clicked")
                }
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));

            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Stock Trading with Quant method");
            ui.label(
                RichText::new("Qunant Rest API Test")
                    .font(FontId::proportional(40.0))
                    .color(Color32::DARK_RED),
            );
            let mut temp = String::from("TEST");
            if ui.button("Button Test").clicked() {
                println!("Button clicked");

                let client = reqwest::blocking::Client::new();
                let res: blocking::Response;

                res = client
                    .get("http://127.0.0.1:8080/stocks/list")
                    .send()
                    .unwrap();

                self.response = match res.status() {
                    reqwest::StatusCode::OK => {
                        // println!("Response Headers:\n{:#?}", res.headers());
                        let v: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
                        println!("{:?}", v);
                        v.to_string()
                    }
                    _ => "no Data available".to_owned(),
                };
            }

            ui.label(
                RichText::new(&self.response)
                    .text_style(TextStyle::Monospace)
                    .color(Color32::DARK_BLUE),
            );

            ui.label(
                RichText::new("‼ Debug build ‼")
                    .text_style(TextStyle::Monospace)
                    .color(Color32::RED),
            )
            .on_hover_text("egui was compiled with debug assertions enabled.");
            // egui::warn_if_debug_build(ui);
        });

        if true {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        // order::hello();
    }
}
