//! java: public interface ItemMetadataRetriever (net.azureaaron.networth.item)
//!
//! Intermediary to retrieve more complex to access item metadata such as a New Year Cake Bag's stored cake years.

/// java: public interface ItemMetadataRetriever
pub trait ItemMetadataRetriever {
    /// java: IntList cakeBagCakeYears();
    fn cake_bag_cake_years(&self) -> Vec<i32>;
}
