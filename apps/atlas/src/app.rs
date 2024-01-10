use egui::{Align, Key};
use serde::{Deserialize, Serialize};

use crate::theme::{set_theme, LATTE, MACCHIATO};

#[derive(Clone, Serialize, Deserialize)]
pub struct NapkinService {
    host: String,
    port: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NapkinSettings {
    model: String,
    service: NapkinService,
}

impl NapkinSettings {
    pub fn default() -> Self {
        Self {
            model: "mistral".to_owned(),
            service: NapkinService {
                host: "localhost".to_owned(),
                port: "11434".to_owned(),
            },
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AtlasApp {
    // Example stuff:
    label: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    theme: Theme,
    side_panel_open: bool,
    settings_window_open: bool,
    napkin_settings: NapkinSettings,
    napkin_temp_settings: NapkinSettings,
}

impl Default for AtlasApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            theme: Theme::Dark,
            side_panel_open: true,
            settings_window_open: false,
            napkin_settings: NapkinSettings::default(),
            napkin_temp_settings: NapkinSettings::default(),
        }
    }
}

impl AtlasApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn save_settings(&mut self) {
        self.napkin_settings = self.napkin_temp_settings.clone();
    }

    pub fn revert_settings(&mut self) {
        self.napkin_temp_settings = self.napkin_settings.clone();
    }
}

impl eframe::App for AtlasApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(
            &ctx,
            match self.theme {
                Theme::Light => LATTE,
                Theme::Dark => MACCHIATO,
            },
        );
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Settings").clicked() {
                            self.settings_window_open = true;
                        }
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    ui.horizontal(|ui| match self.theme {
                        Theme::Dark => {
                            if ui
                                .button("☀")
                                .on_hover_text("Switch to light mode")
                                .clicked()
                            {
                                self.theme = Theme::Light;
                            }
                        }
                        Theme::Light => {
                            if ui
                                .button("🌙")
                                .on_hover_text("Switch to dark mode")
                                .clicked()
                            {
                                self.theme = Theme::Dark;
                            }
                        }
                    });
                    ui.toggle_value(&mut self.side_panel_open, "File Browser");
                });
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .show_animated(ctx, self.side_panel_open, |ui| {
                ui.set_width(200.0);
                ui.with_layout(
                    egui::Layout::top_down(Align::Min).with_cross_align(Align::Min),
                    |ui| ui.heading("Side Panel"),
                );
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(":Z90:", |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Created by ");
                        ui.hyperlink_to("Z90 Studios", "https://github.com/Z90-Studios");
                        ui.label(".");
                    });
                });
                egui::warn_if_debug_build(ui);
            });
        });
        central_panel(ctx, self);
        settings_window(ctx, self);
    }
}

fn central_panel(ctx: &egui::Context, app: &mut AtlasApp) {
    egui::CentralPanel::default()
    .show(ctx, |ui| {
    // The central panel the region left after adding TopPanel's and SidePanel's
    // ui.heading("eframe template");

    // ui.horizontal(|ui| {
    //     ui.label("Write something: ");
    //     ui.text_edit_singleline(&mut self.label);
    // });

    // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
    // if ui.button("Increment").clicked() {
    //     self.value += 1.0;
    // }

    ui.heading("Project: Napkin Atlas");
    ui.label("So here's the plan:\n\nThe purpose of this application is to serve as the frontend to a locally run AI agent. This agent will do the following:\n\n1. Parse a codebase, or other information.\n2. Map the data into a network graph with vector database.\n3. Use the data in prompting along with multiple other elements to create a cohesive change to codebases.");

    ui.separator();

    ui.label(format!("Host: {}, Port: {}", app.napkin_settings.service.host, app.napkin_settings.service.port));

    // ui.add(egui::github_link_file!(
    //     "https://github.com/emilk/eframe_template/blob/master/",
    //     "Source code."
    // ));
});

    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::B)) {
        app.side_panel_open = !app.side_panel_open;
    }
}

fn settings_window(ctx: &egui::Context, app: &mut AtlasApp) {
    let mut should_close = false;
    let mut should_save = false;

    egui::Window::new("Settings")
        .open(&mut app.settings_window_open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("LLM Settings");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.text_edit_singleline(&mut app.napkin_temp_settings.model);
                ui.label("Model: ");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.text_edit_singleline(&mut app.napkin_temp_settings.service.host);
                ui.label("Host: ").rect.set_width(80.0);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.text_edit_singleline(&mut app.napkin_temp_settings.service.port);
                ui.label("Port: ").rect.set_width(80.0);
            });
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        should_save = true;
                        should_close = true;
                    }
                });
            })
        });

    if should_close {
        app.settings_window_open = false;

        if should_save {
            app.save_settings();
        } else {
            app.revert_settings();
        }
    }
}