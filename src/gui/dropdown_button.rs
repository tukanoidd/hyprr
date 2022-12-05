use iced::{
    widget::{button, text, Column},
    Alignment, Element, Length, Padding,
};

pub struct DropdownButton<'a, Message>
where
    Message: 'a + Clone,
{
    text: String,
    children: Vec<Element<'a, Message>>,

    width: Length,
    height: Length,

    padding: Padding,
}

#[allow(dead_code)]
impl<'a, Message> DropdownButton<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(text: &str) -> Self {
        Self::with_children(text, vec![])
    }

    pub fn with_children(text: &str, children: Vec<Element<'a, Message>>) -> Self {
        Self {
            text: text.to_string(),
            children,

            width: Length::Fill,
            height: Length::Shrink,

            padding: Padding::from([10, 5]),
        }
    }

    pub fn add_child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn add_children(mut self, children: impl Iterator<Item = Element<'a, Message>>) -> Self {
        self.children.extend(children);
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

    pub fn view(self, is_open: bool, on_press: Message) -> Element<'a, Message> {
        let button = button(text(format!(
            "{} {}",
            if is_open { "▼" } else { "▶" },
            self.text,
        )))
        .padding([5, 10])
        .on_press(on_press);

        let res = Column::new()
            .push(button)
            .width(self.width)
            .height(self.height)
            .padding(self.padding)
            .spacing(10)
            .align_items(Alignment::Start);

        match is_open {
            true => res.push(
                self.children.into_iter().fold(
                    Column::<'a, Message>::new()
                        .width(Length::Fill)
                        .padding([5, 5, 30, 5])
                        .align_items(Alignment::Start),
                    |col, child| col.push(child),
                ),
            ),
            false => res,
        }
        .into()
    }
}
