use iced::{
    widget::{button, text, Column, Container, Scrollable, Space},
    Alignment, Element, Length, Padding,
};

use crate::gui::app::GuiAppMsg;

pub struct ScrollableList {
    items: Vec<Vec<String>>,
    section_names: Vec<String>,
    section_space: Length,

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

#[allow(dead_code)]
impl ScrollableList {
    pub fn with_items(items: Vec<Vec<String>>) -> Self {
        Self {
            items,
            section_names: vec![],
            section_space: Length::Units(20),

            width: Length::Fill,
            height: Length::Fill,

            list_height: Length::Units(700),

            on_refresh: None,
            button_width: Length::Units(80),
            button_height: Length::Units(30),

            alignment: Alignment::Center,

            spacing: 40,
            padding: [10, 300, 50, 300].into(),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, GuiAppMsg> {
        let list = Container::new(Scrollable::new(self.items.iter().enumerate().fold(
            Column::new().spacing(10),
            |mut column, (index, item)| {
                let section_name = self.section_names.get(index);
                if let Some(section_name) = section_name {
                    column = column
                        .push(text("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"))
                        .push(text(section_name))
                        .push(text("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"));
                };

                column = match item.len() {
                    0 => column,
                    1 => column.push(text(item[0].clone())),
                    _ => column.push(item.iter().fold(
                        Column::new().width(Length::Fill).spacing(5),
                        |inner_column, text_str| inner_column.push(text(text_str)),
                    )),
                }
                .push(text("--------------------------------"));

                if section_name.is_some() {
                    column = column.push(Space::with_height(self.section_space));
                }

                column.width(Length::Fill).spacing(5)
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
            log::warn!("BUTTON ADDED");
            res = res.push(
                button(text("Refresh"))
                    .height(self.button_height)
                    .width(self.button_width)
                    .on_press(on_refresh()),
            );
        };

        res.into()
    }

    pub fn on_refresh(mut self, on_refresh: fn() -> GuiAppMsg) -> Self {
        self.on_refresh = Some(on_refresh);
        self
    }

    pub fn section_names(mut self, section_names: Vec<String>) -> Self {
        self.section_names = section_names;
        self
    }

    pub fn section_space(mut self, section_space: Length) -> Self {
        self.section_space = section_space;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn list_height(mut self, list_height: Length) -> Self {
        self.list_height = list_height;
        self
    }

    pub fn button_width(mut self, button_width: Length) -> Self {
        self.button_width = button_width;
        self
    }

    pub fn button_height(mut self, button_height: Length) -> Self {
        self.button_height = button_height;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}
