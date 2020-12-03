use crate::domain::models::product::{ProductOpts, Product, SiteResponse};
use crate::external::client::ApiCaller;
use crate::database::api;
use crate::domain::result::Result;
use crate::domain::models::site::Site;

lazy_static! {
    static ref CALLER: ApiCaller = ApiCaller::new();
}

pub async fn update_db() -> Result<()> {
    let products = CALLER.request_products_and_stores().await?;
    api::update_db(products.0, &products.1, &products.2).await
}

pub async fn fetch_products(opts: ProductOpts) -> Result<Vec<Product>> {
    api::select_products(opts).await
}

pub async fn init_db() -> Result<()> {
    api::init_db().await
}

pub async fn fetch_site_names() -> Result<Vec<SiteResponse>> {
    let sites: Vec<Site> = api::select_sites().await?;
    let mut names = Vec::with_capacity(sites.len());
    for site in sites {
        if site.name != "" {
            names.push(SiteResponse{site_id: site.site_id, site_name: site.name});
        }
    }
    Ok(names)
}
