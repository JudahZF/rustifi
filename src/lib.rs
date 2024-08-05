mod auth;
mod controller;

use auth::sign_in;
use controller::sites::Site;
use reqwest::{cookie::Jar, header::HeaderMap, Client, Url};
use std::{borrow::Borrow, sync::Arc};

pub struct UnifiController {
    addr: String,
    api_root: String,
    cookie_store: Arc<Jar>,
    is_udm: bool,
    client: Client,
    sites: Vec<Site>,
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
            api_root = addr.clone() + "/proxy/network";
        }

        match sign_in(&client, &addr, username.clone(), password.clone(), is_udm).await {
            Ok(_) => {}
            Err(_) => panic!("Unable to sign into the controller API"),
        };

        let mut sites = Vec::new();
        let url = Url::parse((addr.clone() + "/stat/sites").as_str()).unwrap();

        match controller::sites::get_sites(&client, &cookie_store, url).await {
            Ok(out) => {
                for site in out.data {
                    sites.push(Site {
                        name: site.name,
                        client: client.clone(),
                        id: site._id.clone(),
                        api_addr: api_root.clone() + "/s/" + site._id.as_str(),
                        headers: out.headers.clone(),
                    });
                }
            }
            Err(e) => return Err(e),
        };

        Ok(UnifiController {
            addr,
            is_udm,
            cookie_store,
            api_root,
            client,
            sites,
        })
    }

    pub fn get_sites(&self) -> Vec<Site> {
        self.sites.clone()
    }
}
