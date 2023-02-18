use hyprland::data::{
    Clients, Devices, Keyboard, LayerClient, LayerDisplay, Layers, Mouse, Tablet, TabletBelongsTo,
    TabletType, Workspace,
};
use hyprland::{
    data::{
        Client, CursorPosition, Monitor, Monitors, Transforms, Version, WorkspaceBasic, Workspaces,
    },
    prelude::*,
};
use itertools::Itertools;

#[derive(Eq, PartialEq, enum_iterator::Sequence, serde::Serialize, serde::Deserialize)]
pub(crate) enum AppTab {
    General,
    Monitors,
    Workspaces,
    Clients,
    Layers,
    Devices,
}

impl std::fmt::Display for AppTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AppTab::General => "General",
                AppTab::Monitors => "Monitors",
                AppTab::Workspaces => "Workspaces",
                AppTab::Clients => "Clients",
                AppTab::Layers => "Layers",
                AppTab::Devices => "Devices",
            }
        )
    }
}

impl AppTab {
    pub fn selectable_label(&self, ui: &mut egui::Ui, checked: &mut bool) -> egui::Response {
        let response = ui.selectable_label(*checked, self.to_string());

        if response.clicked() {
            *checked = !*checked;
        }

        response
    }

    pub fn window(&self, ui: &mut egui::Ui) {
        egui::Window::new(self.to_string())
            .resizable(true)
            .drag_bounds(ui.clip_rect())
            .show(ui.ctx(), |ui| {
                self.data_view(ui);
            });
    }

    fn data_view(&self, ui: &mut egui::Ui) {
        fn client_data_view(
            ui: &mut egui::Ui,
            client: &Client,
            title: impl Into<egui::WidgetText>,
        ) {
            ui.collapsing(title, |ui| {
                let Client {
                    address,
                    at,
                    size,
                    workspace,
                    floating,
                    fullscreen,
                    fullscreen_mode,
                    monitor,
                    class,
                    title,
                    pid,
                    xwayland,
                    pinned,
                    grouped,
                    swallowing,
                } = client;

                ui.label(format!("PID: {pid}"));
                ui.label(format!("Address: {address}"));
                ui.label(format!("Class: {class}"));
                ui.label(format!("Title: {title}"));
                ui.label(format!("At: {}x{}", at.0, at.1));
                ui.label(format!("Size: {}x{}", size.0, size.1));
                ui.label(format!("Monitor: {monitor}"));
                workspace_basic_data_view(ui, workspace);
                ui.label(format!("Floating: {floating}"));
                ui.label(format!("Fullscreen: {fullscreen} ({fullscreen_mode})"));
                ui.label(format!("XWayland: {xwayland}"));
                ui.label(format!("Pinned: {pinned}"));

                grouped.iter().for_each(|client| {
                    client_data_view(ui, client, "Grouped");
                });

                if let Some(swallowing) = swallowing {
                    client_data_view(ui, swallowing, "Swallowing");
                }
            });
        }

        fn general_data_view(ui: &mut egui::Ui) {
            let mut any_shown = false;

            if let Ok(version) = Version::get() {
                ui.collapsing("Hyprland Version", |ui| {
                    let Version {
                        branch,
                        commit,
                        dirty,
                        commit_message,
                        flags,
                    } = version;

                    ui.label(format!("Branch: {branch}"));
                    ui.label(format!("Commit: {commit}"));
                    ui.label(format!("Dirty: {dirty}"));
                    ui.label(format!("Commit Message: {commit_message}"));
                    ui.label(format!("Flags: [{}]", flags.join(", ")));
                });

                any_shown = true;
            }

            if let Ok(Some(active_window)) = Client::get_active() {
                client_data_view(ui, &active_window, "Active Window");

                any_shown = true;
            }

            if let Ok(cursor_position) = CursorPosition::get() {
                ui.label(format!(
                    "Cursor Position: {}x{}",
                    cursor_position.x, cursor_position.y
                ));

                any_shown = true;
            }

            if !any_shown {
                ui.label("Error: No data available");
            }
        }

        fn transform_data_view(ui: &mut egui::Ui, transform: &Transforms) {
            ui.label(format!(
                "Transform: {}",
                match transform {
                    Transforms::Normal => "Normal",
                    Transforms::Normal90 => "Normal +90",
                    Transforms::Normal180 => "Normal +180",
                    Transforms::Normal270 => "Normal +270",
                    Transforms::Flipped => "Flipped",
                    Transforms::Flipped90 => "Flipped +90",
                    Transforms::Flipped180 => "Flipped +180",
                    Transforms::Flipped270 => "Flipped +270",
                }
            ));
        }

        fn workspace_basic_data_view(ui: &mut egui::Ui, workspace_basic: &WorkspaceBasic) {
            let WorkspaceBasic { id, name } = workspace_basic;

            ui.label(format!("Workspace {name} ({id})"));
        }

        fn monitors_data_view(ui: &mut egui::Ui) {
            match Monitors::get() {
                Ok(monitors) => {
                    monitors.iter().for_each(|monitor| {
                        let Monitor {
                            id,
                            name,
                            description,
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
                            dpms_status,
                        } = monitor;

                        ui.collapsing(format!("Monitor {name} ({id})"), |ui| {
                            ui.label(format!("Description: {description}"));
                            ui.label(format!("Size: {width}x{height}"));
                            ui.label(format!("Refresh Rate: {refresh_rate}"));
                            ui.label(format!("Position: {x}x{y}"));
                            workspace_basic_data_view(ui, active_workspace);
                            ui.label(format!("Reserved: {reserved:?}"));
                            ui.label(format!("Scale: {scale}"));
                            transform_data_view(ui, transform);
                            ui.label(format!("Focused: {focused}"));
                            ui.label(format!("DPMS Status: {dpms_status}"));
                        });
                    });
                }
                Err(error) => {
                    ui.label(format!("Error: {error}"));
                }
            }
        }

        fn workspace_data_view(ui: &mut egui::Ui, workspace: &Workspace) {
            let Workspace {
                id,
                name,
                monitor,
                windows,
                fullscreen,
                last_window,
                last_window_title,
            } = workspace;

            ui.collapsing(format!("Workspace {name} ({id})"), |ui| {
                ui.label(format!("Monitor: {monitor}"));
                ui.label(format!("Fullscreen: {fullscreen}"));
                ui.label(format!(
                    "Windows: {windows} (Last: {last_window_title} ({last_window}))"
                ));
            });
        }

        fn workspaces_data_view(ui: &mut egui::Ui) {
            match Workspaces::get() {
                Ok(workspaces) => {
                    workspaces.iter().for_each(|workspace| {
                        workspace_data_view(ui, workspace);
                    });
                }
                Err(err) => {
                    ui.label(format!("Error: {err}"));
                }
            }
        }

        fn clients_data_view(ui: &mut egui::Ui) {
            match Clients::get() {
                Ok(clients) => {
                    clients.iter().for_each(|client| {
                        client_data_view(ui, client, &client.title);
                    });
                }
                Err(err) => {
                    ui.label(format!("Error: {err}"));
                }
            }
        }

        fn layer_client_data_view(ui: &mut egui::Ui, layer_client: &LayerClient) {
            let LayerClient {
                address,
                x,
                y,
                w,
                h,
                namespace,
            } = layer_client;

            ui.collapsing(format!("LayerClient {address}"), |ui| {
                ui.label(format!("Namespace: {namespace}"));
                ui.label(format!("Size: {w}x{h}"));
                ui.label(format!("Position: {x}x{y}"));
            });
        }

        fn layer_data_view(ui: &mut egui::Ui, (layer_name, layer_data): (&String, &LayerDisplay)) {
            ui.collapsing(format!("Layer {layer_name}"), |ui| {
                ui.heading("Levels");
                ui.separator();

                layer_data
                    .iter()
                    .sorted_by_key(|(level_name, _)| (*level_name).clone())
                    .for_each(|(level_name, level_data)| {
                        ui.collapsing(format!("Level {level_name}"), |ui| {
                            level_data.iter().for_each(|layer_client| {
                                layer_client_data_view(ui, layer_client);
                            });
                        });
                    });
            });
        }

        fn layers_data_view(ui: &mut egui::Ui) {
            match Layers::get() {
                Ok(layers) => {
                    layers
                        .iter()
                        .sorted_by_key(|(layer_name, _)| (*layer_name).clone())
                        .for_each(|layer| {
                            layer_data_view(ui, layer);
                        });
                }
                Err(err) => {
                    ui.label(format!("Error: {err}"));
                }
            }
        }

        fn devices_data_view(ui: &mut egui::Ui) {
            match Devices::get() {
                Ok(devices) => {
                    let Devices {
                        mice,
                        keyboards,
                        tablets,
                    } = devices;
                    let mut any_shown = false;

                    if !mice.is_empty() {
                        ui.collapsing("Mice", |ui| {
                            mice.iter().for_each(|Mouse { address, name }| {
                                ui.label(format!("{address} ({name})"));
                            });
                        });

                        any_shown = true;
                    }

                    if !keyboards.is_empty() {
                        ui.collapsing("Keyboards", |ui| {
                            keyboards.iter().for_each(
                                |Keyboard {
                                     address,
                                     name,
                                     rules,
                                     model,
                                     layout,
                                     variant,
                                     options,
                                     active_keymap,
                                 }| {
                                    ui.collapsing(format!("{name} ({model}) ({address})"), |ui| {
                                        ui.label(format!("Rules: {rules}"));
                                        ui.label(format!("Layout: {layout}"));
                                        ui.label(format!("Variant: {variant}"));
                                        ui.label(format!("Options: {options}"));
                                        ui.label(format!("Active Keymap: {active_keymap}"));
                                    });
                                },
                            );
                        });

                        any_shown = true;
                    }

                    if !tablets.is_empty() {
                        ui.collapsing("Tablets", |ui| {
                            tablets.iter().for_each(
                                |Tablet {
                                     address,
                                     tablet_type,
                                     belongs_to,
                                     name,
                                 }| {
                                    ui.collapsing(
                                        format!(
                                            "{} ({address})",
                                            match name {
                                                Some(name) => name,
                                                None => "Unknown",
                                            }
                                        ),
                                        |ui| {
                                            ui.label(format!(
                                                "Type: {}",
                                                match tablet_type {
                                                    None => {
                                                        "Unknown"
                                                    }
                                                    Some(tablet_type) => {
                                                        match tablet_type {
                                                            TabletType::TabletPad => "Pad",
                                                            TabletType::TabletTool => "Tool",
                                                        }
                                                    }
                                                }
                                            ));
                                            ui.label(format!(
                                                "Belongs to: {}",
                                                match belongs_to {
                                                    None => {
                                                        "None".to_string()
                                                    }
                                                    Some(belongs_to) => {
                                                        match belongs_to {
                                                            TabletBelongsTo::TabletPad {
                                                                name,
                                                                address,
                                                            } => {
                                                                format!("Pad {name} ({address})")
                                                            }
                                                            TabletBelongsTo::Address(address) => {
                                                                address.to_string()
                                                            }
                                                        }
                                                    }
                                                }
                                            ));
                                        },
                                    );
                                },
                            );
                        });

                        any_shown = true;
                    }

                    if !any_shown {
                        ui.label("No devices found");
                    }
                }
                Err(err) => {
                    ui.label(format!("Error: {err}"));
                }
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| match self {
            AppTab::General => general_data_view(ui),
            AppTab::Monitors => monitors_data_view(ui),
            AppTab::Workspaces => workspaces_data_view(ui),
            AppTab::Clients => clients_data_view(ui),
            AppTab::Layers => layers_data_view(ui),
            AppTab::Devices => devices_data_view(ui),
        });
    }
}
