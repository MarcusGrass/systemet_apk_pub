use crate::domain::models::product::Product;

static DENS: f64 = 789 as f64;

pub fn get_apk(product: &Product) -> f64 {
    if product.volume != 0.0 && product.alcohol_percentage != 0.0 && product.price != 0.0 {
        return product.volume * product.alcohol_percentage * DENS / (product.price * 1000.0 * 100.0); // volume in ml, percent in absolute
    }
    return 0.0;
}

pub fn get_recyc_apk(product: &Product) -> f64 {
    if product.volume != 0.0 && product.alcohol_percentage != 0.0 && product.price != 0.0 {
        return product.volume * product.alcohol_percentage * DENS / ((product.price + product.recycle_fee) * 1000.0 * 100.0); // volume in ml, percent in absolute
    }
    return 0.0;
}