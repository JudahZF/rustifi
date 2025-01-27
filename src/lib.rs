pub mod auth;
pub mod controller;
pub mod device;
pub mod responses;
pub mod site;
pub mod types;
mod utils;

use auth::sign_in;
use reqwest::{cookie::Jar, Client};
use site::types::Site;
use std::sync::Arc;

pub struct UnifiController {
    addr: String,
    api_root: String,
    cookie_store: Arc<Jar>,
    client: Client,
    sites: Vec<Site>,
    pub current_site: Option<Site>,
}

impl UnifiController {
    pub async fn new(
        addr: String,
        username: String,
        password: String,
        is_udm: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let jar = Jar::default();
        let cookie_store = Arc::new(jar);

        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent("unifi-rs")
            .cookie_store(true)
            .cookie_provider(cookie_store.clone())
            .build()?;

        let mut api_root = addr.clone() + "/api";

        if is_udm {
            api_root = addr.clone() + "/proxy/network/api";
        }

        match sign_in(&client, &addr, username.clone(), password.clone(), is_udm).await {
            Ok(_) => {}
            Err(_) => panic!("Unable to sign into the controller API"),
        };

        let sites = match controller::get_sites(&client, &cookie_store, api_root.clone()).await {
            Ok(sites) => sites,
            Err(e) => panic!("Unable to get the sites: {}", e),
        };

        Ok(UnifiController {
            addr,
            cookie_store,
            api_root,
            client,
            sites,
            current_site: None,
        })
    }

    pub fn get_sites(&self) -> Vec<Site> {
        self.sites.clone()
    }

    pub fn set_site(&mut self, site_name: String) -> Result<(), Box<dyn std::error::Error>> {
        for site in self.sites.clone() {
            if site.name == site_name {
                let mut new_site = site.clone();
                new_site.set_active(self.client.clone(), &self.cookie_store);
                self.current_site = Some(new_site);
                return Ok(());
            }
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Site not found",
        )))
    }
}
