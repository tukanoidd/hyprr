use std::collections::HashMap;

use hyprland::data::{Devices, Keyboard, Mouse, Tablet, TabletBelongsTo, TabletType};
use iced::widget::{text, Column};

use crate::gui::tabs::templates::RefreshableTabMsg;
use crate::gui::tabs::TabsMsg;
use crate::{
    gui::{
        app::GuiAppMsg,
        dropdown_button::DropdownButton,
        tabs::{
            templates::{RefreshableTabData, RefreshableTabDataUnit, RefreshableTabTemplate},
            GuiAppTab,
        },
        wrapper_functions::get_devices,
    },
    refreshable_tab_impl,
};

const MICE_CATEGORY: &str = "Mice";
const KEYBOARDS_CATEGORY: &str = "Keyboards";
const TABLETS_CATEGORY: &str = "Tablets";

impl RefreshableTabDataUnit for Mouse {
    #[inline]
    fn title(&self) -> String {
        format!("Mouse {}", self.address)
    }
}

impl RefreshableTabDataUnit for Keyboard {
    #[inline]
    fn title(&self) -> String {
        format!("Keyboard \"{}\"", self.name)
    }
}

impl RefreshableTabDataUnit for Tablet {
    #[inline]
    fn title(&self) -> String {
        format!("Tablet \"{}\"", self.name.as_ref().unwrap())
    }
}

impl RefreshableTabData for Devices {
    fn titles(&self) -> Vec<String> {
        let mut res = Vec::new();

        res.extend(self.mice.iter().map(RefreshableTabDataUnit::title));
        res.extend(self.keyboards.iter().map(RefreshableTabDataUnit::title));

        let mut unknwon_num = 0;

        res.extend(
            self.tablets
                .iter()
                .map(|tablet: &Tablet| match tablet.name.is_some() {
                    true => tablet.title(),
                    false => {
                        unknwon_num += 1;
                        tablet.unknown(unknwon_num - 1)
                    }
                }),
        );

        res
    }

    fn is_empty(&self) -> bool {
        self.mice.is_empty() || self.keyboards.is_empty() || self.tablets.is_empty()
    }
}

#[derive(Debug)]
pub struct DevicesTab {
    devices: Box<Devices>,

    dropdowns_open: HashMap<String, bool>,
}

impl DevicesTab {
    #[inline]
    pub fn new() -> Self {
        Self {
            devices: box Devices {
                mice: vec![],
                keyboards: vec![],
                tablets: vec![],
            },
            dropdowns_open: HashMap::new(),
        }
    }
}

impl RefreshableTabTemplate for DevicesTab {
    fn add_info_to_list<'a>(&'a self, list: Column<'a, GuiAppMsg>) -> Column<'a, GuiAppMsg> {
        let mice = DropdownButton::new(MICE_CATEGORY)
            .add_children(self.devices.mice.iter().map(|mouse: &Mouse| {
                let title = mouse.title();

                let Mouse { name, .. } = mouse;

                DropdownButton::new(&title)
                    .add_child(text(format!("Name: {name}")))
                    .view(
                        self.dropdowns_open.get(&title).copied().unwrap_or(false),
                        GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                            GuiAppTab::Devices,
                            RefreshableTabMsg::ToggleDropdown(title),
                        )),
                    )
            }))
            .view(
                self.dropdowns_open
                    .get(MICE_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                    GuiAppTab::Devices,
                    RefreshableTabMsg::ToggleDropdown(MICE_CATEGORY.to_string()),
                )),
            );

        let keyboards = DropdownButton::new(KEYBOARDS_CATEGORY)
            .add_children(self.devices.keyboards.iter().map(|keyboard: &Keyboard| {
                let title = keyboard.title();

                let Keyboard {
                    address,
                    rules,
                    model,
                    layout,
                    variant,
                    options,
                    active_keymap,
                    ..
                } = keyboard;

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
                        GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                            GuiAppTab::Devices,
                            RefreshableTabMsg::ToggleDropdown(title),
                        )),
                    )
            }))
            .view(
                self.dropdowns_open
                    .get(KEYBOARDS_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                    GuiAppTab::Devices,
                    RefreshableTabMsg::ToggleDropdown(KEYBOARDS_CATEGORY.to_string()),
                )),
            );

        let mut unknwon_num = 0;
        let tablets = DropdownButton::new(TABLETS_CATEGORY)
            .add_children(self.devices.tablets.iter().map(|tablet: &Tablet| {
                let title = match tablet.name.is_some() {
                    true => tablet.title(),
                    false => {
                        unknwon_num += 1;
                        tablet.unknown(unknwon_num - 1)
                    }
                };

                let Tablet {
                    address,
                    tablet_type,
                    belongs_to,
                    ..
                } = tablet;

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
                    )))
                    .view(
                        self.dropdowns_open.get(&title).copied().unwrap_or(false),
                        GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                            GuiAppTab::Devices,
                            RefreshableTabMsg::ToggleDropdown(title),
                        )),
                    )
            }))
            .view(
                self.dropdowns_open
                    .get(TABLETS_CATEGORY)
                    .copied()
                    .unwrap_or(false),
                GuiAppMsg::Tabs(TabsMsg::RefreshableTab(
                    GuiAppTab::Devices,
                    RefreshableTabMsg::ToggleDropdown(TABLETS_CATEGORY.to_string()),
                )),
            );

        list.push(mice).push(keyboards).push(tablets)
    }

    refreshable_tab_impl!(data: devices<Devices> [query: get_devices]);
    refreshable_tab_impl!(dropdowns);

    fn custom_dropdown_checks(&mut self) {
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
    }

    #[inline]
    fn app_tab(&self) -> GuiAppTab {
        GuiAppTab::Devices
    }
}
