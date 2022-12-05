use std::collections::HashMap;

use hyprland::data::{Devices, Keyboard, Mouse, Tablet, TabletBelongsTo, TabletType};
use iced::{
    widget::{button, text, Column, Scrollable, Space},
    Alignment, Command, Element, Length,
};

use crate::gui::{
    app, dropdown_button::DropdownButton, tabs::TabsMsg, wrapper_functions::get_devices,
};

const MICE_CATEGORY: &str = "Mice";
const KEYBOARDS_CATEGORY: &str = "Keyboards";
const TABLETS_CATEGORY: &str = "Tablets";

#[derive(Debug, Clone)]
pub enum DevicesTabMsg {
    Refresh,
    Refreshed(Devices),
    ToggleDropdown(String),
}

impl From<DevicesTabMsg> for app::GuiAppMsg {
    fn from(value: DevicesTabMsg) -> Self {
        app::GuiAppMsg::Tabs(TabsMsg::Devices(value))
    }
}

pub struct DevicesTab {
    devices: Devices,

    dropdowns_open: HashMap<String, bool>,
}

impl DevicesTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            devices: Devices {
                mice: vec![],
                keyboards: vec![],
                tablets: vec![],
            },
            dropdowns_open: HashMap::new(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.mice.is_empty()
            || self.devices.keyboards.is_empty()
            || self.devices.tablets.is_empty()
    }

    pub fn view(&self) -> Element<app::GuiAppMsg> {
        let refresh_button = button(text("Refresh"))
            .height(Length::Units(30))
            .width(Length::Units(80))
            .on_press(DevicesTabMsg::Refresh.into());

        let mice = DropdownButton::new(MICE_CATEGORY)
            .add_children(
                self.devices
                    .mice
                    .iter()
                    .map(|Mouse { address, name }: &Mouse| {
                        let title = format!("Mouse {address}");

                        DropdownButton::new(&title)
                            .add_child(text(format!("Name: {name}")))
                            .view(
                                self.dropdowns_open.get(&title).copied().unwrap_or(false),
                                DevicesTabMsg::ToggleDropdown(title).into(),
                            )
                    }),
            )
            .view(
                self.dropdowns_open
                    .get(MICE_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                DevicesTabMsg::ToggleDropdown(MICE_CATEGORY.to_string()).into(),
            );

        let keyboards = DropdownButton::new(KEYBOARDS_CATEGORY)
            .add_children(self.devices.keyboards.iter().map(
                |Keyboard {
                     address,
                     name,
                     rules,
                     model,
                     layout,
                     variant,
                     options,
                     active_keymap,
                 }: &Keyboard| {
                    let title = format!("Keyboard \"{name}\"");

                    DropdownButton::new(&title)
                        .add_child(text(format!("Address: {address}")))
                        .add_child(text(format!("Rules: {rules}")))
                        .add_child(text(format!("Model: {model}")))
                        .add_child(text(format!("Layout: {layout}")))
                        .add_child(text(format!("Variant: {variant}")))
                        .add_child(text(format!("Options: {options}")))
                        .add_child(text(format!("Active Keymap: {active_keymap}")))
                        .view(
                            self.dropdowns_open.get(&title).copied().unwrap_or(false),
                            DevicesTabMsg::ToggleDropdown(title).into(),
                        )
                },
            ))
            .view(
                self.dropdowns_open
                    .get(KEYBOARDS_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                DevicesTabMsg::ToggleDropdown(KEYBOARDS_CATEGORY.to_string()).into(),
            );

        let mut unknwon_num = 0;
        let tablets = DropdownButton::new(TABLETS_CATEGORY)
            .add_children(self.devices.tablets.iter().map(
                |Tablet {
                     address,
                     tablet_type,
                     belongs_to,
                     name,
                 }: &Tablet| {
                    let title = format!(
                        "Tablet \"{}\"",
                        name.clone().unwrap_or({
                            unknwon_num += 1;
                            format!("Unknown {}", unknwon_num - 1)
                        })
                    );

                    DropdownButton::new(&title)
                        .add_child(text(format!("Address: {address}")))
                        .add_child(text(format!(
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
                        )))
                        .add_child(text(format!(
                            "    Belongs to: {}",
                            match belongs_to {
                                Some(belongs_to) => {
                                    match belongs_to {
                                        TabletBelongsTo::TabletPad { name, address } => {
                                            format!(
                                                "Tablet Pad \"{}\" (address: {})",
                                                name, address
                                            )
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
                        )))
                        .view(
                            self.dropdowns_open.get(&title).copied().unwrap_or(false),
                            DevicesTabMsg::ToggleDropdown(title).into(),
                        )
                },
            ))
            .view(
                self.dropdowns_open
                    .get(TABLETS_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                DevicesTabMsg::ToggleDropdown(TABLETS_CATEGORY.to_string()).into(),
            );

        let devices = Column::<app::GuiAppMsg>::new()
            .push(refresh_button)
            .push(Space::with_height(Length::Units(20)))
            .push(mice)
            .push(keyboards)
            .push(tablets)
            .align_items(Alignment::Center)
            .spacing(10);

        Scrollable::<app::GuiAppMsg>::new(devices).into()
    }

    pub fn update(&mut self, msg: DevicesTabMsg) -> Command<app::GuiAppMsg> {
        match msg {
            DevicesTabMsg::Refresh => {
                return Command::perform(get_devices(), |devices| {
                    DevicesTabMsg::Refreshed(devices).into()
                })
            }
            DevicesTabMsg::Refreshed(devices) => {
                self.devices = devices;

                if !self.dropdowns_open.contains_key(MICE_CATEGORY) {
                    self.dropdowns_open.insert(MICE_CATEGORY.to_string(), false);
                }

                if !self.dropdowns_open.contains_key(KEYBOARDS_CATEGORY) {
                    self.dropdowns_open
                        .insert(KEYBOARDS_CATEGORY.to_string(), false);
                }

                if !self.dropdowns_open.contains_key(TABLETS_CATEGORY) {
                    self.dropdowns_open
                        .insert(TABLETS_CATEGORY.to_string(), false);
                }

                self.devices
                    .mice
                    .iter()
                    .for_each(|Mouse { address, .. }: &Mouse| {
                        let title = format!("Mouse {address}");

                        if !self.dropdowns_open.contains_key(&title) {
                            self.dropdowns_open.insert(title.clone(), false);
                        }
                    });

                self.devices
                    .keyboards
                    .iter()
                    .for_each(|Keyboard { name, .. }: &Keyboard| {
                        let title = format!("Keyboard \"{name}\"");

                        if !self.dropdowns_open.contains_key(&title) {
                            self.dropdowns_open.insert(title.clone(), false);
                        }
                    });

                let mut unknwon_num = 0;

                self.devices
                    .tablets
                    .iter()
                    .for_each(|Tablet { name, .. }: &Tablet| {
                        let title = format!(
                            "Tablet \"{}\"",
                            name.clone().unwrap_or({
                                unknwon_num += 1;
                                format!("Unknown {}", unknwon_num - 1)
                            })
                        );

                        if !self.dropdowns_open.contains_key(&title) {
                            self.dropdowns_open.insert(title.clone(), false);
                        }
                    });
            }
            DevicesTabMsg::ToggleDropdown(title) => {
                self.dropdowns_open
                    .entry(title)
                    .and_modify(|val: &mut bool| {
                        *val = !*val;
                    })
                    .or_insert(false);
            }
        }

        Command::none()
    }
}
