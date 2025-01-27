mod devices;
pub mod stats_type;
pub mod types;

use crate::device::{models::access_points::AccessPoint, Device};
use crate::responses::stat::devices::DeviceListResponse;
use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderMap,
    Client, Url,
};
use types::Site;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct ReturnedDevices {
    pub AccessPoints: Vec<AccessPoint>,
    pub Switches: Vec<Device>,
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
                //print!("{:?}", response.text().await?);
                let mut aps: Vec<AccessPoint> = Vec::new();
                let mut switches: Vec<Device> = Vec::new();
                let response = match response.json::<DeviceListResponse>().await {
                    Ok(response) => response,
                    Err(e) => panic!("{:?}", e),
                };
                for device in response.data {
                    if device.adopted {
                        if device.type_field == "uap".to_string() {
                            aps.push(AccessPoint::from(device));
                        } else {
                            switches.push(Device::from(device));
                        }
                    }
                }
                return Ok(ReturnedDevices {
                    AccessPoints: aps,
                    Switches: switches,
                });
            }
            Err(e) => return Err(Box::new(e)),
        };
    }
}
