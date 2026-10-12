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
