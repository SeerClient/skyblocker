//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.CoopBanking by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class CoopBanking
#[derive(Clone)]
pub struct CoopBanking {
    /// java: public double balance;
    pub balance: f64,
    /// java: public List<Transaction> transactions = List.of();
    pub transactions: Vec<Transaction>,
}

impl CoopBanking {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            balance: 0.0,
            transactions: vec![],
        };
        this
    }

}

/// java: public static class Transaction
#[derive(Clone)]
pub struct Transaction {
    /// java: public double amount;
    pub amount: f64,
    /// java: public long timestamp;
    pub timestamp: i64,
    /// java: public BankAction action;
    pub action: Option<BankAction>,
    /// java: @SerializedName("initiator_name") public String initiatorName;
    pub initiator_name: String,
}

impl Transaction {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            amount: 0.0,
            timestamp: 0,
            action: None,
            initiator_name: Default::default(),
        };
        this
    }

}

/// java: public enum BankAction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BankAction {
    Deposit,
    Withdraw,
}

impl BankAction {
    pub const VALUES: [BankAction; 2] = [BankAction::Deposit, BankAction::Withdraw];

    /// java: values()
    pub fn values() -> Vec<BankAction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            BankAction::Deposit => "DEPOSIT",
            BankAction::Withdraw => "WITHDRAW",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> BankAction {
        match name {
            "DEPOSIT" => BankAction::Deposit,
            "WITHDRAW" => BankAction::Withdraw,
            _ => panic!("IllegalArgumentException: No enum constant BankAction.{name}"),
        }
    }

}

impl std::fmt::Display for BankAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl CoopBanking {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "balance") {
            if let Some(__x) = gson::as_f64(__v) {
                self.balance = __x;
            }
        }
        if let Some(__v) = gson::member(json, "transactions") {
            if let Some(__x) = gson::list(__v, |__e0| Some(Transaction::from_json(__e0))) {
                self.transactions = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CoopBanking.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Transaction {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "amount") {
            if let Some(__x) = gson::as_f64(__v) {
                self.amount = __x;
            }
        }
        if let Some(__v) = gson::member(json, "timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.timestamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "action") {
            if let Some(__x) = gson::as_string(__v).and_then(|__s| BankAction::values().into_iter().find(|__a| __a.name() == __s)).map(Some) {
                self.action = __x;
            }
        }
        if let Some(__v) = gson::member(json, "initiator_name") {
            if let Some(__x) = gson::as_string(__v) {
                self.initiator_name = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Transaction.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
