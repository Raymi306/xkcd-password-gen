// provides:
// static WORDLIST: &[&str] = &[...]
include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

use eframe::egui;
use egui::RichText;
use egui::Color32;
use rand::Rng;
use rand::SeedableRng;
use rand::TryRngCore;
use rand::rngs::OsRng;
use rand::rngs::SmallRng;

use xkcd_password_gen::config::Config;
use xkcd_password_gen::config::ConfigBuilder;
use xkcd_password_gen::password_maker::PasswordMaker;
use xkcd_password_gen::types::PaddingType;
use xkcd_password_gen::types::StrEnum;
use xkcd_password_gen::types::WordTransformationType;

struct App {
    config_prev: Config,
    config_curr: Config,
    password_maker: PasswordMaker<OsRng>,
    preview_maker: PasswordMaker<SmallRng>,
    preview: String,
    generated_passwords: Vec<String>,
    padding_characters: String,
    separator_characters: String,
    seed_string: String,
    curr_seed: u64,
    prev_seed: u64,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(17.0, 17.0);
            style.interaction.selectable_labels = false;
        });
        let config_prev = ConfigBuilder::new().build().unwrap();
        let config_curr = config_prev.clone();
        // TODO take references to a config?
        let password_maker = PasswordMaker::<OsRng>::new(config_curr.clone());
        let seed_string = "13414357264162109690".to_owned();
        let curr_seed = 13414357264162109690;
        let prev_seed = 13414357264162109690;
        let mut preview_maker: PasswordMaker<SmallRng> = PasswordMaker {
            rng: SmallRng::seed_from_u64(curr_seed).unwrap_err(),
            config: config_curr.clone(),
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        };
        let preview: String = preview_maker.make_password();
        let generated_passwords: Vec<String> = Vec::new();
        let padding_characters: String = config_curr.padding_characters.iter().collect();
        let separator_characters: String = config_curr.separator_characters.iter().collect();
        Self {
            config_prev,
            config_curr,
            password_maker,
            seed_string,
            curr_seed,
            prev_seed,
            preview_maker,
            preview,
            generated_passwords,
            padding_characters,
            separator_characters,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                egui::CollapsingHeader::new(RichText::new("PREVIEW").color(Color32::ORANGE))
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let response = ui.add_sized(
                                [160.0, 17.0],
                                egui::TextEdit::singleline(&mut self.seed_string),
                            );
                            if response.changed() {
                                if let Ok(seed) = self.seed_string.parse::<u64>() {
                                    self.curr_seed = seed;
                                } else {
                                    self.seed_string = self.curr_seed.to_string();
                                }
                            }
                            if ui.button("random seed").clicked() {
                                self.curr_seed = self.password_maker.rng.random();
                                self.seed_string = self.curr_seed.to_string();
                            }
                        });
                        if self.config_curr != self.config_prev || self.curr_seed != self.prev_seed {
                            self.preview_maker.rng = SmallRng::seed_from_u64(self.curr_seed).unwrap_err();
                            self.preview_maker.config = self.config_curr.clone();
                            self.preview = self.preview_maker.make_password();
                            self.config_prev = self.config_curr.clone();
                            self.prev_seed = self.curr_seed;
                        }
                        ui.label(&self.preview);
                    });
                egui::CollapsingHeader::new("words")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Slider::new(&mut self.config_curr.word_count, 0..=32)
                                .text("count")
                                .logarithmic(true),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.config_curr.word_min_length, 3..=9)
                                .text("min length"),
                        );
                        ui.add(
                            egui::Slider::new(
                                &mut self.config_curr.word_max_length,
                                self.config_curr.word_min_length..=9,
                            )
                            .text("max length"),
                        );
                        egui::ComboBox::from_label("transform")
                            .selected_text(self.config_curr.word_transformation.to_static_str())
                            .show_ui(ui, |ui| {
                                for (description, item) in WordTransformationType::NAME_MEMBER_ARR {
                                    ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 3.0);
                                    ui.selectable_value(
                                        &mut self.config_curr.word_transformation,
                                        *item,
                                        *description,
                                    );
                                }
                            });
                    });
                egui::CollapsingHeader::new("digits")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Slider::new(&mut self.config_curr.digits_before, 0..=255)
                                .text("before")
                                .logarithmic(true),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.config_curr.digits_after, 0..=255)
                                .text("after")
                                .logarithmic(true),
                        );
                    });
                egui::CollapsingHeader::new("padding")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let name_label = ui.label("possible choices");
                            ui.text_edit_singleline(&mut self.padding_characters)
                                .labelled_by(name_label.id);
                        });
                        self.config_curr.padding_characters =
                            self.padding_characters.chars().collect();
                        egui::ComboBox::from_label("type")
                            .selected_text(self.config_curr.padding_type.to_static_str())
                            .show_ui(ui, |ui| {
                                for (description, item) in PaddingType::NAME_MEMBER_ARR {
                                    ui.style_mut().spacing.item_spacing = egui::vec2(3.0, 3.0);
                                    ui.selectable_value(
                                        &mut self.config_curr.padding_type,
                                        *item,
                                        *description,
                                    );
                                }
                            });
                        ui.add(
                            egui::Slider::new(&mut self.config_curr.padding_length, 0..=255)
                                .text("length")
                                .logarithmic(true),
                        );
                    });
                egui::CollapsingHeader::new("separator")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let name_label = ui.label("possible choices");
                            ui.text_edit_singleline(&mut self.separator_characters)
                                .labelled_by(name_label.id);
                        });
                        self.config_curr.separator_characters =
                            self.separator_characters.chars().collect();
                    });
                ui.add(
                    egui::Slider::new(&mut self.config_curr.count, 1..=255)
                        .text("how many to generate")
                        .logarithmic(true),
                );

                if ui
                    .add_sized(
                        [384.0, 17.0],
                        egui::Button::new("generate").fill(Color32::DARK_GREEN),
                    )
                    .clicked()
                {
                    self.password_maker.config = self.config_curr.clone();
                    self.generated_passwords = self.password_maker.make_passwords();
                }

                for item in self.generated_passwords.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("📋").clicked() {
                            ctx.copy_text(item.clone());
                        }
                        ui.add(egui::Label::new(item).selectable(true));
                    });
                }
                ui.allocate_space(ui.available_size());
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([400.0, 740.0])
            .with_inner_size([400.0, 740.0]),
        ..Default::default()
    };
    eframe::run_native(
        "fmn-passgen",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
