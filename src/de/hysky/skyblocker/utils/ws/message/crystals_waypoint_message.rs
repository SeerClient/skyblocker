//! Ported from de.hysky.skyblocker.utils.ws.message.CrystalsWaypointMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::crystals_locations_manager::CrystalsLocationsManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::CrystalHollowsLocationsCategory;
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

/// java: private static final Codec<CrystalsWaypointMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( CrystalHollowsLocationsCategory.CODEC.fieldOf("name").forGetter(CrystalsWaypointMessag
pub static CODEC: compat::MainThread<CodecRef<CrystalsWaypointMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &CrystalsWaypointMessage| o.location(), codec::field_of(mining_location_label::CRYSTAL_HOLLOWS_LOCATIONS_CATEGORY_CODEC.get_mut().clone(), "name")), record_codec_builder::of(|o: &CrystalsWaypointMessage| o.coordinates(), codec::field_of(mcrust::net::minecraft::core::block_pos::codec(), "coordinates")), move |__a0, __a1| CrystalsWaypointMessage::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<List<CrystalsWaypointMessage>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: compat::MainThread<CodecRef<Vec<CrystalsWaypointMessage>>> = compat::MainThread::new(|| codec::list(CODEC.get_mut().clone()));

/// java: public record CrystalsWaypointMessage(CrystalHollowsLocationsCategory location, BlockPos coordinates) implements Message<CrystalsWaypointMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct CrystalsWaypointMessage {
    /// java: CrystalHollowsLocationsCategory location
    pub location: CrystalHollowsLocationsCategory,
    /// java: BlockPos coordinates
    pub coordinates: BlockPos,
}

impl CrystalsWaypointMessage {
    /// java: the canonical record constructor
    pub fn new(mut location: CrystalHollowsLocationsCategory, mut coordinates: BlockPos) -> Self {
        Self { location, coordinates }
    }

    /// java: the record accessor `location()`
    pub fn location(&self) -> CrystalHollowsLocationsCategory {
        self.location
    }

    /// java: the record accessor `coordinates()`
    pub fn coordinates(&self) -> BlockPos {
        self.coordinates.clone()
    }

    /// java: public static void handle(Type type, Optional<Dynamic<?>> message)
    pub fn handle(mut r#type: Type, message: &Option<Dynamic>) {
        match r#type {
            Type::Response => {
                if message.is_none() {
                    return;
                }
                let mut waypoint: CrystalsWaypointMessage = CODEC.get_mut().parse_dynamic(&message.clone().unwrap()).get_or_throw().unwrap();
                RenderHelper::run_on_render_thread(Arc::new(move || { CrystalsLocationsManager::add_custom_waypoint_from_socket(&vec![waypoint.clone()]); }));
            }
            Type::InitialMessage => {
                if message.is_none() {
                    return;
                }
                let mut waypoints: Vec<CrystalsWaypointMessage> = LIST_CODEC.get_mut().parse_dynamic(&message.clone().unwrap()).get_partial_or_throw().unwrap();
                if waypoints.is_empty() {
                    return;
                }
                RenderHelper::run_on_render_thread(Arc::new(move || { CrystalsLocationsManager::add_custom_waypoint_from_socket(&waypoints.clone()); }));
            }
            _ => {
            }
        }
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::crystals_waypoint_message::CrystalsWaypointMessage> for CrystalsWaypointMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<CrystalsWaypointMessage> getCodec()
    fn get_codec(&self) -> CodecRef<CrystalsWaypointMessage> {
        return CODEC.get_mut().clone();
    }

}
