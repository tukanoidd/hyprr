use hyprland::{
    data::{asynchronous as hypr_async, Monitor, Monitors, Transforms},
    shared::WorkspaceType,
};
use iced::{
    executor,
    widget::{
        button,
        canvas::{Cursor, Frame, Geometry, Program},
        column, text, Column, Container, Scrollable,
    },
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Renderer, Size,
    Theme,
};
use iced_aw::{TabBar, TabLabel};

#[derive(Debug, Copy, Clone, enum_iterator::Sequence)]
pub enum GuiAppTab {
    Monitors,
    Test,
}

impl GuiAppTab {
    fn label(&self) -> &'static str {
        match self {
            GuiAppTab::Monitors => "Monitors",
            GuiAppTab::Test => "Test",
        }
    }

    fn view<'a>(&self, monitors: &'a Monitors) -> Element<'a, GuiAppMsg> {
        match self {
            GuiAppTab::Monitors => Self::monitors_tab(monitors),
            GuiAppTab::Test => iced::widget::text("Test").into(),
        }
    }

    fn monitors_tab(monitors: &Monitors) -> Element<GuiAppMsg> {
        let list = Container::new(
            Scrollable::new(
                column(
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
                                Column::new()
                                    .push(text(format!("Monitor {id} ({name})")))
                                    .push(text(format!("    Size: {width}x{height}")))
                                    .push(text(format!("    Refresh Rate: {refresh_rate}Hz")))
                                    .push(text(format!("    Position: {x}x{y}")))
                                    .push(text(format!(
                                        "    Active Workspace: {}",
                                        match active_workspace.id {
                                            WorkspaceType::Regular(id) => {
                                                format!("Regular (id: {}) \"{}\"", id, name)
                                            }
                                            WorkspaceType::Special => {
                                                format!("Special \"{}\"", active_workspace.name)
                                            }
                                        }
                                    )))
                                    .push(text(format!(
                                        "    Reserved: ({}, {}, {}, {})",
                                        reserved.0, reserved.1, reserved.2, reserved.3
                                    )))
                                    .push(text(format!("    Scale: {scale}")))
                                    .push(text(format!(
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
                                    )))
                                    .push(text(format!("    Focused: {}", focused)))
                                    .push(text("--------------------------------"))
                                    .width(Length::Fill)
                                    .spacing(5)
                                    .into()
                            },
                        )
                        .collect(),
                )
                .spacing(10),
            )
            .height(Length::Units(700)),
        );

        Column::new()
            .push(list)
            .push(
                button(text("Refresh"))
                    .height(Length::Units(30))
                    .width(Length::Units(80))
                    .on_press(GuiAppMsg::RefreshMonitors),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(40)
            .padding([10, 300, 50, 300])
            .into()
    }
}

impl From<usize> for GuiAppTab {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::Test,
            _ => Self::Monitors,
        }
    }
}

impl From<GuiAppTab> for usize {
    fn from(value: GuiAppTab) -> Self {
        match value {
            GuiAppTab::Monitors => 0,
            GuiAppTab::Test => 1,
        }
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
                let mut frame = Frame::new(bounds);
                frame.fill_rectangle(
                    Point::new(monitor.x as f32 * scale_x, monitor.y as f32 * scale_y),
                    Size::new(
                        monitor.width as f32 * scale_x,
                        monitor.height as f32 * scale_y,
                    ),
                    Color::BLACK,
                );

                frame.into_geometry()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum GuiAppMsg {
    TabChanged(GuiAppTab),

    RefreshMonitors,
    MonitorsRefreshed(Monitors),
}

pub struct GuiApp {
    current_tab: GuiAppTab,

    monitors: Monitors,
}

impl GuiApp {
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
                current_tab: GuiAppTab::Monitors,
                monitors: vec![],
            },
            Command::perform(Self::get_monitors(), GuiAppMsg::MonitorsRefreshed),
        )
    }

    #[inline]
    fn title(&self) -> String {
        "Hyprr".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GuiAppMsg::TabChanged(new_tab) => {
                self.current_tab = new_tab;
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
            .push(self.current_tab.view(&self.monitors))
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
