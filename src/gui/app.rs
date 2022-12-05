use iced::{executor, Application, Command, Element, Renderer, Theme};

use crate::gui::{
    tabs::{clients::ClientsTabMsg, Tabs, TabsMsg},
    wrapper_functions::*,
};

#[derive(Debug, Clone)]
pub enum GuiAppMsg {
    Tabs(TabsMsg),
}

pub struct GuiApp {
    tabs: Tabs,
}

impl Application for GuiApp {
    type Executor = executor::Default;
    type Message = GuiAppMsg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self { tabs: Tabs::new() },
            Command::perform(get_clients(), |clients| {
                GuiAppMsg::Tabs(TabsMsg::Clients(ClientsTabMsg::Refreshed(clients)))
            }),
        )
    }

    #[inline]
    fn title(&self) -> String {
        "Hyprr".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        return match message {
            GuiAppMsg::Tabs(tabs_msg) => self.tabs.update(tabs_msg),
        };

        //Command::none()
    }

    #[inline]
    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        self.tabs.view()
    }

    #[inline]
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
