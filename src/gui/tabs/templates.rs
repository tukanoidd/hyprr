use iced::{
    widget::{button, text, Column, Scrollable, Space},
    Alignment, Command, Element, Length,
};

pub trait TabTemplate {
    type Message;
    type AppMessage;

    fn view(&self, on_refresh: Self::Message) -> Element<Self::AppMessage>
    where
        <Self as TabTemplate>::AppMessage: Clone,
        Self::Message: Into<Self::AppMessage>,
    {
        let refresh_button = button(text("Refresh"))
            .height(Length::Units(30))
            .width(Length::Units(80))
            .on_press(on_refresh.into());

        let list = Column::new()
            .push(refresh_button)
            .push(Space::with_height(Length::Units(20)))
            .align_items(Alignment::Center)
            .spacing(10);

        let list_with_info = self.add_info_to_list(list);

        Scrollable::new(list_with_info).into()
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::AppMessage>;

    fn add_info_to_list<'a>(
        &'a self,
        list: Column<'a, Self::AppMessage>,
    ) -> Column<'a, Self::AppMessage>;

    fn is_empty(&self) -> bool;
}
