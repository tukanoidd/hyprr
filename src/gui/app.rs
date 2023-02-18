use crate::gui::tabs::AppTab;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct App {
    selected_tabs: Vec<AppTab>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_tabs: vec![AppTab::General],
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                self.selected_tabs = enum_iterator::all()
                    .filter(|tab| {
                        let mut selected = self.selected_tabs.contains(tab);
                        tab.selectable_label(ui, &mut selected);

                        selected
                    })
                    .collect();
            })
        });
    }

    fn main_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.selected_tabs.iter().for_each(|tab| {
                tab.window(ui);
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.header(ctx);

        self.main_panel(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
