use hyprland::data::{asynchronous as hypr_async, Clients, Devices, Layers, Monitors, Workspaces};

pub async fn get_clients() -> Clients {
    match hypr_async::get_clients().await {
        Ok(clients) => clients,
        Err(err) => {
            log::error!("Error getting clients: {}", err);
            Clients::new()
        }
    }
}

pub async fn get_devices() -> Devices {
    match hypr_async::get_devices().await {
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

pub async fn get_monitors() -> Monitors {
    match hypr_async::get_monitors().await {
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
