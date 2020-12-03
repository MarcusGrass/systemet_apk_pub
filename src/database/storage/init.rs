use rusqlite::{Connection, NO_PARAMS};
use crate::domain::result::Result;

pub async fn init_product_db(con: Connection) -> Result<()> {
    info!("Creating products table");
    con.execute("CREATE TABLE IF NOT EXISTS products (
                    product_id VARCHAR PRIMARY KEY,
                    product_number text,
                    product_name_bold text,
                    product_name_thin text,
                    category text,
                    product_number_short text,
                    producer_name text,
                    supplier_name text,
                    is_kosher BOOLEAN not null,
                    bottle_text_short text,
                    restricted_parcel_quantity int not null,
                    seal text,
                    is_organic BOOLEAN not null,
                    is_ethical BOOLEAN not null,
                    ethical_label text,
                    is_web_launch BOOLEAN not null,
                    sell_start_date text,
                    is_completely_out_of_stock BOOLEAN not null,
                    is_temporary_out_of_stock BOOLEAN not null,
                    alcohol_percentage REAL not null,
                    volume REAL not null,
                    price REAL not null,
                    country text,
                    origin_level1 text,
                    origin_level2 text,
                    vintage int not null,
                    sub_category text,
                    a_type text,
                    style text,
                    assortment_text text,
                    beverage_description_short text,
                    usage_text text,
                    taste text,
                    assortment text,
                    is_manufacturing_country BOOLEAN not null,
                    recycle_fee REAL not null,
                    is_regional_retricted BOOLEAN not null,
                    is_in_store_search_assortment text,
                    is_news BOOLEAN not null,
                    apk REAL not null,
                    apk_recycling REAL not null,
                    link text not null,
                    ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )", NO_PARAMS)?;
    info!("Products table created");
    Ok(())
}

pub async fn init_site_db(con: Connection) -> Result<()> {
    info!("Creating sites table");
    con.execute("CREATE TABLE IF NOT EXISTS sites (
            site_id VARCHAR PRIMARY KEY,
            is_tasting_store bool,
            alias text,
            address text,
            display_name text,
            postal_code text,
            city text,
            county text,
            country text,
            is_store bool,
            is_agent bool,
            is_active_for_agent_order bool,
            phone text,
            email text,
            services text,
            depot text,
            name text
        )", NO_PARAMS)?;
    info!("Sites table created");
    Ok(())
}


pub async fn init_junction_db(con: Connection) -> Result<()> {
    info!("Creating junction table");
    con.execute("CREATE TABLE IF NOT EXISTS sites_products (
                    product_key VARCHAR REFERENCES products(product_id) ON DELETE CASCADE,
                    site_key VARCHAR REFERENCES sites(site_id) ON DELETE CASCADE,
                    PRIMARY KEY (product_key, site_key)
        )", NO_PARAMS)?;
    info!("Sites table created");
    Ok(())
}