use eframe::egui;
use rand::rngs::ThreadRng;

use xkcd_password_gen::config::ConfigBuilder;
use xkcd_password_gen::password_maker::PasswordMaker;
use xkcd_password_gen::types::PaddingType;
use xkcd_password_gen::types::StrEnum;
use xkcd_password_gen::types::WordTransformationType;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 700.0]),
        ..Default::default()
    };

    // Our application state:
    let config = ConfigBuilder::new().build().unwrap();
    // TODO force OsRng
    let mut password_maker = PasswordMaker::<ThreadRng>::new(config);
    let mut output: Vec<String> = password_maker.create_passwords();
    let mut padding_characters: String = password_maker.config.padding_characters.iter().collect();
    let mut separator_characters: String =
        password_maker.config.separator_characters.iter().collect();
    //let mut dirty = false;

    eframe::run_simple_native("xkcd-password-gen", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.spacing.item_spacing = egui::vec2(17.0, 17.0);
            });
            egui::ScrollArea::both().show(ui, |ui| {
                egui::CollapsingHeader::new("words")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Slider::new(&mut password_maker.config.word_count, 0..=32)
                                .text("count")
                                .logarithmic(true),
                        );
                        ui.add(
                            egui::Slider::new(&mut password_maker.config.word_min_length, 3..=9)
                                .text("min length")
                                .logarithmic(true),
                        );
                        ui.add(
                            egui::Slider::new(
                                &mut password_maker.config.word_max_length,
                                password_maker.config.word_min_length..=9,
                            )
                            .text("max length")
                            .logarithmic(true),
                        );
                        egui::ComboBox::from_label("transformation")
                            .selected_text(
                                password_maker.config.word_transformation.to_static_str(),
                            )
                            .show_ui(ui, |ui| {
                                for (description, item) in WordTransformationType::NAME_MEMBER_ARR {
                                    ui.selectable_value(
                                        &mut password_maker.config.word_transformation,
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
                            egui::Slider::new(&mut password_maker.config.digits_before, 0..=255)
                                .text("before")
                                .logarithmic(true),
                        );
                        ui.add(
                            egui::Slider::new(&mut password_maker.config.digits_after, 0..=255)
                                .text("after")
                                .logarithmic(true),
                        );
                    });
                egui::CollapsingHeader::new("padding")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let name_label = ui.label("possible choices");
                            ui.text_edit_singleline(&mut padding_characters)
                                .labelled_by(name_label.id);
                        });
                        password_maker.config.padding_characters =
                            padding_characters.chars().collect();
                        egui::ComboBox::from_label("type")
                            .selected_text(password_maker.config.padding_type.to_static_str())
                            .show_ui(ui, |ui| {
                                for (description, item) in PaddingType::NAME_MEMBER_ARR {
                                    ui.selectable_value(
                                        &mut password_maker.config.padding_type,
                                        *item,
                                        *description,
                                    );
                                }
                            });
                        ui.add(
                            egui::Slider::new(&mut password_maker.config.padding_length, 0..=255)
                                .text("length")
                                .logarithmic(true),
                        );
                    });
                egui::CollapsingHeader::new("separator")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let name_label = ui.label("possible choices");
                            ui.text_edit_singleline(&mut separator_characters)
                                .labelled_by(name_label.id);
                        });
                        password_maker.config.separator_characters =
                            separator_characters.chars().collect();
                    });
                ui.add(
                    egui::Slider::new(&mut password_maker.config.count, 1..=255)
                        .text("how many to generate")
                        .logarithmic(true),
                );

                if ui.button("generate").clicked() {
                    output = password_maker.create_passwords();
                }

                for item in output.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("📋").clicked() {
                            ctx.copy_text(item.clone());
                        }
                        ui.label(item);
                    });
                }
                ui.allocate_space(ui.available_size());
            });
        });
    })
}
