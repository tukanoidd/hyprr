use std::{collections::HashMap, fmt::Debug};

use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;
use iced::{
    widget::{button, text, Column, Scrollable, Space},
    Alignment, Command, Element, Length,
};

use crate::gui::{
    app::GuiAppMsg,
    tabs::{GuiAppTab, TabsMsg},
};

#[macro_export]
macro_rules! refreshable_tab_impl {
    (data: $var_name:ident<$ty:path> $([query: $fun_path:path])* $([dropdowns: $drop_var_name:ident])*) => {
        paste::paste! {
            fn data(&self) -> &dyn $crate::gui::tabs::templates::RefreshableTabData {
                &*self.$var_name
            }

            #[inline]
            fn set_data(&mut self, value: Box<dyn $crate::gui::tabs::templates::RefreshableTabData>) {
                if let Ok(val) = value.downcast::<$ty>() {
                    self.$var_name = val;
                }
            }

            $($crate::refreshable_tab_impl!(query: $fun_path);)*
            $($crate::refreshable_tab_impl!(dropdowns: $drop_var_name);)*
        }
    };

    (query: $fun_path:path) => {
        #[inline]
        fn query_data_static() -> iced_futures::BoxFuture<Box<dyn RefreshableTabData>> {
            Box::pin($fun_path())
        }

        #[inline]
        fn query_data(&self) -> iced_futures::BoxFuture<Box<dyn RefreshableTabData>> {
            Box::pin($fun_path())
        }
    };

    (dropdowns: $drop_var_name:ident) => {
        #[inline]
        fn dropdowns_open(&self) -> Option<&HashMap<String, bool>> {
            Some(&self.$drop_var_name)
        }

        #[inline]
        fn dropdowns_open_mut(&mut self) -> Option<&mut HashMap<String, bool>> {
            Some(&mut self.$drop_var_name)
        }
    };

    (dropdowns) => {
        $crate::refreshable_tab_impl!(dropdowns: dropdowns_open);
    }
}

pub use refreshable_tab_impl;

pub trait RefreshableTabData: Debug + DynClone + Downcast + Send {
    fn titles(&self) -> Vec<String>;
    fn is_empty(&self) -> bool;
}

dyn_clone::clone_trait_object!(RefreshableTabData);
impl_downcast!(RefreshableTabData);

pub trait RefreshableTabDataUnit {
    fn title(&self) -> String;

    #[inline]
    fn unknown(&self, unknown_num: usize) -> String {
        format!("Unknown {unknown_num}")
    }
}

#[derive(Debug, Clone)]
pub enum RefreshableTabMsg {
    Refresh,
    Refreshed(Box<dyn RefreshableTabData>),

    ToggleDropdown(String),
}

pub trait RefreshableTabTemplate: Debug + Send + Sync {
    fn view(&self, on_refresh: RefreshableTabMsg) -> Element<GuiAppMsg> {
        let refresh_button = button(text("Refresh"))
            .height(Length::Units(30))
            .width(Length::Units(80))
            .on_press(GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                self.app_tab(),
                on_refresh,
            )));

        let list = Column::new()
            .push(refresh_button)
            .push(Space::with_height(Length::Units(20)))
            .align_items(Alignment::Center)
            .spacing(10);

        let list_with_info = self.add_info_to_list(list);

        Scrollable::new(list_with_info).into()
    }

    fn update(&mut self, msg: RefreshableTabMsg) -> Command<GuiAppMsg> {
        match msg {
            RefreshableTabMsg::Refresh => {
                let tab = self.app_tab();

                return Command::perform(self.query_data(), move |data| {
                    GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                        tab,
                        RefreshableTabMsg::Refreshed(data),
                    ))
                });
            }
            RefreshableTabMsg::Refreshed(data) => {
                self.set_data(data);

                log::info!("New Data: {:?}", self.data());

                if self.dropdowns_open().is_some() {
                    self.data().titles().iter().for_each(|title| {
                        if !self.dropdowns_open().unwrap().contains_key(title) {
                            self.dropdowns_open_mut()
                                .unwrap()
                                .insert(title.clone(), false);
                        }
                    })
                }

                self.custom_dropdown_checks()
            }
            RefreshableTabMsg::ToggleDropdown(title) => {
                if let Some(dropdowns_open) = self.dropdowns_open_mut() {
                    dropdowns_open
                        .entry(title)
                        .and_modify(|val: &mut bool| {
                            *val = !*val;
                        })
                        .or_insert(false);
                }
            }
        }

        Command::none()
    }

    fn add_info_to_list<'a>(&'a self, list: Column<'a, GuiAppMsg>) -> Column<'a, GuiAppMsg>;

    #[inline]
    fn is_empty(&self) -> bool {
        self.data().is_empty()
    }

    fn data(&self) -> &dyn RefreshableTabData;
    fn set_data(&mut self, data: Box<dyn RefreshableTabData>);

    fn query_data_static() -> iced_futures::BoxFuture<Box<dyn RefreshableTabData>>
    where
        Self: Sized;

    fn query_data(&self) -> iced_futures::BoxFuture<Box<dyn RefreshableTabData>>;

    #[inline]
    fn dropdowns_open(&self) -> Option<&HashMap<String, bool>> {
        None
    }

    #[inline]
    fn dropdowns_open_mut(&mut self) -> Option<&mut HashMap<String, bool>> {
        None
    }

    fn custom_dropdown_checks(&mut self) {}

    fn app_tab(&self) -> GuiAppTab;
}
