//! Ported from io.github.moulberry.repo.constants.BazaarStocks (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Value public final class BazaarStocks
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BazaarStocks {
    /// java: Map<@NEUId String, String> stocks;
    stocks: HashMap<String, String>,
}

/// java: @Getter public static class InternalRepresentation
#[derive(Clone, Debug, Default)]
pub struct InternalRepresentation {
    /// java: @NEUId String id;
    pub(crate) id: Option<String>,
    /// java: String stock;
    pub(crate) stock: Option<String>,
}

impl InternalRepresentation {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        InternalRepresentation { id: gson::string(json, "id"), stock: gson::string(json, "stock") }
    }

    /// java: public String getId()
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// java: public String getStock()
    pub fn get_stock(&self) -> Option<&str> {
        self.stock.as_deref()
    }
}

impl BazaarStocks {
    /// java: public BazaarStocks(final Map<@NEUId String, String> stocks)
    pub fn new(stocks: HashMap<String, String>) -> Self {
        BazaarStocks { stocks }
    }

    /// java: public @NotNull String getBazaarStockOrDefault(@NotNull @NEUId String id) { return stocks.getOrDefault(id, id); }
    pub fn get_bazaar_stock_or_default(&self, id: &str) -> String {
        self.stocks.get(id).cloned().unwrap_or_else(|| id.to_string())
    }

    /// java: public Map<@NEUId String, String> getStocks()
    pub fn get_stocks(&self) -> &HashMap<String, String> {
        &self.stocks
    }
}

impl std::fmt::Display for BazaarStocks {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BazaarStocks(stocks={:?})", self.stocks)
    }
}
