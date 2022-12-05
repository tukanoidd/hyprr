use std::collections::HashMap;

use hyprland::{
    data::{Client, Clients},
    shared::WorkspaceType,
};
use iced::{
    widget::{button, text, Column, Scrollable, Space},
    Alignment, Command, Element, Length,
};

use crate::gui::{
    app, dropdown_button::DropdownButton, tabs::TabsMsg, wrapper_functions::get_clients,
};

#[derive(Debug, Clone)]
pub enum ClientsTabMsg {
    Refresh,
    Refreshed(Clients),
    ToggleClient(String),
}

impl From<ClientsTabMsg> for app::GuiAppMsg {
    fn from(value: ClientsTabMsg) -> Self {
        app::GuiAppMsg::Tabs(TabsMsg::Clients(value))
    }
}

pub struct ClientsTab {
    clients: Clients,

    dropdowns_open: HashMap<String, bool>,
}

impl ClientsTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            clients: vec![],
            dropdowns_open: HashMap::new(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    pub fn view(&self) -> Element<app::GuiAppMsg> {
        let refresh_button = button(text("Refresh"))
            .height(Length::Units(30))
            .width(Length::Units(80))
            .on_press(ClientsTabMsg::Refresh.into());

        let clients = self.clients.iter().fold(
            Column::new()
                .push(refresh_button)
                .push(Space::with_height(Length::Units(20)))
                .align_items(Alignment::Center)
                .spacing(10),
            |col: Column<app::GuiAppMsg>,
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
                            ClientsTabMsg::ToggleClient(title.clone()).into(),
                        ),
                )
            },
        );

        Scrollable::new(clients).into()
    }

    pub fn update(&mut self, msg: ClientsTabMsg) -> Command<app::GuiAppMsg> {
        match msg {
            ClientsTabMsg::Refresh => {
                return Command::perform(get_clients(), |clients| {
                    ClientsTabMsg::Refreshed(clients).into()
                })
            }
            ClientsTabMsg::Refreshed(clients) => {
                self.clients = clients;

                self.clients
                    .iter()
                    .for_each(|Client { title, .. }: &Client| {
                        if !self.dropdowns_open.contains_key(title) {
                            self.dropdowns_open.insert(title.clone(), false);
                        }
                    })
            }
            ClientsTabMsg::ToggleClient(title) => {
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
}
