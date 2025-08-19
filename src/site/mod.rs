pub mod types;

use crate::device::{AccessPoint, Device};
use crate::responses::stat::devices::DeviceListResponse;
use reqwest::{
    Client, Url,
    cookie::{CookieStore, Jar},
    header::HeaderMap,
};
use types::Site;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct ReturnedDevices {
    pub access_points: Vec<AccessPoint>,
    pub switches: Vec<Device>,
}

impl Site {
    pub fn set_active(&mut self, client: Client, jar: &Jar) {
        self.client = Some(client);
        self.cookies = Some(
            jar.cookies(&Url::parse(self.addr.as_str()).unwrap())
                .unwrap(),
        );
    }

    pub async fn get_devices(&self) -> Result<ReturnedDevices, Box<dyn std::error::Error>> {
        let url = Url::parse((self.addr.clone() + "/stat/device").as_str()).unwrap();
        let cookies = self.cookies.clone();
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", cookies.unwrap());
        match self
            .client
            .as_ref()
            .unwrap()
            .get(url)
            .headers(headers.clone())
            .send()
            .await
        {
            Ok(response) => {
                let mut aps: Vec<AccessPoint> = Vec::new();
                let mut switches: Vec<Device> = Vec::new();
                let resp_text = response.text().await.unwrap();
                let response = match serde_json::from_str::<DeviceListResponse>(&resp_text) {
                    Ok(response) => response,
                    Err(e) => {
                        println!("Error parsing JSON: {}", e);
                        return Err(Box::new(e));
                    }
                };
                for device in response.data {
                    if device.adopted {
                        if device.type_field == *"uap" {
                            aps.push(AccessPoint::from(device));
                        } else {
                            switches.push(Device::from(device));
                        }
                    }
                }
                Ok(ReturnedDevices {
                    access_points: aps,
                    switches,
                })
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
