use hyprland::data::{asynchronous as hypr_async, Clients, Devices, Layers, Monitors, Workspaces};

use crate::gui::tabs::templates::RefreshableTabData;

pub async fn get_clients() -> Box<dyn RefreshableTabData> {
    box match hypr_async::get_clients().await {
        Ok(clients) => clients,
        Err(err) => {
            log::error!("Error getting clients: {}", err);
            Clients::new()
        }
    }
}

pub async fn get_devices() -> Box<dyn RefreshableTabData> {
    box match hypr_async::get_devices().await {
        Ok(devices) => devices,
        Err(err) => {
            log::error!("Error getting devices: {}", err);
            Devices {
                mice: vec![],
                keyboards: vec![],
                tablets: vec![],
            }
        }
    }
}

pub async fn get_monitors() -> Box<dyn RefreshableTabData> {
    box match hypr_async::get_monitors().await {
        Ok(monitors) => monitors,
        Err(err) => {
            log::error!("Error getting monitors: {}", err);
            Monitors::new()
        }
    }
}

pub async fn get_layers() -> Layers {
    match hypr_async::get_layers().await {
        Ok(layers) => layers,
        Err(err) => {
            log::error!("Error getting layers: {}", err);
            Layers::new()
        }
    }
}

pub async fn get_workspaces() -> Workspaces {
    match hypr_async::get_workspaces().await {
        Ok(workspaces) => workspaces,
        Err(err) => {
            log::error!("Error getting workspaces: {}", err);
            Workspaces::new()
        }
    }
}
