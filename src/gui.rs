////in main:     // let mut filepath = gui::start_gui();

// use egui::{Color32, Label, Sense, Ui, Window};
// use eframe::{egui};
//
// pub fn start_gui() {
//
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(320.0, 240.0)),
//         ..Default::default()
//     };
//
//     // Our application state:
//     let mut filepath = "".to_owned();
//     let mut age = 42;
//
//     eframe::run_simple_native("Imsearch", options,  |ctx, _frame| {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Imsearch RustTiger");
//             ui.horizontal(|ui| {
//                 let filepath_label = ui.label("put filepath 'xxx.png' here: ");
//                 ui.text_edit_singleline(&mut filepath)
//                     .labelled_by(filepath_label.id);
//             });
//
//             if ui.button("OK").clicked() {
//                 //todo()!
//             }
//             ui.label(format!("\nThe specified filepath is: '{filepath}'"));
//
//         });
//     }).expect("starting GUI failed");
// }