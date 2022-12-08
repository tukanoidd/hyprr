use std::collections::HashMap;

use hyprland::{
    data::{Client, Clients},
    shared::WorkspaceType,
};
use iced::widget::{text, Column};

use crate::gui::tabs::TabsMsg;
use crate::{
    gui::{
        app::GuiAppMsg,
        dropdown_button::DropdownButton,
        tabs::{
            templates::{
                RefreshableTabData, RefreshableTabDataUnit, RefreshableTabMsg,
                RefreshableTabTemplate,
            },
            GuiAppTab,
        },
        wrapper_functions::get_clients,
    },
    refreshable_tab_impl,
};

impl RefreshableTabDataUnit for Client {
    #[inline]
    fn title(&self) -> String {
        self.title.clone()
    }
}

impl RefreshableTabData for Clients {
    #[inline]
    fn titles(&self) -> Vec<String> {
        self.iter().map(RefreshableTabDataUnit::title).collect()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[derive(Debug)]
pub struct ClientsTab {
    clients: Box<Clients>,

    dropdowns_open: HashMap<String, bool>,
}

impl ClientsTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            clients: box vec![],
            dropdowns_open: HashMap::new(),
        }
    }
}

impl RefreshableTabTemplate for ClientsTab {
    fn add_info_to_list<'a>(&'a self, list: Column<'a, GuiAppMsg>) -> Column<'a, GuiAppMsg> {
        self.clients.iter().fold(
            list,
            |col: Column<GuiAppMsg>,
             Client {
                 address,
                 at,
                 size,
                 workspace,
                 floating,
                 monitor,
                 class,
                 title,
                 pid,
                 xwayland,
             }: &Client| {
                col.push(
                    DropdownButton::new(title)
                        .add_child(text(format!("Address {address}")))
                        .add_child(text(format!("At: {}x{}", at.0, at.1)))
                        .add_child(text(format!("Size: {}x{}", size.0, size.1)))
                        .add_child(text(format!(
                            "Workspace: {}",
                            match workspace.id {
                                WorkspaceType::Regular(id) => {
                                    format!("Regular (id: {}) \"{}\"", id, workspace.name)
                                }
                                WorkspaceType::Special => {
                                    format!("Special \"{}\"", workspace.name)
                                }
                            }
                        )))
                        .add_child(text(format!("Floating: {floating}")))
                        .add_child(text(format!("Monitor: {monitor}")))
                        .add_child(text(format!("Class: {class}")))
                        .add_child(text(format!("Pid: {pid}")))
                        .add_child(text(format!("XWayland: {xwayland}")))
                        .view(
                            self.dropdowns_open.get(title).copied().unwrap_or(false),
                            GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                                self.app_tab(),
                                RefreshableTabMsg::ToggleDropdown(title.clone()),
                            )),
                        ),
                )
            },
        )
    }

    refreshable_tab_impl!(data: clients<Clients> [query: get_clients]);
    refreshable_tab_impl!(dropdowns);

    #[inline]
    fn app_tab(&self) -> GuiAppTab {
        GuiAppTab::Clients
    }
}
