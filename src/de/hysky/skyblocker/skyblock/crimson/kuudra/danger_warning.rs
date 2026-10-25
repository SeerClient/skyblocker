//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.DangerWarning by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: private static final Supplier<MutableComponent> DANGER_TEXT = () -> Component.translatable("skyblocker.crimson.kuudra.danger");
pub static DANGER_TEXT: compat::MainThread<Arc<dyn Fn() -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move || component::translatable("skyblocker.crimson.kuudra.danger")));
/// java: private static final Title TITLE = new Title(DANGER_TEXT.get());
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new_1((DANGER_TEXT.get_mut())()));

/// java: public class DangerWarning
#[derive(Clone)]
pub struct DangerWarning {}

impl DangerWarning {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DangerWarning::update_indicator(); }), 5) });
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DangerWarning::reset();
        }));
    }

    /// java: private static void updateIndicator()
    pub fn update_indicator() {
        if Utils::is_in_kuudra() && compat::read(&SkyblockerConfigManager::get().crimson_isle.kuudra, |__t| __t.danger_warning) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().is_some() {
            for i in 1..=5 {
                let mut under: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().below_n(i)).unwrap();
                let mut title: Option<compat::Shared<Title>> = DangerWarning::get_danger_title(&under);
                if let Some(title) = &title {
                    TitleContainer::add_title_and_play_sound(title);
                    return;
                } else {
                    if i == 5 {
                        TitleContainer::remove_title(TITLE.get_mut());
                    }
                }
            }
        }
    }

    /// java: private static Title getDangerTitle(BlockPos pos)
    pub fn get_danger_title(pos: &BlockPos) -> Option<compat::Shared<Title>> {
        let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(pos);
        let mut block: Arc<Block> = state.get_block();
        //   java: int argb = switch (block) { case Block b when b == Blocks.DYED_TERRACOTTA.green() -> DyeColor.GREEN.getTextureDiffuseColor();
        //     .. lime, yellow, orange, pink, red ..; default -> 0; };
        // (each case compares the block by identity)
        use mcrust::net::minecraft::world::item::dye_color::DyeColor;
        use mcrust::net::minecraft::world::level::block::blocks;
        let terracotta = blocks::dyed_terracotta();
        let mut argb: i32 = if Arc::ptr_eq(&block, terracotta.green()) {
            DyeColor::Green.get_texture_diffuse_color()
        } else if Arc::ptr_eq(&block, terracotta.lime()) {
            DyeColor::Lime.get_texture_diffuse_color()
        } else if Arc::ptr_eq(&block, terracotta.yellow()) {
            DyeColor::Yellow.get_texture_diffuse_color()
        } else if Arc::ptr_eq(&block, terracotta.orange()) {
            DyeColor::Orange.get_texture_diffuse_color()
        } else if Arc::ptr_eq(&block, terracotta.pink()) {
            DyeColor::Pink.get_texture_diffuse_color()
        } else if Arc::ptr_eq(&block, terracotta.red()) {
            DyeColor::Red.get_texture_diffuse_color()
        } else {
            0
        };
        return (if argb != 0 { Some(Title::set_text(TITLE.get_mut(), (DANGER_TEXT.get_mut())().with_color((argb & 0x00FFFFFF)))) } else { None });
    }

    /// java: private static void reset()
    pub fn reset() {
        TitleContainer::remove_title(TITLE.get_mut());
    }

}
