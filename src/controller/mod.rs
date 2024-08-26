mod stat;
use crate::site::types::Site;
use stat::sites::res_types::SiteListResponse;

use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderMap,
    Client, Url,
};

pub async fn get_sites(
    client: &Client,
    jar: &Jar,
    addr: String,
) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let url = Url::parse((addr.clone() + "/stat/sites").as_str()).unwrap();
    let cookies = jar.cookies(&url);
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookies.unwrap());
    match client.get(url).headers(headers.clone()).send().await {
        Ok(response) => {
            //println!("{:?}", response.text().await?);
            let mut sites = Vec::new();
            let response = match response.json::<SiteListResponse>().await {
                Ok(response) => response,
                Err(e) => panic!("{:?}", e),
            };
            for site in response.data {
                sites.push(Site {
                    name: site.name,
                    id: site.id.clone(),
                    addr: addr.clone() + "/s/" + site.attr_hidden_id.as_str(),
                    client: None,
                    cookies: None,
                });
            }
            return Ok(sites);
        }
        Err(e) => return Err(Box::new(e)),
    };
}
