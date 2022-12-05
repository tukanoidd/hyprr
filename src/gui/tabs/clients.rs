use std::collections::HashMap;

use hyprland::{
    data::{Client, Clients},
    shared::WorkspaceType,
};
use iced::{
    widget::{button, text, Column, Scrollable, Space},
    Alignment, Command, Element, Length,
};

use crate::gui::tabs::TabsMsg;
use crate::gui::{app::GuiAppMsg, dropdown_button::DropdownButton, wrapper_functions::get_clients};

#[derive(Debug, Clone)]
pub enum ClientsTabMsg {
    Refresh,
    Refreshed(Clients),
    ToggleClient(String),
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

    pub fn view<'a>(&self) -> Element<'a, GuiAppMsg> {
        Scrollable::new(
            self.clients.iter().fold(
                Column::new()
                    .push(
                        button(text("Refresh"))
                            .height(Length::Units(30))
                            .width(Length::Units(80))
                            .on_press(GuiAppMsg::Tabs(TabsMsg::Clients(ClientsTabMsg::Refresh))),
                    )
                    .push(Space::with_height(Length::Units(20)))
                    .align_items(Alignment::Center)
                    .spacing(10),
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
                                GuiAppMsg::Tabs(TabsMsg::Clients(ClientsTabMsg::ToggleClient(
                                    title.clone(),
                                ))),
                            ),
                    )
                },
            ),
        )
        .into()

        /*ScrollableList::with_items(
            self.clients
                .iter()
                .map(
                    |Client {
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
                        vec![
                            title.clone(),
                            format!("    Address {address}"),
                            format!("    At: {}x{}", at.0, at.1),
                            format!("    Size: {}x{}", size.0, size.1),
                            format!(
                                "    Workspace: {}",
                                match workspace.id {
                                    WorkspaceType::Regular(id) => {
                                        format!("Regular (id: {}) \"{}\"", id, workspace.name)
                                    }
                                    WorkspaceType::Special => {
                                        format!("Special \"{}\"", workspace.name)
                                    }
                                }
                            ),
                            format!("    Floating: {floating}",),
                            format!("    Monitor: {monitor}"),
                            format!("    Class: {class}"),
                            format!("    Pid: {pid}"),
                            format!("    XWayland: {xwayland}"),
                        ]
                    },
                )
                .collect(),
        )
        .on_refresh(|| GuiAppMsg::RefreshClients)
        .view()*/
    }

    pub fn update(&mut self, msg: ClientsTabMsg) -> Command<GuiAppMsg> {
        match msg {
            ClientsTabMsg::Refresh => {
                return Command::perform(get_clients(), |clients| {
                    GuiAppMsg::Tabs(TabsMsg::Clients(ClientsTabMsg::Refreshed(clients)))
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
