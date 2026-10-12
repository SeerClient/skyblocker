//! Ported from de.hysky.skyblocker.skyblock.museum.PriceData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;

/// java: public class PriceData
#[derive(Clone)]
pub struct PriceData {
    /// java: private final double lBinPrice;
    pub l_bin_price: f64,
    /// java: private final double craftCost;
    pub craft_cost: f64,
    /// java: private double effectivePrice;
    pub effective_price: f64,
}

impl PriceData {
    /// java: public PriceData(double lBinPrice, double craftCost)
    pub fn new(mut l_bin_price: f64, mut craft_cost: f64) -> compat::Shared<Self> {
        let __init_l_bin_price = l_bin_price;
        let __init_craft_cost = craft_cost;
        let __this = compat::shared(Self {
            l_bin_price: __init_l_bin_price,
            craft_cost: __init_craft_cost,
            effective_price: 0.0,
        });
        __this
    }

    /// java: public PriceData(Donation donation)
    pub fn new_1(mut donation: compat::Shared<Donation>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            l_bin_price: 0.0,
            craft_cost: 0.0,
            effective_price: 0.0,
        });
        if Donation::is_set(&donation) {
            let mut total_l_bin_price: f64 = ((0) as f64) as f64;
            let mut total_craft_cost: f64 = ((0) as f64) as f64;
            for piece in Donation::get_set(&donation) {
                let mut l_bin_price: f64 = ItemUtils::get_item_price_string(Some(piece.0.clone())).unwrap_or(((0) as f64) as f64);
                let mut craft_cost: f64 = ItemUtils::get_craft_cost(&piece.0);
                total_l_bin_price = (total_l_bin_price + l_bin_price);
                total_craft_cost = (total_craft_cost + craft_cost);
                piece.1;
            }
            { let __v = total_l_bin_price; compat::write(&__this, |__t| __t.l_bin_price = __v); }
            { let __v = total_craft_cost; compat::write(&__this, |__t| __t.craft_cost = __v); }
        } else {
            { let __v = ItemUtils::get_item_price_string(Some(Donation::get_id(&donation))).unwrap_or(((0) as f64) as f64); compat::write(&__this, |__t| __t.l_bin_price = __v); }
            { let __v = ItemUtils::get_craft_cost(&Donation::get_id(&donation)); compat::write(&__this, |__t| __t.craft_cost = __v); }
        }
        __this
    }

    /// java: public double getLBinPrice()
    pub fn get_l_bin_price(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.l_bin_price);
    }

    /// java: public double getCraftCost()
    pub fn get_craft_cost(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.craft_cost);
    }

    /// java: public double getEffectivePrice()
    pub fn get_effective_price(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.effective_price);
    }

    /// java: public void setEffectivePrice(double effectivePrice)
    pub fn set_effective_price(__this: &compat::Shared<Self>, mut effective_price: f64) {
        { let __v = effective_price; compat::write(__this, |__t| __t.effective_price = __v); }
    }

}
