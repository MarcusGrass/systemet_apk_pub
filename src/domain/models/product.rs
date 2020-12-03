use serde::{Serialize, Deserialize};
use serde::export::Formatter;
use super::serialization_helpers::nullable_string;
use regex::Regex;
use unidecode;

lazy_static! {
    static ref RE: Regex = Regex::new("[^A-Za-z0-9 ]").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename="ProductId", deserialize_with = "nullable_string")]
    pub product_id: String,
    #[serde(rename="ProductNumber", deserialize_with="nullable_string")]
    pub product_number: String,
    #[serde(rename="ProductNameBold", deserialize_with="nullable_string")]
    pub product_name_bold: String,
    #[serde(rename="ProductNameThin", deserialize_with="nullable_string")]
    pub product_name_thin: String,
    #[serde(rename="Category", deserialize_with="nullable_string")]
    pub category: String,
    #[serde(rename="ProductNumberShort", deserialize_with="nullable_string")]
    pub product_number_short: String,
    #[serde(rename="ProducerName", deserialize_with="nullable_string")]
    pub producer_name: String,
    #[serde(rename="SupplierName", deserialize_with="nullable_string")]
    pub supplier_name: String,
    #[serde(rename="IsKosher")]
    pub is_kosher: bool,
    #[serde(rename="BottleTextShort", deserialize_with="nullable_string")]
    pub bottle_text_short: String,
    #[serde(rename="RestrictedParcelQuantity")]
    pub restricted_parcel_quantity: i32,
    #[serde(rename="Seal", deserialize_with="nullable_string")]
    pub seal: String,
    #[serde(rename="IsOrganic")]
    pub is_organic: bool,
    #[serde(rename="IsEthical")]
    pub is_ethical: bool,
    #[serde(rename="EthicalLabel", deserialize_with="nullable_string")]
    pub ethical_label: String,
    #[serde(rename="IsWebLaunch")]
    pub is_web_launch: bool,
    #[serde(rename="SellStartDate", deserialize_with="nullable_string")]
    pub sell_start_date: String,
    #[serde(rename="IsCompletelyOutOfStock")]
    pub is_completely_out_of_stock: bool,
    #[serde(rename="IsTemporaryOutOfStock")]
    pub is_temporary_out_of_stock: bool,
    #[serde(rename="AlcoholPercentage")]
    pub alcohol_percentage: f64,
    #[serde(rename="Volume")]
    pub volume: f64,
    #[serde(rename="Price")]
    pub price: f64,
    #[serde(rename="Country", deserialize_with="nullable_string")]
    pub country: String,
    #[serde(rename="OriginLevel1", deserialize_with="nullable_string")]
    pub origin_level1: String,
    #[serde(rename="OriginLevel2", deserialize_with="nullable_string")]
    pub origin_level2: String,
    #[serde(rename="Vintage")]
    pub vintage: i32,
    #[serde(rename="SubCategory", deserialize_with="nullable_string")]
    pub sub_category: String,
    #[serde(rename="Type", deserialize_with="nullable_string")]
    pub a_type: String,
    #[serde(rename="Style", deserialize_with="nullable_string")]
    pub style: String,
    #[serde(rename="AssortmentText", deserialize_with="nullable_string")]
    pub assortment_text: String,
    #[serde(rename="BeverageDescriptionShort", deserialize_with="nullable_string")]
    pub beverage_description_short: String,
    #[serde(rename="Usage", deserialize_with="nullable_string")]
    pub usage_text: String,
    #[serde(rename="Taste", deserialize_with="nullable_string")]
    pub taste: String,
    #[serde(rename="Assortment", deserialize_with="nullable_string")]
    pub assortment: String,
    #[serde(rename="IsManufacturingCountry")]
    pub is_manufacturing_country: bool,
    #[serde(rename="RecycleFee")]
    pub recycle_fee: f64,
    #[serde(rename="IsRegionalRestricted")]
    pub is_regional_retricted: bool,
    #[serde(rename="IsInStoreSearchAssortment", deserialize_with="nullable_string")]
    pub is_in_store_search_assortment: String,
    #[serde(rename="IsNews")]
    pub is_news: bool,

    #[serde(rename="Apk")]
    #[serde(default)]
    pub apk: f64,

    #[serde(rename="ApkRecycling")]
    #[serde(default)]
    pub apk_recycling: f64,

    #[serde(rename="Link")]
    #[serde(default)]
    pub link: String,
}

impl Product {
    pub fn construct_link(&self) -> String {
        let cat_no_se_signs = remove_swe_signs_and_replace_spaces(&self.category);
        let name_no_space = remove_swe_signs_and_replace_spaces(&self.product_name_bold);
        format!("https://systembolaget.se/dryck/{}/{}-{}", cat_no_se_signs,
                name_no_space, self.product_number)
    }
}

fn remove_swe_signs_and_replace_spaces(source: &str) -> String {
    let signs = unidecode::unidecode(&source.to_lowercase());
    return RE.replace_all(&signs, "")
        .replace(" ", "-");
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "Product(product_id={}, product_number={}, product_name_bold={}, product_name_thin={}, category={}, \
        alcohol_percentage={}, volume={}, price={}, apk={}, apk_recycling={})",
        self.product_id.as_str(), self.product_number, self.product_name_bold.as_str(),
        self.product_name_thin.as_str(), self.category.as_str(),
        self.alcohol_percentage, self.volume, self.price, self.apk, self.apk_recycling)
    }
}

#[derive(Deserialize)]
pub struct ProductOpts {
    pub count: usize,

    #[serde(default)]
    pub include_recycling: bool,

    #[serde(default)]
    pub exists_in_store: bool,
    pub max_volume: f64,

    #[serde(default)]
    pub site_id: String,

    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MinimalSite {

    #[serde(rename="SiteId")]
    pub site_id: String,

    #[serde(rename="Products")]
    pub products: Vec<MinimalProduct>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MinimalProduct {

    #[serde(rename="ProductId")]
    pub product_id: String,
    #[serde(rename="ProductNumber")]
    pub product_number: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SiteResponse {

    #[serde(rename="SiteName")]
    pub site_name: String,
    #[serde(rename="SiteId")]
    pub site_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_str() {
        let src = "Sju komma två'an roséviner";
        let expect = "sju-komma-tvaan-roseviner";
        assert_eq!(expect, remove_swe_signs_and_replace_spaces(src));
    }

}
