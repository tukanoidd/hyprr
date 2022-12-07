use std::collections::HashMap;

use hyprland::{
    data::{Monitor, Monitors, Transforms},
    shared::WorkspaceType,
};
use iced::{
    widget::{text, Column},
    Command,
};

use crate::gui::{
    app,
    dropdown_button::DropdownButton,
    tabs::{templates::TabTemplate, TabsMsg},
    wrapper_functions::get_monitors,
};

#[derive(Debug, Clone)]
pub enum MonitorsTabMsg {
    Refresh,
    Refreshed(Monitors),
    ToggleMonitor(String),
}

impl From<MonitorsTabMsg> for app::GuiAppMsg {
    fn from(value: MonitorsTabMsg) -> Self {
        app::GuiAppMsg::Tabs(TabsMsg::Monitors(value))
    }
}

pub struct MonitorsTab {
    monitors: Monitors,
    dropdowns_open: HashMap<String, bool>,
}

impl MonitorsTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            monitors: vec![],
            dropdowns_open: Default::default(),
        }
    }
}

impl TabTemplate for MonitorsTab {
    type Message = MonitorsTabMsg;
    type AppMessage = app::GuiAppMsg;

    fn update(&mut self, msg: Self::Message) -> Command<Self::AppMessage> {
        match msg {
            MonitorsTabMsg::Refresh => {
                return Command::perform(get_monitors(), |monitors| {
                    MonitorsTabMsg::Refreshed(monitors).into()
                });
            }
            MonitorsTabMsg::Refreshed(monitors) => {
                self.monitors = monitors;

                self.monitors
                    .iter()
                    .for_each(|Monitor { id, name, .. }: &Monitor| {
                        let title = format!("Monitor {id} ({name})");

                        if !self.dropdowns_open.contains_key(&title) {
                            self.dropdowns_open.insert(title.clone(), false);
                        }
                    })
            }
            MonitorsTabMsg::ToggleMonitor(title) => {
                self.dropdowns_open
                    .entry(title)
                    .and_modify(|val: &mut bool| {
                        *val = !*val;
                    })
                    .or_insert(false);
            }
        }

        Command::none()
    }

    fn add_info_to_list<'a>(
        &'a self,
        list: Column<'a, Self::AppMessage>,
    ) -> Column<'a, Self::AppMessage> {
        self.monitors.iter().fold(
            list,
            |col,
             Monitor {
                 id,
                 name,
                 width,
                 height,
                 refresh_rate,
                 x,
                 y,
                 active_workspace,
                 reserved,
                 scale,
                 transform,
                 focused,
             }: &Monitor| {
                let title = format!("Monitor {id} ({name})");

                col.push(
                    DropdownButton::new(&title)
                        .add_child(text(format!("Size: {width}x{height}")))
                        .add_child(text(format!("Refresh Rate: {refresh_rate}Hz")))
                        .add_child(text(format!("Position: {x}x{y}")))
                        .add_child(text(format!(
                            "Active Workspace: {}",
                            match active_workspace.id {
                                WorkspaceType::Regular(id) => {
                                    format!("Regular (id: {}) \"{}\"", id, name)
                                }
                                WorkspaceType::Special => {
                                    format!("Special \"{}\"", active_workspace.name)
                                }
                            }
                        )))
                        .add_child(text(format!(
                            "Reserved: ({}, {}, {}, {})",
                            reserved.0, reserved.1, reserved.2, reserved.3
                        )))
                        .add_child(text(format!("    Scale: {scale}")))
                        .add_child(text(format!(
                            "Transform: {}",
                            match transform {
                                Transforms::Normal => {
                                    "Normal"
                                }
                                Transforms::Normal90 => {
                                    "Normal + 90"
                                }
                                Transforms::Normal180 => {
                                    "Normal + 180"
                                }
                                Transforms::Normal270 => {
                                    "Normal + 270"
                                }
                                Transforms::Flipped => {
                                    "Flipped"
                                }
                                Transforms::Flipped90 => {
                                    "Flipped + 90"
                                }
                                Transforms::Flipped180 => {
                                    "Flipped + 180"
                                }
                                Transforms::Flipped270 => {
                                    "Flipped + 270"
                                }
                            }
                        )))
                        .add_child(text(format!("Focused: {}", focused)))
                        .view(
                            self.dropdowns_open.get(&title).copied().unwrap_or(false),
                            MonitorsTabMsg::ToggleMonitor(title.clone()).into(),
                        ),
                )
            },
        )
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.monitors.is_empty()
    }
}
