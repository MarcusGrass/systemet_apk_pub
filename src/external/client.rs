use reqwest;
use serde_json;
use crate::domain::models::product::{Product, MinimalSite};
use crate::domain::arithmetic::{get_apk, get_recyc_apk};
use crate::domain::models::site::Site;
use crate::domain::result::Result;
use std::time::SystemTime;
use reqwest::Client;

#[derive(Debug, Clone)]
struct EmptyVec;

static PRODUCTS_URL: &str = "https://api-extern.systembolaget.se/product/v1/product";
static PRODUCTS_AND_SITES: &str = "https://api-extern.systembolaget.se/product/v1/product/getproductswithstore";
static SITE_URL: &str = "https://api-extern.systembolaget.se/site/v1/site";

static HEADER_KEY: &str = "";
static HEADER_VAL: &str = "";



pub struct ApiCaller {
    client: Client
}

impl ApiCaller {
    pub async fn request_products_and_stores(&self) -> Result<(Vec<Product>, Vec<Site>, Vec<MinimalSite>)> {
        let prod_sites_both = tokio::try_join!(self.request_all_products(), self.request_all_sites(), self.request_products_with_store())?;
        return Ok(prod_sites_both);
    }

    async fn request_products_with_store(&self) -> Result<Vec<MinimalSite>> {
        info!("Sending http request to url={}", PRODUCTS_AND_SITES);
        let http_time = SystemTime::now();
        let res = self.client.get(PRODUCTS_AND_SITES)
            .header(HEADER_KEY, HEADER_VAL)
            .send()
            .await?;
        let body = res.text().await?;
        info!("Products and sites received, http round trip was: {} millis", SystemTime::now().duration_since(http_time)?.as_millis());
        let processing = SystemTime::now();
        let stores: Vec<MinimalSite> = serde_json::from_str(&body)?;
        info!("Products and sites deserialization complete, processing time was: {} millis", SystemTime::now().duration_since(processing)?.as_millis());
        Ok(stores)
    }

    async fn request_all_sites(&self) -> Result<Vec<Site>> {
        info!("Sending http request to url={}", SITE_URL);
        let http_time = SystemTime::now();
        let res = self.client.get(SITE_URL)
            .header(HEADER_KEY, HEADER_VAL)
            .send()
            .await?;
        let body = res.text().await?;
        info!("Sites received, http round trip was: {} millis", SystemTime::now().duration_since(http_time)?.as_millis());
        let processing = SystemTime::now();
        let sites = serde_json::from_str(&body)
            .map_err(|e| e.into());
        info!("Sites deserialization complete, processing time was: {} millis", SystemTime::now().duration_since(processing)?.as_millis());
        sites
    }

    pub async fn request_all_products(&self) -> Result<Vec<Product>> {
        info!("Sending http request to url={}", PRODUCTS_URL);
        let http_time = SystemTime::now();
        let res = self.client.get(PRODUCTS_URL)
            .header(HEADER_KEY, HEADER_VAL)
            .send()
            .await?;
        let body = res.text().await?;
        info!("Products received, http round trip was: {} millis", SystemTime::now().duration_since(http_time)?.as_millis());
        let processing = SystemTime::now();
        let products = serde_json::from_str(&body)
            .map(|p| {
                ApiCaller::add_apk(p)
            })?;
        info!("Products deserialization complete, processing time was: {} millis", SystemTime::now().duration_since(processing)?.as_millis());
        Ok(products)
    }

    fn add_apk(mut products: Vec<Product>) -> Vec<Product> {
        for elem in products.iter_mut() {
            elem.apk = get_apk(elem);
            elem.apk_recycling = get_recyc_apk(elem);
        }
        products
    }

    pub fn new() -> Self {
        let client = Client::new();
        ApiCaller { client }
    }
}


