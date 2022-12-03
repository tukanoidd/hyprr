use hyprland::{
    data::{asynchronous as hypr_async, Client, Clients, Monitor, Monitors, Transforms},
    shared::WorkspaceType,
};
use iced::{
    executor,
    widget::{
        canvas::{stroke, Cursor, Frame, Geometry, Path, Program, Stroke},
        Column,
    },
    Alignment, Application, Color, Command, Element, Point, Rectangle, Renderer, Size, Theme,
};
use iced_aw::{TabBar, TabLabel};

use super::scrollable_list::ScrollableList;

#[derive(Debug, Copy, Clone, PartialEq, enum_iterator::Sequence)]
pub enum GuiAppTab {
    Clients,
    Monitors,
}

impl From<usize> for GuiAppTab {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::Monitors,
            _ => Self::Clients,
        }
    }
}

impl From<GuiAppTab> for usize {
    fn from(value: GuiAppTab) -> Self {
        match value {
            GuiAppTab::Clients => 0,
            GuiAppTab::Monitors => 1,
        }
    }
}

impl GuiAppTab {
    fn label(&self) -> &'static str {
        match self {
            GuiAppTab::Clients => "Clients",
            GuiAppTab::Monitors => "Monitors",
        }
    }

    fn view<'a>(&self, clients: &Clients, monitors: &'a Monitors) -> Element<'a, GuiAppMsg> {
        match self {
            GuiAppTab::Clients => Self::clients_tab(clients),
            GuiAppTab::Monitors => Self::monitors_tab(monitors),
        }
    }

    fn clients_tab<'a>(clients: &Clients) -> Element<'a, GuiAppMsg> {
        ScrollableList::with_items(
            clients
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
        .on_refresh(|| GuiAppMsg::RefreshMonitors)
        .view()
    }
}

struct MonitorsOutput<'a> {
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
}

#[derive(Debug, Clone)]
pub enum GuiAppMsg {
    TabChanged(GuiAppTab),

    RefreshClients,
    ClientsRefreshed(Clients),

    RefreshMonitors,
    MonitorsRefreshed(Monitors),
}

pub struct GuiApp {
    current_tab: GuiAppTab,

    clients: Clients,
    monitors: Monitors,
}

impl GuiApp {
    async fn get_clients() -> Clients {
        match hypr_async::get_clients().await {
            Ok(clients) => clients,
            Err(err) => {
                log::error!("Error getting clients: {}", err);
                Clients::new()
            }
        }
    }

    async fn get_monitors() -> Monitors {
        match hypr_async::get_monitors().await {
            Ok(monitors) => monitors,
            Err(err) => {
                log::error!("Error getting monitors: {}", err);
                Vec::new()
            }
        }
    }
}

impl Application for GuiApp {
    type Executor = executor::Default;
    type Message = GuiAppMsg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                current_tab: GuiAppTab::Clients,
                clients: vec![],
                monitors: vec![],
            },
            Command::perform(Self::get_clients(), GuiAppMsg::ClientsRefreshed),
        )
    }

    #[inline]
    fn title(&self) -> String {
        "Hyprr".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GuiAppMsg::TabChanged(new_tab) => {
                if self.current_tab == new_tab {
                    return Command::none();
                }

                self.current_tab = new_tab;

                match new_tab {
                    GuiAppTab::Clients => {
                        if self.clients.is_empty() {
                            return Command::perform(
                                Self::get_clients(),
                                GuiAppMsg::ClientsRefreshed,
                            );
                        }
                    }
                    GuiAppTab::Monitors => {
                        if self.monitors.is_empty() {
                            return Command::perform(
                                Self::get_monitors(),
                                GuiAppMsg::MonitorsRefreshed,
                            );
                        }
                    }
                }
            }
            GuiAppMsg::RefreshClients => {
                return Command::perform(Self::get_clients(), GuiAppMsg::ClientsRefreshed);
            }
            GuiAppMsg::ClientsRefreshed(new_clients) => {
                self.clients = new_clients;
            }
            GuiAppMsg::RefreshMonitors => {
                return Command::perform(Self::get_monitors(), GuiAppMsg::MonitorsRefreshed);
            }
            GuiAppMsg::MonitorsRefreshed(new_monitor_list) => {
                self.monitors = new_monitor_list;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        Column::new()
            .push(TabBar::width_tab_labels(
                self.current_tab.into(),
                enum_iterator::all::<GuiAppTab>()
                    .map(|tab| TabLabel::Text(tab.label().to_string()))
                    .collect(),
                |new_tab| GuiAppMsg::TabChanged(new_tab.into()),
            ))
            .push(self.current_tab.view(&self.clients, &self.monitors))
            .align_items(Alignment::Center)
            .spacing(15)
            .padding([0, 80, 20, 80])
            .into()
    }

    #[inline]
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
