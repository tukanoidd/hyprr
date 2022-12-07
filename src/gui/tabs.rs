pub mod clients;
mod devices;
mod templates;

use hyprland::{
    data::{
        LayerClient, LayerDisplay, Layers, Monitor, Monitors, Transforms, Workspace, Workspaces,
    },
    shared::WorkspaceType,
};
use iced::{widget::Column, Alignment, Command, Element};
use iced_aw::{TabBar, TabLabel};

use crate::gui::tabs::templates::TabTemplate;
use crate::gui::{
    app,
    scrollable_list::ScrollableList,
    tabs::{
        clients::{ClientsTab, ClientsTabMsg},
        devices::{DevicesTab, DevicesTabMsg},
    },
    wrapper_functions::*,
};

#[derive(Debug, Clone)]
pub enum TabsMsg {
    TabChanged(GuiAppTab),

    Clients(ClientsTabMsg),
    Devices(DevicesTabMsg),

    /*Monitors(MonitorsTabMsg),
    Layers(LayersTabMsg),
    Workspaces(WorkspacesTabMsg),*/
    RefreshMonitors,
    MonitorsRefreshed(Monitors),

    RefreshLayers,
    LayersRefreshed(Layers),

    RefreshWorkspaces,
    WorkspacesRefreshed(Workspaces),
}

impl From<TabsMsg> for app::GuiAppMsg {
    #[inline]
    fn from(value: TabsMsg) -> Self {
        app::GuiAppMsg::Tabs(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, enum_iterator::Sequence)]
pub enum GuiAppTab {
    Clients,
    Devices,
    Monitors,
    Layers,
    Workspaces,
}

impl From<usize> for GuiAppTab {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Clients,
            1 => Self::Devices,
            2 => Self::Monitors,
            3 => Self::Layers,
            4 => Self::Workspaces,

            _ => Self::Clients,
        }
    }
}

impl From<GuiAppTab> for usize {
    fn from(value: GuiAppTab) -> Self {
        match value {
            GuiAppTab::Clients => 0,
            GuiAppTab::Devices => 1,
            GuiAppTab::Monitors => 2,
            GuiAppTab::Layers => 3,
            GuiAppTab::Workspaces => 4,
        }
    }
}

impl GuiAppTab {
    pub fn label(&self) -> &'static str {
        match self {
            GuiAppTab::Clients => "Clients",
            GuiAppTab::Devices => "Devices",
            GuiAppTab::Monitors => "Monitors",
            GuiAppTab::Layers => "Layers",
            GuiAppTab::Workspaces => "Workspaces",
        }
    }
}

pub struct Tabs {
    current_tab: GuiAppTab,

    clients_tab: ClientsTab,
    devices_tab: DevicesTab,

    monitors: Monitors,
    layers: Layers,
    workspaces: Workspaces,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            current_tab: GuiAppTab::Clients,

            clients_tab: ClientsTab::new(),
            devices_tab: DevicesTab::new(),

            monitors: vec![],
            layers: Default::default(),
            workspaces: vec![],
        }
    }

    pub fn view(&self) -> Element<app::GuiAppMsg> {
        Column::new()
            .push(TabBar::width_tab_labels(
                self.current_tab.into(),
                enum_iterator::all::<GuiAppTab>()
                    .map(|tab| TabLabel::Text(tab.label().to_string()))
                    .collect(),
                |new_tab| TabsMsg::TabChanged(new_tab.into()).into(),
            ))
            .push(match self.current_tab {
                GuiAppTab::Clients => self.clients_tab.view(ClientsTabMsg::Refresh),
                GuiAppTab::Devices => self.devices_tab.view(DevicesTabMsg::Refresh),
                GuiAppTab::Monitors => Self::monitors_tab(&self.monitors),
                GuiAppTab::Layers => Self::layers_tab(&self.layers),
                GuiAppTab::Workspaces => Self::workspaces_tab(&self.workspaces),
            })
            .align_items(Alignment::Center)
            .spacing(15)
            .padding([0, 80, 20, 80])
            .into()
    }

    fn monitors_tab(monitors: &Monitors) -> Element<app::GuiAppMsg> {
        ScrollableList::with_items(
            monitors
                .iter()
                .map(
                    |Monitor {
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
                        vec![
                            format!("Monitor {id} ({name})"),
                            format!("    Size: {width}x{height}"),
                            format!("    Refresh Rate: {refresh_rate}Hz"),
                            format!("    Position: {x}x{y}"),
                            format!(
                                "    Active Workspace: {}",
                                match active_workspace.id {
                                    WorkspaceType::Regular(id) => {
                                        format!("Regular (id: {}) \"{}\"", id, name)
                                    }
                                    WorkspaceType::Special => {
                                        format!("Special \"{}\"", active_workspace.name)
                                    }
                                }
                            ),
                            format!(
                                "    Reserved: ({}, {}, {}, {})",
                                reserved.0, reserved.1, reserved.2, reserved.3
                            ),
                            format!("    Scale: {scale}"),
                            format!(
                                "    Transform: {}",
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
                            ),
                            format!("    Focused: {}", focused),
                        ]
                    },
                )
                .collect(),
        )
        .on_refresh(|| TabsMsg::RefreshMonitors.into())
        .view()
    }

    fn layers_tab(layers: &Layers) -> Element<app::GuiAppMsg> {
        ScrollableList::with_items(
            layers
                .iter()
                .map(
                    |(name, LayerDisplay { levels }): (&String, &LayerDisplay)| {
                        levels.iter().fold(vec![format!("Layer \"{name}\"")], |mut res: Vec<String>, (
                            level_name,
                            layer_clients,
                        ): (
                            &String,
                            &Vec<LayerClient>,
                        )| {
                            res.push(format!("    Level \"{level_name}\""));

                            layer_clients.iter().fold(res, |mut res: Vec<String>, LayerClient { address, x, y, w, h, namespace }: &LayerClient| {
                                res.push(format!("        Client \"{namespace}\""));
                                res.push(format!("            Address: {address}"));
                                res.push(format!("            Position: {x}x{y}"));
                                res.push(format!("            Size: {w}x{h}"));
                                res.push(format!("            Namespace: {namespace}"));
                                res
                            })
                        })
                    },
                )
                .collect(),
        )
            .on_refresh(|| TabsMsg::RefreshLayers.into())
            .view()
    }

    fn workspaces_tab(workspaces: &Workspaces) -> Element<app::GuiAppMsg> {
        ScrollableList::with_items(
            workspaces
                .iter()
                .map(
                    |Workspace {
                         id,
                         name,
                         monitor,
                         windows,
                         fullscreen,
                     }: &Workspace| {
                        vec![
                            format!(
                                "Workspace {} ({})",
                                name,
                                match id {
                                    WorkspaceType::Regular(id) => {
                                        format!("Regular (id: {})", id)
                                    }
                                    WorkspaceType::Special => {
                                        "Special".to_string()
                                    }
                                }
                            ),
                            format!("    Monitor \"{monitor}\"",),
                            format!("    Windows: {}", windows),
                            format!("    Fullscreen: {}", fullscreen),
                        ]
                    },
                )
                .collect(),
        )
        .on_refresh(|| TabsMsg::RefreshWorkspaces.into())
        .view()
    }

    pub fn update(&mut self, msg: TabsMsg) -> Command<app::GuiAppMsg> {
        match msg {
            TabsMsg::TabChanged(new_tab) => {
                self.current_tab = new_tab;

                match new_tab {
                    GuiAppTab::Clients => {
                        if self.clients_tab.is_empty() {
                            return Command::perform(get_clients(), |clients| {
                                ClientsTabMsg::Refreshed(clients).into()
                            });
                        }
                    }
                    GuiAppTab::Devices => {
                        if self.devices_tab.is_empty() {
                            return Command::perform(get_devices(), |devices| {
                                DevicesTabMsg::Refreshed(devices).into()
                            });
                        }
                    }
                    GuiAppTab::Monitors => {
                        if self.monitors.is_empty() {
                            return Command::perform(get_monitors(), |monitors| {
                                TabsMsg::MonitorsRefreshed(monitors).into()
                            });
                        }
                    }
                    GuiAppTab::Layers => {
                        if self.layers.is_empty() {
                            return Command::perform(get_layers(), |layers| {
                                TabsMsg::LayersRefreshed(layers).into()
                            });
                        }
                    }
                    GuiAppTab::Workspaces => {
                        if self.workspaces.is_empty() {
                            return Command::perform(get_workspaces(), |workspaces| {
                                TabsMsg::WorkspacesRefreshed(workspaces).into()
                            });
                        }
                    }
                }
            }
            TabsMsg::Clients(clients_msg) => return self.clients_tab.update(clients_msg),
            TabsMsg::Devices(devices_msg) => return self.devices_tab.update(devices_msg),

            TabsMsg::RefreshMonitors => {
                return Command::perform(get_monitors(), |monitors| {
                    TabsMsg::MonitorsRefreshed(monitors).into()
                });
            }
            TabsMsg::MonitorsRefreshed(new_monitor_list) => self.monitors = new_monitor_list,

            TabsMsg::RefreshLayers => {
                return Command::perform(get_layers(), |layers| {
                    TabsMsg::LayersRefreshed(layers).into()
                });
            }
            TabsMsg::LayersRefreshed(layers) => self.layers = layers,
            TabsMsg::RefreshWorkspaces => {
                return Command::perform(get_workspaces(), |workspaces| {
                    TabsMsg::WorkspacesRefreshed(workspaces).into()
                });
            }
            TabsMsg::WorkspacesRefreshed(workspaces) => self.workspaces = workspaces,
        }

        Command::none()
    }
}

// TODO: make this work later
/*struct MonitorsOutput<'a> {
    monitors: &'a Monitors,
}

impl<'a> Program<GuiAppMsg> for MonitorsOutput<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let bounds = bounds.size();

        let ((x_min, x_max), (y_min, y_max)) = self.monitors.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), monitor: &Monitor| {
                (
                    (
                        x_min.min(monitor.x),
                        x_max.max(monitor.x + monitor.width as i32),
                    ),
                    (
                        y_min.min(monitor.y),
                        y_max.max(monitor.y + monitor.height as i32),
                    ),
                )
            },
        );

        log::info!(
            "x_min: {}, x_max: {}, y_min: {}, y_max: {}",
            x_min,
            x_max,
            y_min,
            y_max
        );

        let scale_x = bounds.width / (x_max - x_min).abs() as f32;
        let scale_y = bounds.height / (y_max - y_min).abs() as f32;

        log::info!("scale_x: {}, scale_y: {}", scale_x, scale_y);

        log::info!("MONITORS: {:?}", self.monitors);

        self.monitors
            .iter()
            .map(|monitor: &Monitor| {
                let rect = Path::rectangle(
                    Point::new(monitor.x as f32 * scale_x, monitor.y as f32 * scale_y),
                    Size::new(
                        monitor.width as f32 * scale_x,
                        monitor.height as f32 * scale_y,
                    ),
                );

                let mut frame = Frame::new(bounds);
                frame.stroke(
                    &rect,
                    Stroke {
                        style: stroke::Style::Solid(Color::BLACK),
                        width: 4.0,
                        line_cap: stroke::LineCap::Square,
                        line_join: stroke::LineJoin::Miter,
                        line_dash: Default::default(),
                    },
                );
                frame.fill_rectangle(
                    Point::new(monitor.x as f32 * scale_x, monitor.y as f32 * scale_y),
                    Size::new(
                        monitor.width as f32 * scale_x,
                        monitor.height as f32 * scale_y,
                    ),
                    Color::WHITE,
                );

                frame.fill_text(format!(
                    "{} ({}) {}x{} {}Hz",
                    monitor.name, monitor.id, monitor.width, monitor.height, monitor.refresh_rate
                ));

                frame.into_geometry()
            })
            .collect()
    }
}*/
