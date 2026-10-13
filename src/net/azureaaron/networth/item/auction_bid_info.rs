//! java: @ApiStatus.Internal public record AuctionBidInfo(OptionalLong winningBid, /* Shen's Specific*/ OptionalLong price, OptionalInt bid, OptionalInt auction)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<AuctionBidInfo> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<AuctionBidInfo>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(
            |o: &AuctionBidInfo| o.winning_bid,
            CodecUtils::optional_long(codec::optional_field_of_opt(
                codec::long_codec(),
                "winning_bid",
            )),
        ),
        record_codec_builder::of(
            |o: &AuctionBidInfo| o.price,
            CodecUtils::optional_long(codec::optional_field_of_opt(codec::long_codec(), "price")),
        ),
        record_codec_builder::of(
            |o: &AuctionBidInfo| o.bid,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "bid")),
        ),
        record_codec_builder::of(
            |o: &AuctionBidInfo| o.auction,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "auction")),
        ),
        AuctionBidInfo::new,
    ))
});

/// java: @ApiStatus.Internal public record AuctionBidInfo(OptionalLong winningBid, /* Shen's Specific*/ OptionalLong price, OptionalInt bid, OptionalInt auction)
#[derive(Clone, Debug, PartialEq)]
pub struct AuctionBidInfo {
    pub winning_bid: Option<i64>,
    pub price: Option<i64>,
    pub bid: Option<i32>,
    pub auction: Option<i32>,
}

impl AuctionBidInfo {
    /// java: the canonical record constructor
    pub fn new(
        winning_bid: Option<i64>,
        price: Option<i64>,
        bid: Option<i32>,
        auction: Option<i32>,
    ) -> Self {
        Self {
            winning_bid,
            price,
            bid,
            auction,
        }
    }

    /// java: the record accessor `winningBid()`
    pub fn winning_bid(&self) -> Option<i64> {
        self.winning_bid
    }

    /// java: the record accessor `price()`
    pub fn price(&self) -> Option<i64> {
        self.price
    }

    /// java: the record accessor `bid()`
    pub fn bid(&self) -> Option<i32> {
        self.bid
    }

    /// java: the record accessor `auction()`
    pub fn auction(&self) -> Option<i32> {
        self.auction
    }
}
