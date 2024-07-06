use anyhow::Context;
use chrono::{DateTime, Local};
use easytier::{
    common::{
        config::{ConfigLoader, NetworkIdentity, PeerConfig, TomlConfigLoader, VpnPortalConfig},
        global_ctx::GlobalCtxEvent,
    },
    launcher::MyNodeInfo,
    rpc::{PeerInfo, Route},
    utils::PeerRoutePair,
};
use serde::{Deserialize, Serialize};

use std::net::Ipv4Addr;
#[cfg(mobile)]
use std::os::unix::io::RawFd;

// #[cfg(mobile)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FdRequest {
    pub ip: String,
}

#[cfg(mobile)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FdResponse {
    pub fd: Option<RawFd>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NetworkConfig {
    pub id: String,
    pub dhcp: bool,
    pub ipv4: Option<String>,
    pub device_name: Option<String>,
    pub token: Option<String>,
    pub network_name: Option<String>,
    pub network_secret: Option<String>,
    pub peer_urls: Vec<String>,
    pub proxy_cidrs: Option<Vec<String>>,
    pub vpn_portal_port: Option<u32>,
    pub vpn_portal_addr: Option<String>,
    pub listener_urls: Vec<String>,
    pub rpc_port: Option<u32>,
}

impl NetworkConfig {
    pub fn gen_config(&self) -> Result<TomlConfigLoader, anyhow::Error> {
        let cfg = TomlConfigLoader::default();
        cfg.set_id(
            self.id
                .parse()
                .with_context(|| format!("failed to parse instance id: {}", self.id))?,
        );
        cfg.set_hostname(self.device_name.clone());

        if self.network_name.is_none() && self.token.is_none() {
            return Err(anyhow::anyhow!("no token or network provided"));
        }

        let (n_name, n_secret) = if self.network_name.is_none() {
            let digest = md5::compute(self.token.clone().unwrap());
            let str = format!("{:x}", digest);
            (
                str.get(0..8).unwrap_or_default().to_owned(),
                str.get(8..).unwrap_or_default().to_owned(),
            )
        } else {
            (
                self.network_name.clone().unwrap(),
                self.network_secret.clone().unwrap_or_default(),
            )
        };

        cfg.set_inst_name(n_name.clone());
        cfg.set_network_identity(NetworkIdentity::new(n_name, n_secret));

        cfg.set_dhcp(self.dhcp);
        if !self.dhcp && self.ipv4.is_some() {
            let ipv4 = self.ipv4.clone().unwrap();
            if ipv4.len() > 0 {
                cfg.set_ipv4(Some(ipv4.parse::<Ipv4Addr>().with_context(|| {
                    format!("failed to parse ipv4 address: {:?}", self.ipv4)
                })?))
            }
        }

        let mut peers = vec![];
        for peer_url in self.peer_urls.iter() {
            if peer_url.is_empty() {
                continue;
            }
            peers.push(PeerConfig {
                uri: peer_url
                    .parse()
                    .with_context(|| format!("failed to parse peer uri: {}", peer_url))?,
            });
        }

        if peers.len() == 0 {
            return Err(anyhow::anyhow!("no peer urls provided"));
        }

        cfg.set_peers(peers);

        let mut listener_urls = vec![];
        for listener_url in self.listener_urls.iter() {
            if listener_url.is_empty() {
                continue;
            }
            listener_urls.push(
                listener_url
                    .parse()
                    .with_context(|| format!("failed to parse listener uri: {}", listener_url))?,
            );
        }
        cfg.set_listeners(listener_urls);

        if let Some(proxy_cidrs) = self.proxy_cidrs.clone() {
            for n in proxy_cidrs.iter() {
                cfg.add_proxy_cidr(
                    n.parse()
                        .with_context(|| format!("failed to parse proxy network: {}", n))?,
                );
            }
        }

        cfg.set_rpc_portal(
            format!("127.0.0.1:{}", self.rpc_port.unwrap_or_default())
                .parse()
                .with_context(|| {
                    format!(
                        "failed to parse rpc portal port: {}",
                        self.rpc_port.unwrap_or_default()
                    )
                })?,
        );

        if self.vpn_portal_addr.is_some() {
            let cidr = format!("{}/24", self.vpn_portal_addr.clone().unwrap());
            cfg.set_vpn_portal_config(VpnPortalConfig {
                client_cidr: cidr
                    .parse()
                    .with_context(|| format!("failed to parse vpn portal client cidr: {}", cidr))?,
                wireguard_listen: format!("0.0.0.0:{}", self.vpn_portal_port.unwrap_or(22022))
                    .parse()
                    .with_context(|| {
                        format!(
                            "failed to parse vpn portal wireguard listen port. {}",
                            self.vpn_portal_port.unwrap_or(22022)
                        )
                    })?,
            });
        }

        Ok(cfg)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetworkInstanceInfo {
    pub id: String,
    pub node: MyNodeInfo,
    pub events: Vec<(DateTime<Local>, GlobalCtxEvent)>,
    pub routes: Vec<Route>,
    pub peers: Vec<PeerInfo>,
    #[serde(rename(deserialize = "camelCase"))]
    pub peer_route_pairs: Vec<PeerRoutePair>,
    pub running: bool,
    pub error: Option<String>,
}
