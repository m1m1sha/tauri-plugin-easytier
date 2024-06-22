use std::{
    collections::BTreeMap,
    io::Error,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use dashmap::DashMap;
use easytier::{common::config::ConfigLoader as _, launcher::NetworkInstance};
use tauri::{command, AppHandle, Manager, Runtime};

use crate::{models::*, Result};

static INSTANCE: once_cell::sync::Lazy<DashMap<String, NetworkInstance>> =
    once_cell::sync::Lazy::new(DashMap::new);

static EMIT_INSTANCE_INFO: once_cell::sync::Lazy<AtomicBool> =
    once_cell::sync::Lazy::new(|| AtomicBool::new(false));

#[command]
pub(crate) fn parse_network_config(cfg: NetworkConfig) -> Result<String> {
    let toml = cfg.gen_config().map_err(|e| e.to_string()).unwrap();
    Ok(toml.dump())
}

#[command]
pub(crate) fn stop_network_instance(id: String) -> Result<()> {
    let _ = INSTANCE.remove(&id);
    Ok(())
}

#[command]
pub(crate) fn collect_network_infos() -> Result<BTreeMap<String, NetworkInstanceInfo>> {
    let mut ret = BTreeMap::new();
    for instance in INSTANCE.iter() {
        if let Some(info) = instance.get_running_info() {
            ret.insert(
                instance.key().clone(),
                NetworkInstanceInfo {
                    id: instance.key().clone().to_lowercase(),
                    node: info.my_node_info,
                    events: info.events,
                    routes: info.routes,
                    peers: info.peers,
                    peer_route_pairs: info.peer_route_pairs,
                    running: info.running,
                    error: info.error_msg,
                },
            );
        }
    }
    Ok(ret)
}

#[command]
pub(crate) async fn start_network_instance<R: Runtime>(
    app: AppHandle<R>,
    cfg: NetworkConfig,
) -> Result<()> {
    if INSTANCE.contains_key(&cfg.id) {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "network instance already exists",
        )
        .into());
    }
    let id = cfg.id.clone();
    let cfg = cfg.gen_config().map_err(|e| e.to_string()).unwrap();
    let mut instance = NetworkInstance::new(cfg);
    // get android fd
    instance.start().map_err(|e| e.to_string()).unwrap();

    if !EMIT_INSTANCE_INFO.load(Ordering::Relaxed) {
        EMIT_INSTANCE_INFO.store(true, Ordering::Relaxed);
        tokio::spawn(async move {
            let mut ret = vec![];
            let mut flag = 0;
            loop {
                for instance in INSTANCE.iter() {
                    if let Some(info) = instance.get_running_info() {
                        ret.push(NetworkInstanceInfo {
                            id: instance.key().clone().to_lowercase(),
                            node: info.my_node_info,
                            events: info.events,
                            routes: info.routes,
                            peers: info.peers,
                            peer_route_pairs: info.peer_route_pairs,
                            running: info.running,
                            error: info.error_msg,
                        });
                    }
                }

                if ret.is_empty() {
                    flag += 1;
                    if flag > 5 {
                        EMIT_INSTANCE_INFO.store(false, Ordering::Relaxed);
                        break;
                    }
                } else {
                    flag = 0;
                }

                let _ = app.emit("network_instance_info", &ret);
                ret.clear();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    INSTANCE.insert(id, instance);
    Ok(())
}
