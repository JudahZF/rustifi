use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderMap,
    Client, Url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GwSystemStats {
    cpu: i8,
    mem: i8,
    uptime: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteHealth {
    drops: Option<i8>,
    gateways: Option<Vec<String>>,
    gw_mac: Option<String>,
    gw_name: Option<String>,
    gw_system_stats: Option<GwSystemStats>,
    gw_version: Option<String>,
    lan_ip: Option<String>,
    latency: Option<i8>,
    nameservers: Option<Vec<String>>,
    netmask: Option<String>,
    num_adopted: Option<i32>,
    num_ap: Option<i32>,
    num_disabled: Option<i32>,
    num_disconnected: Option<i32>,
    num_guest: Option<i32>,
    num_gw: Option<i32>,
    num_iot: Option<i32>,
    num_pending: Option<i32>,
    num_sta: Option<i32>,
    num_sw: Option<i32>,
    num_user: Option<i32>,
    remote_user_enabled: Option<bool>,
    remote_user_num_active: Option<i32>,
    remote_user_num_inactive: Option<i32>,
    remote_user_rx_bytes: Option<i32>,
    remote_user_rx_packets: Option<i32>,
    remote_user_tx_bytes: Option<i32>,
    remote_user_tx_packets: Option<i32>,
    rx_bytes_r: Option<i32>,
    site_to_site_enabled: Option<bool>,
    speedtest_lastrun: Option<i32>,
    speedtest_ping: Option<i16>,
    speedtest_status: Option<String>,
    status: String,
    subsystem: String,
    tx_bytes_r: Option<i32>,
    uptime: Option<i32>,
    wan_ip: Option<String>,
    xput_down: Option<i32>,
    xput_up: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseSite {
    attr_hidden_id: String,
    attr_no_delete: bool,
    desc: String,
    health: Vec<SiteHealth>,
    pub _id: String,
    pub name: String,
    anonymous_id: String,
    num_new_alarms: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Meta {
    rc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Response {
    meta: Meta,
    data: Vec<ResponseSite>,
}

pub struct Out {
    pub headers: HeaderMap,
    pub data: Vec<ResponseSite>,
}

pub async fn get_sites(
    client: &Client,
    jar: &Jar,
    url: Url,
) -> Result<Out, Box<dyn std::error::Error>> {
    let cookies = jar.cookies(&url);
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookies.unwrap());
    match client.get(url).headers(headers.clone()).send().await {
        Ok(response) => {
            let response = response.json::<Response>().await?;
            return Ok(Out {
                headers,
                data: response.data,
            });
        }
        Err(e) => return Err(Box::new(e)),
    };
}

#[derive(Debug, Clone)]
pub struct Site {
    pub api_addr: String,
    pub client: Client,
    pub headers: HeaderMap,
    pub name: String,
    pub id: String,
}

impl Site {
    pub async fn get_active_clients(&self) -> Result<(), Box<dyn std::error::Error>> {
        print!(
            "{:?}",
            self.client
                .get(Url::parse((self.api_addr.clone() + "/stat/sta").as_str()).unwrap())
                .headers(self.headers.clone())
                .send()
                .await?
        );
        Ok(())
    }

    pub async fn get_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.id.clone())
    }

    pub async fn get_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.name.clone())
    }
}
