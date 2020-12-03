use crate::domain::models::product::ProductOpts;

pub struct QueryBuilder {
    opts: ProductOpts,
}

impl QueryBuilder {
    fn to_query(&self) -> String {
        format!("{}{}{}{}{}", self.create_base(), self.include_category(), self.add_site(),
                self.include_recycling(), self.limit())
    }

    fn create_base(&self) -> String {
        return format!("SELECT * FROM products p \
            WHERE volume <= {}", self.opts.max_volume);
    }

    fn add_site(&self) -> String {
        if self.opts.site_id != "" {
            format!(" AND p.product_id IN (
                                       SELECT product_key FROM sites_products sp WHERE sp.site_key='{}'
                                       )", self.opts.site_id)
        } else if self.opts.exists_in_store {
            String::from(" AND EXISTS(
                            SELECT * FROM sites_products WHERE product_key = p.product_id
                       )")
        } else {
            String::new()
        }
    }

    fn include_category(&self) -> String {
        if self.opts.category != "" {
            format!(" AND p.category = '{}'", self.opts.category)
        } else {
            String::new()
        }
    }

    fn include_recycling(&self) -> String {
        if self.opts.include_recycling {
            String::from(" ORDER BY apk_recycling")
        } else {
            String::from(" ORDER BY apk")
        }
    }

    fn limit(&self) -> String {
        format!(" DESC LIMIT {};", self.opts.count)
    }

    pub fn build(opts: ProductOpts) -> String {
        let this = QueryBuilder{ opts };
        return this.to_query();
    }
}


