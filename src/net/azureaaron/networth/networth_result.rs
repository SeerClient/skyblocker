//! java: public record NetworthResult(double price, double base, List<Calculation> calculations) (net.azureaaron.networth)

use super::calculation::Calculation;

/// java: public record NetworthResult(double price, double base, List<Calculation> calculations)
#[derive(Clone, Debug, PartialEq)]
pub struct NetworthResult {
    pub price: f64,
    pub base: f64,
    pub calculations: Vec<Calculation>,
}

impl NetworthResult {
    /// java: static final NetworthResult EMPTY = new NetworthResult(0, 0, List.of());
    pub fn empty() -> Self {
        Self::new(0.0, 0.0, Vec::new())
    }

    /// java: @ApiStatus.Internal public NetworthResult(double price, double base, List<Calculation> calculations)
    pub fn new(price: f64, base: f64, calculations: Vec<Calculation>) -> Self {
        Self {
            price,
            base,
            calculations,
        }
    }

    /// java: static NetworthResult of(double price, double base, List<Calculation> calculations)
    pub fn of(price: f64, base: f64, calculations: Vec<Calculation>) -> Self {
        Self::new(price, base, calculations)
    }

    /// java: the record accessor `price()`
    pub fn price(&self) -> f64 {
        self.price
    }

    /// java: the record accessor `base()`
    pub fn base(&self) -> f64 {
        self.base
    }

    /// java: the record accessor `calculations()`
    pub fn calculations(&self) -> &Vec<Calculation> {
        &self.calculations
    }
}
