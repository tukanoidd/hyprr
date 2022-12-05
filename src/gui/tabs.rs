pub mod clients;

use hyprland::{
    data::{
        Devices, Keyboard, LayerClient, LayerDisplay, Layers, Monitor, Monitors, Mouse, Tablet,
        TabletBelongsTo, TabletType, Transforms, Workspace, Workspaces,
    },
    shared::WorkspaceType,
};
use iced::{widget::Column, Alignment, Command, Element};
use iced_aw::{TabBar, TabLabel};

use crate::gui::{
    app::GuiAppMsg,
    scrollable_list::ScrollableList,
    tabs::clients::{ClientsTab, ClientsTabMsg},
    wrapper_functions::*,
};

#[derive(Debug, Clone)]
pub enum TabsMsg {
    TabChanged(GuiAppTab),
    Clients(ClientsTabMsg),

    /*Devices(DevicesTabMsg),
    Monitors(MonitorsTabMsg),
    Layers(LayersTabMsg),
    Workspaces(WorkspacesTabMsg),*/
    RefreshDevices,
    DevicesRefreshed(Devices),

    RefreshMonitors,
    MonitorsRefreshed(Monitors),

    RefreshLayers,
    LayersRefreshed(Layers),

    RefreshWorkspaces,
    WorkspacesRefreshed(Workspaces),
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

    devices: Devices,
    monitors: Monitors,
    layers: Layers,
    workspaces: Workspaces,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            current_tab: GuiAppTab::Devices,

            clients_tab: ClientsTab::new(),

            devices: Devices {
                mice: vec![],
                keyboards: vec![],
                tablets: vec![],
            },
            monitors: vec![],
            layers: Default::default(),
            workspaces: vec![],
        }
    }

    pub fn view(&self) -> Element<GuiAppMsg> {
        Column::new()
            .push(TabBar::width_tab_labels(
                self.current_tab.into(),
                enum_iterator::all::<GuiAppTab>()
                    .map(|tab| TabLabel::Text(tab.label().to_string()))
                    .collect(),
                |new_tab| GuiAppMsg::Tabs(TabsMsg::TabChanged(new_tab.into())),
            ))
            .push(match self.current_tab {
                GuiAppTab::Clients => self.clients_tab.view(),
                GuiAppTab::Devices => Self::devices_tab(&self.devices),
                GuiAppTab::Monitors => Self::monitors_tab(&self.monitors),
                GuiAppTab::Layers => Self::layers_tab(&self.layers),
                GuiAppTab::Workspaces => Self::workspaces_tab(&self.workspaces),
            })
            .align_items(Alignment::Center)
            .spacing(15)
            .padding([0, 80, 20, 80])
            .into()
    }

    fn devices_tab(devices: &Devices) -> Element<GuiAppMsg> {
        let mice = &devices.mice;
        let keyboards = &devices.keyboards;
        let tablets = &devices.tablets;

        let mut res = Vec::new();

        res.push(mice.iter().fold(
            Vec::new(),
            |mut res: Vec<String>, Mouse { address, name }: &Mouse| {
                res.push(format!("Mouse {address}"));
                res.push(format!("    Name: {name}"));

                res
            },
        ));
        res.push(keyboards.iter().fold(
            Vec::new(),
            |mut res: Vec<String>,
             Keyboard {
                 address,
                 name,
                 rules,
                 model,
                 layout,
                 variant,
                 options,
                 active_keymap,
             }: &Keyboard| {
                res.push(format!("Keyboard \"{name}\""));
                res.push(format!("    Address: {address}"));
                res.push(format!("    Rules: {rules}"));
                res.push(format!("    Model: {model}"));
                res.push(format!("    Layout: {layout}"));
                res.push(format!("    Variant: {variant}"));
                res.push(format!("    Options: {options}"));
                res.push(format!("    Active Keymap: {active_keymap}"));

                res
            },
        ));
        res.push(tablets.iter().fold(
            Vec::new(),
            |mut res: Vec<String>,
             Tablet {
                 address,
                 tablet_type,
                 belongs_to,
                 name,
             }: &Tablet| {
                res.push(format!(
                    "Tablet \"{}\"",
                    name.clone().unwrap_or("Unknown".to_string())
                ));
                res.push(format!("    Address: {address}"));
                res.push(format!(
                    "    Type: {}",
                    match tablet_type {
                        Some(tablet_type) => {
                            match tablet_type {
                                TabletType::TabletPad => "Tablet Pad",
                                TabletType::TabletTool => "Tablet Tool",
                            }
                        }
                        None => {
                            "Unknown"
                        }
                    }
                ));
                res.push(format!(
                    "    Belongs to: {}",
                    match belongs_to {
                        Some(belongs_to) => {
                            match belongs_to {
                                TabletBelongsTo::TabletPad { name, address } => {
                                    format!("Tablet Pad \"{}\" (address: {})", name, address)
                                }
                                TabletBelongsTo::Address(address) => {
                                    format!("Address: {})", address)
                                }
                            }
                        }
                        None => {
                            "Unknown".to_string()
                        }
                    }
                ));

                res
            },
        ));

        ScrollableList::with_items(res)
            .section_names(vec![
                "Mice".to_string(),
                "Keyboards".to_string(),
                "Tablets".to_string(),
            ])
            .on_refresh(|| GuiAppMsg::Tabs(TabsMsg::RefreshDevices))
            .view()
    }

    fn monitors_tab(monitors: &Monitors) -> Element<GuiAppMsg> {
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
        .on_refresh(|| GuiAppMsg::Tabs(TabsMsg::RefreshMonitors))
        .view()
    }

    fn layers_tab(layers: &Layers) -> Element<GuiAppMsg> {
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
            .on_refresh(|| GuiAppMsg::Tabs(TabsMsg::RefreshLayers))
            .view()
    }

    fn workspaces_tab(workspaces: &Workspaces) -> Element<GuiAppMsg> {
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
        .on_refresh(|| GuiAppMsg::Tabs(TabsMsg::RefreshWorkspaces))
        .view()
    }

    pub fn update(&mut self, msg: TabsMsg) -> Command<GuiAppMsg> {
        match msg {
            TabsMsg::TabChanged(new_tab) => {
                self.current_tab = new_tab;

                match new_tab {
                    GuiAppTab::Clients => {
                        if self.clients_tab.is_empty() {
                            return Command::perform(get_clients(), |clients| {
                                GuiAppMsg::Tabs(TabsMsg::Clients(ClientsTabMsg::Refreshed(clients)))
                            });
                        }
                    }
                    GuiAppTab::Monitors => {
                        if self.monitors.is_empty() {
                            return Command::perform(get_monitors(), |monitors| {
                                GuiAppMsg::Tabs(TabsMsg::MonitorsRefreshed(monitors))
                            });
                        }
                    }
                    GuiAppTab::Devices => {
                        if self.devices.keyboards.is_empty() && self.devices.mice.is_empty() {
                            return Command::perform(get_devices(), |devices| {
                                GuiAppMsg::Tabs(TabsMsg::DevicesRefreshed(devices))
                            });
                        }
                    }
                    GuiAppTab::Layers => {
                        if self.layers.is_empty() {
                            return Command::perform(get_layers(), |layers| {
                                GuiAppMsg::Tabs(TabsMsg::LayersRefreshed(layers))
                            });
                        }
                    }
                    GuiAppTab::Workspaces => {
                        if self.workspaces.is_empty() {
                            return Command::perform(get_workspaces(), |workspaces| {
                                GuiAppMsg::Tabs(TabsMsg::WorkspacesRefreshed(workspaces))
                            });
                        }
                    }
                }
            }
            TabsMsg::Clients(clients_msg) => return self.clients_tab.update(clients_msg),
            TabsMsg::RefreshDevices => {
                return Command::perform(get_devices(), |devices| {
                    GuiAppMsg::Tabs(TabsMsg::DevicesRefreshed(devices))
                });
            }
            TabsMsg::DevicesRefreshed(devices) => self.devices = devices,

            TabsMsg::RefreshMonitors => {
                return Command::perform(get_monitors(), |monitors| {
                    GuiAppMsg::Tabs(TabsMsg::MonitorsRefreshed(monitors))
                });
            }
            TabsMsg::MonitorsRefreshed(new_monitor_list) => self.monitors = new_monitor_list,

            TabsMsg::RefreshLayers => {
                return Command::perform(get_layers(), |layers| {
                    GuiAppMsg::Tabs(TabsMsg::LayersRefreshed(layers))
                });
            }
            TabsMsg::LayersRefreshed(layers) => self.layers = layers,
            TabsMsg::RefreshWorkspaces => {
                return Command::perform(get_workspaces(), |workspaces| {
                    GuiAppMsg::Tabs(TabsMsg::WorkspacesRefreshed(workspaces))
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
