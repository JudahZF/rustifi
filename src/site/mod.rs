mod devices;
mod stat;
pub mod stats_type;
pub mod types;

use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderMap,
    Client, Url,
};
use stat::{devices::res_types::DeviceStatsList, devices_basic::res_types::DeviceListResponse};
use types::{Device, Site};

impl Site {
    pub fn set_active(&mut self, client: Client, jar: &Jar) {
        self.client = Some(client);
        self.cookies = Some(
            jar.cookies(&Url::parse(self.addr.as_str()).unwrap())
                .unwrap(),
        );
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>, Box<dyn std::error::Error>> {
        let url = Url::parse((self.addr.clone() + "/stat/device-basic").as_str()).unwrap();
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
                //print!("{:?}", response.text().await?);
                let mut devices = Vec::new();
                let response = response.json::<DeviceListResponse>().await?;
                for device in response.data {
                    if device.adopted && !device.disabled {
                        devices.push(Device::new(
                            device.mac,
                            device.state,
                            device.type_field,
                            device.model,
                            device.in_gateway_mode,
                            device.name,
                        ));
                    }
                }
                return Ok(devices);
            }
            Err(e) => return Err(Box::new(e)),
        };
    }
    pub async fn get_device_stats(
        &self, //,
               //mac_addr: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                //print!("{:?}", response.text().await?);
                //let mut devices = Vec::new();
                let response = response.json::<DeviceStatsList>().await?;
                print!("{:?}", response.data);
                //for device in response.data {
                //    if device.adopted && !device.disabled {
                //        devices.push(Device::new(
                //            device.mac,
                //            device.state,
                //            device.type_field,
                //            device.model,
                //            device.in_gateway_mode,
                //            device.name,
                //        ));
                //    }
                //}
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        };
    }
}
