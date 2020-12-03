use crate::domain::models::product::{Product, ProductOpts, MinimalSite};
use crate::domain::models::site::Site;
use crate::domain::result::*;
use rusqlite::{Connection};
use std::collections::{HashMap};

use super::storage::storage::{select_all_sites, select_all_products, insert_products, insert_sites, insert_junctions};
use super::storage::init::*;

static DB_NAME: &str = "products.db";

pub async fn select_products(opts: ProductOpts) -> Result<Vec<Product>> {
    select_all_products(opts, create_connection().await?).await
}

pub async fn select_sites() -> Result<Vec<Site>> {
    select_all_sites(create_connection().await?).await
}

pub async fn update_db(products: Vec<Product>, sites: &Vec<Site>, mapping: &Vec<MinimalSite>) -> Result<()> {
    let mut product_map = HashMap::new();
    for product in products {
        product_map.insert(String::from(&product.product_id), product);
    }
    let mut assembled_products = Vec::new();
    for site in mapping {
        for prod in &site.products{
            if let Some(found) = product_map.remove(&prod.product_id) {
                let mut p = found;
                p.link = p.construct_link();
                assembled_products.push(p);
            }
        }
    }
    tokio::try_join!(insert_products(&assembled_products, create_connection().await?), insert_sites(sites, create_connection().await?))?;
    insert_junctions(mapping, create_connection().await?).await?;
    Ok(())
}

pub async fn init_db() -> Result<()>{
    info!("Initializing database {}", DB_NAME);
    tokio::try_join!(init_product_db(create_connection().await?), init_site_db(create_connection().await?))?;
    init_junction_db(create_connection().await?).await?;
    Ok(())
}

async fn create_connection() -> Result<Connection> {
    Connection::open(DB_NAME)
        .map_err(|e| e.into())
}