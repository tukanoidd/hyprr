use iced::{
    widget::{button, text, Column, Container, Scrollable},
    Alignment, Element, Length, Padding,
};

use crate::gui::app::GuiAppMsg;

pub struct ScrollableList {
    items: Vec<Vec<String>>,

    width: Length,
    height: Length,

    list_height: Length,

    on_refresh: Option<fn() -> GuiAppMsg>,
    button_width: Length,
    button_height: Length,

    alignment: Alignment,
    spacing: u16,
    padding: Padding,
}

impl ScrollableList {
    pub fn with_items(items: Vec<Vec<String>>) -> ScrollableListBuilder {
        ScrollableListBuilder {
            items,

            width: None,
            height: None,

            list_height: None,

            on_refresh: None,
            button_width: None,
            button_height: None,

            alignment: None,

            spacing: None,
            padding: None,
        }
    }

    pub fn view<'a>(&self) -> iced::Element<'a, GuiAppMsg> {
        let list = Container::new(Scrollable::new(self.items.iter().fold(
            Column::new().spacing(10),
            |column, item| {
                match item.len() {
                    0 => column,
                    1 => column.push(text(item[0].clone())),
                    _ => column.push(item.iter().fold(
                        Column::new().width(Length::Fill).spacing(5),
                        |inner_column, text_str| inner_column.push(text(text_str)),
                    )),
                }
                .push(text("--------------------------------"))
                .width(Length::Fill)
                .spacing(5)
            },
        )))
        .height(self.list_height);

        let mut res = Column::new()
            .push(list)
            .width(self.width)
            .height(self.height)
            .align_items(self.alignment)
            .spacing(self.spacing)
            .padding(self.padding);

        if let Some(on_refresh) = self.on_refresh {
            res = res.push(
                button(text("Refresh"))
                    .height(self.button_height)
                    .width(self.button_width)
                    .on_press(on_refresh()),
            );
        };

        res.into()
    }
}

#[derive(Default)]
pub struct ScrollableListBuilder {
    items: Vec<Vec<String>>,

    list_height: Option<Length>,

    width: Option<Length>,
    height: Option<Length>,

    on_refresh: Option<fn() -> GuiAppMsg>,
    button_width: Option<Length>,
    button_height: Option<Length>,

    alignment: Option<Alignment>,
    spacing: Option<u16>,
    padding: Option<Padding>,
}

impl ScrollableListBuilder {
    pub fn build(self) -> ScrollableList {
        ScrollableList {
            items: self.items,

            on_refresh: None,

            width: self.width.unwrap_or(Length::Fill),
            height: self.height.unwrap_or(Length::Fill),

            list_height: self.list_height.unwrap_or(Length::Units(700)),

            button_width: self.button_width.unwrap_or(Length::Units(80)),
            button_height: self.button_height.unwrap_or(Length::Units(30)),

            alignment: self.alignment.unwrap_or(Alignment::Center),
            spacing: self.spacing.unwrap_or(40),
            padding: self
                .padding
                .map(Into::into)
                .unwrap_or([10, 300, 50, 300].into()),
        }
    }

    pub fn view<'a>(self) -> Element<'a, GuiAppMsg> {
        self.build().view()
    }

    pub fn on_refresh(mut self, on_refresh: fn() -> GuiAppMsg) -> Self {
        self.on_refresh = Some(on_refresh);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    pub fn list_height(mut self, list_height: Length) -> Self {
        self.list_height = Some(list_height);
        self
    }

    pub fn button_width(mut self, button_width: Length) -> Self {
        self.button_width = Some(button_width);
        self
    }

    pub fn button_height(mut self, button_height: Length) -> Self {
        self.button_height = Some(button_height);
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }
}
