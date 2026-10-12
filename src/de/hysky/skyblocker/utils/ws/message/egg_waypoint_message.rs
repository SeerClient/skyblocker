//! Ported from de.hysky.skyblocker.utils.ws.message.EggWaypointMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::egg_finder;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::egg_finder::EggFinder;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::egg_finder::EggType;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::r#type::Type;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final Codec<EggWaypointMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( EggFinder.EggType.CODEC.fieldOf("eggType").forGetter(EggWaypointMessage::eggType), BlockPos
pub static CODEC: compat::MainThread<CodecRef<EggWaypointMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &EggWaypointMessage| o.egg_type(), codec::field_of(egg_finder::EGG_TYPE_CODEC.get_mut().clone(), "eggType")), record_codec_builder::of(|o: &EggWaypointMessage| o.coordinates(), codec::field_of(mcrust::net::minecraft::core::block_pos::codec(), "coordinates")), record_codec_builder::of(|o: &EggWaypointMessage| o.expiration_epoch(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::long_codec(), "expirationEpoch")), move |__a0, __a1, __a2| EggWaypointMessage::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: private static final Codec<List<EggWaypointMessage>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: compat::MainThread<CodecRef<Vec<EggWaypointMessage>>> = compat::MainThread::new(|| codec::list(CODEC.get_mut().clone()));

/// java: public record EggWaypointMessage(EggFinder.EggType eggType, BlockPos coordinates, Optional<Long> expirationEpoch) implements Message<EggWaypointMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct EggWaypointMessage {
    /// java: EggFinder.EggType eggType
    pub egg_type: EggType,
    /// java: BlockPos coordinates
    pub coordinates: BlockPos,
    /// java: Optional<Long> expirationEpoch
    pub expiration_epoch: Option<i64>,
}

impl EggWaypointMessage {
    /// java: the canonical record constructor
    pub fn new(mut egg_type: EggType, mut coordinates: BlockPos, mut expiration_epoch: Option<i64>) -> Self {
        Self { egg_type, coordinates, expiration_epoch }
    }

    /// java: the record accessor `eggType()`
    pub fn egg_type(&self) -> EggType {
        self.egg_type
    }

    /// java: the record accessor `coordinates()`
    pub fn coordinates(&self) -> BlockPos {
        self.coordinates.clone()
    }

    /// java: the record accessor `expirationEpoch()`
    pub fn expiration_epoch(&self) -> Option<i64> {
        self.expiration_epoch.clone()
    }

    /// java: public static void handle(Type type, Optional<Dynamic<?>> message)
    pub fn handle(mut r#type: Type, message: &Option<Dynamic>) {
        match r#type {
            Type::Response => {
                if message.is_none() {
                    return;
                }
                let mut waypoint: EggWaypointMessage = CODEC.get_mut().parse_dynamic(&message.clone().unwrap()).get_or_throw().unwrap();
                RenderHelper::run_on_render_thread(Arc::new(move || { EggFinder::on_websocket_message(&waypoint); }));
            }
            Type::InitialMessage => {
                if message.is_none() {
                    return;
                }
                let mut waypoints: Vec<EggWaypointMessage> = LIST_CODEC.get_mut().parse_dynamic(&message.clone().unwrap()).get_or_throw().unwrap();
                let mut now: i64 = compat::millis();
                RenderHelper::run_on_render_thread(Arc::new(move || { for __a0 in waypoints.iter().cloned().filter(|w| (w.expiration_epoch.is_some() && w.expiration_epoch().unwrap() > now)) { EggFinder::on_websocket_message(&__a0); } }));
            }
            _ => {
            }
        }
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::egg_waypoint_message::EggWaypointMessage> for EggWaypointMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<EggWaypointMessage> getCodec()
    fn get_codec(&self) -> CodecRef<EggWaypointMessage> {
        return CODEC.get_mut().clone();
    }

}
