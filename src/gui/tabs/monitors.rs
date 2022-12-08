use std::collections::HashMap;

use hyprland::{
    data::{Monitor, Monitors, Transforms},
    shared::WorkspaceType,
};
use iced::widget::{text, Column};

use crate::{
    gui::{
        app::GuiAppMsg,
        dropdown_button::DropdownButton,
        tabs::{
            templates::{
                RefreshableTabData, RefreshableTabDataUnit, RefreshableTabMsg,
                RefreshableTabTemplate,
            },
            GuiAppTab, TabsMsg,
        },
        wrapper_functions::get_monitors,
    },
    refreshable_tab_impl,
};

impl RefreshableTabDataUnit for Monitor {
    fn title(&self) -> String {
        format!("Monitor {} ({})", self.id, self.name)
    }
}

impl RefreshableTabData for Monitors {
    fn titles(&self) -> Vec<String> {
        self.iter().map(RefreshableTabDataUnit::title).collect()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[derive(Debug)]
pub struct MonitorsTab {
    monitors: Box<Monitors>,
    dropdowns_open: HashMap<String, bool>,
}

impl MonitorsTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            monitors: box vec![],
            dropdowns_open: Default::default(),
        }
    }
}

impl RefreshableTabTemplate for MonitorsTab {
    fn add_info_to_list<'a>(&'a self, list: Column<'a, GuiAppMsg>) -> Column<'a, GuiAppMsg> {
        self.monitors.iter().fold(list, |col, monitor: &Monitor| {
            let title = monitor.title();

            log::info!("Hallo: {}", &title);

            let Monitor {
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
                ..
            } = monitor;

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
                        GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                            GuiAppTab::Monitors,
                            RefreshableTabMsg::ToggleDropdown(title.clone()),
                        )),
                    ),
            )
        })
    }

    refreshable_tab_impl!(data: monitors<Monitors> [query: get_monitors]);
    refreshable_tab_impl!(dropdowns);

    fn app_tab(&self) -> GuiAppTab {
        GuiAppTab::Monitors
    }
}
