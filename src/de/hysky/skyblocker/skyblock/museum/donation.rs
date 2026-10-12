//! Ported from de.hysky.skyblocker.skyblock.museum.Donation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::museum::price_data::PriceData;

/// java: public class Donation
#[derive(Clone)]
pub struct Donation {
    /// java: private final String category;
    pub category: String,
    /// java: private final String id;
    pub id: String,
    /// java: private final List<ObjectObjectMutablePair<String, PriceData>> set;
    pub set: Vec<(String, compat::Shared<PriceData>)>,
    /// java: private final List<String> downgrades = new ArrayList<>();
    pub downgrades: Vec<String>,
    /// java: private final int xp;
    pub xp: i32,
    /// java: private List<ObjectIntPair<String>> countsTowards;
    pub counts_towards: Vec<(String, i32)>,
    /// java: private PriceData priceData;
    pub price_data: Option<compat::Shared<PriceData>>,
    /// java: private @Nullable ObjectDoublePair<String> discount;
    pub discount: Option<(String, f64)>,
    /// java: private int totalXp;
    pub total_xp: i32,
    /// java: private double xpCoinsRatio;
    pub xp_coins_ratio: f64,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl Donation {
    /// java: public Donation(String category, String id, List<ObjectObjectMutablePair<String, PriceData>> set, int xp)
    pub fn new(mut category: String, mut id: String, mut set: Vec<(String, compat::Shared<PriceData>)>, mut xp: i32) -> compat::Shared<Self> {
        let __init_category = category.clone();
        let __init_id = id.clone();
        let __init_set = set.clone();
        let __init_xp = xp;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            category: __init_category,
            id: __init_id,
            set: __init_set,
            downgrades: Vec::new(),
            xp: __init_xp,
            counts_towards: Default::default(),
            price_data: None,
            discount: None,
            total_xp: 0,
            xp_coins_ratio: 0.0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public int getTotalXp()
    pub fn get_total_xp(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.total_xp);
    }

    /// java: public void setTotalXp(int totalXp)
    pub fn set_total_xp(__this: &compat::Shared<Self>, mut total_xp: i32) {
        { let __v = total_xp; compat::write(__this, |__t| __t.total_xp = __v); }
    }

    /// java: public List<ObjectIntPair<String>> getCountsTowards()
    pub fn get_counts_towards(__this: &compat::Shared<Self>) -> Vec<(String, i32)> {
        return compat::read(__this, |__t| __t.counts_towards.clone());
    }

    /// java: public void setCountsTowards(List<ObjectIntPair<String>> countsTowards)
    pub fn set_counts_towards(__this: &compat::Shared<Self>, mut counts_towards: Vec<(String, i32)>) {
        { let __v = counts_towards.clone(); compat::write(__this, |__t| __t.counts_towards = __v); }
    }

    /// java: public PriceData getPriceData()
    pub fn get_price_data(__this: &compat::Shared<Self>) -> compat::Shared<PriceData> {
        return compat::read(__this, |__t| __t.price_data.clone()).unwrap();
    }

    /// java: public void setPriceData()
    pub fn set_price_data(__this: &compat::Shared<Self>) {
        { let __v = Some(PriceData::new_1(__this.clone())); compat::write(__this, |__t| __t.price_data = __v); }
    }

    /// java: public @Nullable ObjectDoublePair<String> getDiscount()
    pub fn get_discount(__this: &compat::Shared<Self>) -> Option<(String, f64)> {
        return compat::read(__this, |__t| __t.discount.clone());
    }

    /// java: public void setDiscount(@Nullable ObjectDoublePair<String> discount)
    pub fn set_discount(__this: &compat::Shared<Self>, mut discount: Option<(String, f64)>) {
        { let __v = discount.clone(); compat::write(__this, |__t| __t.discount = __v); }
    }

    /// java: public boolean hasDiscount()
    pub fn has_discount(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(__this, |__t| __t.discount.clone()).is_some() && compat::read(__this, |__t| __t.discount.clone()).1 > 0.0);
    }

    /// java: public void addDowngrade(String downgrade)
    pub fn add_downgrade(__this: &compat::Shared<Self>, mut downgrade: String) {
        let __h0 = downgrade.clone();
        __this.lock().borrow_mut().downgrades.push(__h0);
    }

    /// java: public List<String> getDowngrades()
    pub fn get_downgrades(__this: &compat::Shared<Self>) -> Vec<String> {
        return compat::read(__this, |__t| __t.downgrades.clone());
    }

    /// java: public double getXpCoinsRatio()
    pub fn get_xp_coins_ratio(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.xp_coins_ratio);
    }

    /// java: public void setXpCoinsRatio(double xpCoinsRatio)
    pub fn set_xp_coins_ratio(__this: &compat::Shared<Self>, mut xp_coins_ratio: f64) {
        { let __v = xp_coins_ratio; compat::write(__this, |__t| __t.xp_coins_ratio = __v); }
    }

    /// java: public String getCategory()
    pub fn get_category(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.category.clone());
    }

    /// java: public String getId()
    pub fn get_id(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.id.clone());
    }

    /// java: public boolean isSet()
    pub fn is_set(__this: &compat::Shared<Self>) -> bool {
        return !(__this.lock().borrow().set.is_empty());
    }

    /// java: public List<ObjectObjectMutablePair<String, PriceData>> getSet()
    pub fn get_set(__this: &compat::Shared<Self>) -> Vec<(String, compat::Shared<PriceData>)> {
        return compat::read(__this, |__t| __t.set.clone());
    }

    /// java: public int getXp()
    pub fn get_xp(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.xp);
    }

    /// java: public boolean isCraftable()
    pub fn is_craftable(__this: &compat::Shared<Self>) -> bool {
        return (PriceData::get_craft_cost(&compat::read(__this, |__t| __t.price_data.clone()).unwrap()) > (0 as f64));
    }

    /// java: public boolean hasLBinPrice()
    pub fn has_l_bin_price(__this: &compat::Shared<Self>) -> bool {
        return (PriceData::get_l_bin_price(&compat::read(__this, |__t| __t.price_data.clone()).unwrap()) > (0 as f64));
    }

    /// java: public boolean hasPrice()
    pub fn has_price(__this: &compat::Shared<Self>) -> bool {
        return (PriceData::get_effective_price(&compat::read(__this, |__t| __t.price_data.clone()).unwrap()) > (0 as f64));
    }

}
