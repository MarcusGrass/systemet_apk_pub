use rusqlite::{Connection, NO_PARAMS};
use crate::domain::models::product::{Product, ProductOpts, MinimalSite};
use crate::domain::models::site::{Site, Position};
use crate::domain::result::Result;
use crate::database::storage::query_utils;
use std::time::SystemTime;

pub async fn insert_products(products: &Vec<Product>, mut con: Connection) -> Result<()> {
    let start = SystemTime::now();
    info!("Starting transaction to insert {} products", products.len());
    let transaction = con.transaction()?;
    transaction.execute_batch("DELETE FROM products;")?;
    for product in products {
        transaction.execute(
            "
              INSERT INTO products (
              product_id,
              product_number,
              product_name_bold,
              product_name_thin,
              category,
              product_number_short,
              producer_name,
              supplier_name,
              is_kosher,
              bottle_text_short,
              restricted_parcel_quantity,
              seal,
              is_organic,
              is_ethical,
              ethical_label,
              is_web_launch,
              sell_start_date,
              is_completely_out_of_stock,
              is_temporary_out_of_stock,
              alcohol_percentage,
              volume,
              price,
              country,
              origin_level1,
              origin_level2,
              vintage,
              sub_category,
              a_type,
              style,
              assortment_text,
              beverage_description_short,
              usage_text,
              taste,
              assortment,
              is_manufacturing_country,
              recycle_fee,
              is_regional_retricted,
              is_in_store_search_assortment,
              is_news,
              apk,
              apk_recycling,
              link)
        VALUES (
              ?1,
              ?2,
              ?3,
              ?4,
              ?5,
              ?5,
              ?7,
              ?8,
              ?9,
              ?10,
              ?11,
              ?12,
              ?13,
              ?14,
              ?15,
              ?16,
              ?17,
              ?18,
              ?19,
              ?20,
              ?21,
              ?22,
              ?23,
              ?24,
              ?25,
              ?26,
              ?27,
              ?28,
              ?29,
              ?30,
              ?31,
              ?32,
              ?33,
              ?34,
              ?35,
              ?36,
              ?37,
              ?38,
              ?39,
              ?40,
              ?41,
              ?42
        )", params![
        product.product_id.as_str(),
        product.product_number.as_str(),
        product.product_name_bold.as_str(),
        product.product_name_thin.as_str(),
        product.category.as_str().to_lowercase(),
        product.product_number_short.as_str(),
        product.producer_name.as_str(),
        product.supplier_name.as_str(),
        product.is_kosher,
        product.bottle_text_short.as_str(),
        product.restricted_parcel_quantity,
        product.seal.as_str(),
        product.is_organic,
        product.is_ethical,
        product.ethical_label.as_str(),
        product.is_web_launch,
        product.sell_start_date.as_str(),
        product.is_completely_out_of_stock,
        product.is_temporary_out_of_stock,
        product.alcohol_percentage,
        product.volume,
        product.price,
        product.country.as_str(),
        product.origin_level1.as_str(),
        product.origin_level2.as_str(),
        product.vintage,
        product.sub_category.as_str(),
        product.a_type.as_str(),
        product.style.as_str(),
        product.assortment_text.as_str(),
        product.beverage_description_short.as_str(),
        product.usage_text.as_str(),
        product.taste.as_str(),
        product.assortment.as_str(),
        product.is_manufacturing_country,
        product.recycle_fee,
        product.is_regional_retricted,
        product.is_in_store_search_assortment.as_str(),
        product.is_news,
        product.apk,
        product.apk_recycling,
        product.link
        ])?;
    }
    transaction.commit().map_err(|e| -> rusqlite::Error {
        warn!("{}", e);
        e
    })?;
    info!("Committed transaction of {} products in {} seconds", products.len(),
          SystemTime::now().duration_since(start)?.as_secs());
    Ok(())
}

pub async fn select_all_products(opts:ProductOpts, con: Connection) -> Result<Vec<Product>> {
    let query = query_utils::QueryBuilder::build(opts);
    select_all(query.as_str(), con).await
}


pub async fn select_all(query: &str, con: Connection) -> Result<Vec<Product>> {
    let mut stmt = con.prepare(query)?;
    let source = stmt.query_map(NO_PARAMS, |row| {
        Ok(Product {
            product_id: row.get(0)?,
            product_number: row.get(1)?,
            product_name_bold: row.get(2)?,
            product_name_thin: row.get(3)?,
            category: row.get(4)?,
            product_number_short: row.get(5)?,
            producer_name: row.get(6)?,
            supplier_name: row.get(7)?,
            is_kosher: row.get(8)?,
            bottle_text_short: row.get(9)?,
            restricted_parcel_quantity: row.get(10)?,
            seal: row.get(11)?,
            is_organic: row.get(12)?,
            is_ethical: row.get(13)?,
            ethical_label: row.get(14)?,
            is_web_launch: row.get(15)?,
            sell_start_date: row.get(16)?,
            is_completely_out_of_stock: row.get(17)?,
            is_temporary_out_of_stock: row.get(18)?,
            alcohol_percentage: row.get(19)?,
            volume: row.get(20)?,
            price: row.get(21)?,
            country: row.get(22)?,
            origin_level1: row.get(23)?,
            origin_level2: row.get(24)?,
            vintage: row.get(25)?,
            sub_category: row.get(26)?,
            a_type: row.get(27)?,
            style: row.get(28)?,
            assortment_text: row.get(29)?,
            beverage_description_short: row.get(30)?,
            usage_text: row.get(31)?,
            taste: row.get(32)?,
            assortment: row.get(33)?,
            is_manufacturing_country: row.get(34)?,
            recycle_fee: row.get(35)?,
            is_regional_retricted: row.get(36)?,
            is_in_store_search_assortment: row.get(37)?,
            is_news: row.get(38)?,
            apk: row.get(39)?,
            apk_recycling: row.get(40)?,
            link: row.get(41)?,
        })
    })?;
    let mut unpacked = Vec::new();
    for prod in source {
        unpacked.push(prod?);
    }
    Ok(unpacked)
}

pub async fn insert_sites(sites: &Vec<Site>, mut con: Connection) -> Result<()> {
    let start = SystemTime::now();
    info!("Starting transaction to insert {} sites", sites.len());
    let transaction = con.transaction()?;
    transaction.execute_batch("DELETE FROM sites;")?;
    for site in sites {
        transaction.execute(
            "
              INSERT INTO sites (
              site_id,
              is_tasting_store,
              alias,
              address,
              display_name,
              postal_code,
              city,
              county,
              country,
              is_store,
              is_agent,
              is_active_for_agent_order,
              phone,
              email,
              services,
              depot,
              name)
        VALUES (
              ?1,
              ?2,
              ?3,
              ?4,
              ?5,
              ?5,
              ?7,
              ?8,
              ?9,
              ?10,
              ?11,
              ?12,
              ?13,
              ?14,
              ?15,
              ?16,
              ?17
        )", params![
           site.site_id.as_str(),
           site.is_tasting_store,
           site.alias.as_str(),
           site.address.as_str(),
           site.display_name.as_str(),
           site.postal_code.as_str(),
           site.city.as_str(),
           site.county.as_str(),
           site.country.as_str(),
           site.is_store,
           site.is_agent,
           site.is_active_for_agent_order,
           site.phone.as_str(),
           site.email.as_str(),
           site.services.as_str(),
           site.depot.as_str(),
           site.name.as_str(),
        ])?;
    }
    transaction.commit().map_err(|e| -> rusqlite::Error {
        warn!("{}", e);
        e
    })?;
    info!("Committed transaction of {} sites in {} seconds", sites.len(),
          SystemTime::now().duration_since(start)?.as_secs());
    Ok(())
}

pub async fn select_all_sites(con: Connection) -> Result<Vec<Site>> {
    let mut stmt = con.prepare("SELECT * FROM sites WHERE is_store=true")?;
    let source = stmt.query_map(NO_PARAMS, |row| {
        Ok(Site {
            site_id: row.get(0)?,
            is_tasting_store: row.get(1)?,
            alias: row.get(2)?,
            address: row.get(3)?,
            display_name: row.get(4)?,
            postal_code: row.get(5)?,
            city: row.get(6)?,
            county: row.get(7)?,
            country: row.get(8)?,
            is_store: row.get(9)?,
            is_agent: row.get(10)?,
            is_active_for_agent_order: row.get(11)?,
            phone: row.get(12)?,
            email: row.get(13)?,
            services: row.get(14)?,
            depot: row.get(15)?,
            name: row.get(16)?,
            opening_hours: Vec::new(),
            position: Position {lat: 0.0, long: 0.0}
        })
    })?;
    let mut unpacked = Vec::new();
    for site in source {
        unpacked.push(site?);
    }
    Ok(unpacked)
}

pub async fn insert_junctions(junctions: &Vec<MinimalSite>, mut con: Connection) -> Result<()> {
    let start = SystemTime::now();
    info!("Starting transaction to insert {} sites_products", junctions.len());
    let transaction = con.transaction()?;
    transaction.execute_batch("DELETE FROM sites_products;")?;
    for site in junctions {
        for prod in &site.products {
            let res = transaction.execute("
                INSERT INTO sites_products (product_key, site_key)
                VALUES (
                ?1,
                ?2
                )", params![
                    prod.product_id,
                    site.site_id
                    ]
            );
            if res.is_err() {
                debug!("Caught error inserting minimal_site={:?} {:?}", site, res.unwrap_err())
            }
        }

    }
    transaction.commit().map_err(|e| -> rusqlite::Error {
        warn!("{}", e);
        e
    })?;
    info!("Committed transaction of {} products in {} seconds", junctions.len(),
          SystemTime::now().duration_since(start)?.as_secs());
    Ok(())
}