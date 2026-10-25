//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.ThreeWeirdos by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: @SuppressWarnings("unused") private static final ThreeWeirdos INSTANCE = new ThreeWeirdos();
pub static INSTANCE: compat::MainThread<compat::Shared<ThreeWeirdos>> = compat::MainThread::new(|| ThreeWeirdos::new());
/// java: protected static final Pattern PATTERN = Pattern.compile("^\\[NPC] ([A-Z][a-z]+): (?:The reward is(?: not in my chest!|n't in any of our chests\\.)|My chest (?:doesn't have the reward\\. We are all te
pub static PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[NPC] ([A-Z][a-z]+): (?:The reward is(?: not in my chest!|n't in any of our chests\\.)|My chest (?:doesn't have the reward\\. We are all telling the truth\\.|has the reward and I'm telling the truth!)|At least one of them is lying, and the reward is not in [A-Z][a-z]+'s chest!|Both of them are telling the truth\\. Also, [A-Z][a-z]+ has the reward in their chest!)$"));
/// java: private static final float[] GREEN_COLOR_COMPONENTS = new float[]{0, 1, 0};
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((0) as f32) as f32, ((1) as f32) as f32, ((0) as f32) as f32]);
/// java: private static @Nullable BlockPos pos;
pub static POS: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);
/// java: static @Nullable AABB boundingBox;
pub static BOUNDING_BOX: compat::MainThread<Option<AABB>> = compat::MainThread::new(|| None);

/// java: public class ThreeWeirdos extends DungeonPuzzle
#[derive(Clone)]
pub struct ThreeWeirdos {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
}

impl std::ops::Deref for ThreeWeirdos {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ThreeWeirdos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ThreeWeirdos {
    /// java: private ThreeWeirdos()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("three-weirdos".to_string(), vec!["three-chests".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {..}); - the listener holds this object
        let __this_allow_game = compat::RenderThread::new(__this.clone());
        compat::register_allow_game(move |message, overlay| {
            use mcrust::net::minecraft::network::chat::component::Component as _;
            let __this = __this_allow_game.get();
            // java: ClientLevel world = Minecraft.getInstance().level;
            let world = mcrust::mcrust::modding::game::level();
            // java: if (overlay || !shouldSolve() || !SkyblockerConfigManager.get().dungeons.puzzleSolvers.solveThreeWeirdos || world == null || !DungeonManager.isCurrentRoomMatched()) return true;
            let should_solve: bool = { let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) };
            if overlay || !should_solve || !crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_three_weirdos || world.is_none() || !crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::is_current_room_matched() {
                return true;
            }
            let world = world.unwrap();
            // java: Matcher matcher = PATTERN.matcher(ChatFormatting.stripFormatting(message.getString()));
            let mut matcher = PATTERN.matcher(&mcrust::net::minecraft::chat_formatting::strip_formatting(&message.get_string()));
            if !matcher.matches() {
                return true;
            }
            let name: String = matcher.group(1).unwrap_or_default();
            // java: Room room = DungeonManager.getCurrentRoom(); if (room == null || !room.isMatched()) return true;
            let Some(room) = crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::get_current_room() else {
                return true;
            };
            if !{ let __f = compat::dispatch(&room, |__t| __t.is_matched()); __f(&room) } {
                return true;
            }
            ThreeWeirdos::check_for_npc(__this, &world, &room, &BlockPos::new(13, 69, 24), &name);
            ThreeWeirdos::check_for_npc(__this, &world, &room, &BlockPos::new(15, 69, 25), &name);
            ThreeWeirdos::check_for_npc(__this, &world, &room, &BlockPos::new(17, 69, 24), &name);
            return true;
        });
        // java: UseBlockCallback.EVENT.register((_, _, _, blockHitResult) -> {
        //     if (blockHitResult.getType() == HitResult.Type.BLOCK && blockHitResult.getBlockPos().equals(pos)) pos = null;
        //     return InteractionResult.PASS; });
        mcrust::mcrust::modding::events::use_block_callback::register(|_, _, _, block_hit_result| {
            if block_hit_result.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Block && POS.get().as_ref() == Some(&block_hit_result.get_block_pos()) {
                *POS.get_mut() = None;
            }
            mcrust::net::minecraft::world::interaction_result::InteractionResult::Pass
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
    }

    /// java: private void checkForNPC(ClientLevel world, Room room, BlockPos relative, String name)
    pub fn check_for_npc(__this: &compat::Shared<Self>, world: &ClientLevel, room: &compat::Shared<dyn Room>, relative: &BlockPos, name: &str) {
        let mut npc_pos: BlockPos = ({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, relative) });
        // (the port's class filter has no ArmorStand.class: the base class Entity plus `instanceof ArmorStand`)
        use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;
        use mcrust::net::minecraft::network::chat::component::Component as _;
        let npcs: Vec<mcrust::net::minecraft::world::level::entity_getter::SharedEntity> = world.get_entities_of_class(
            mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::Entity,
            None,
            &AABB::encapsulating_full_blocks(&npc_pos, &npc_pos),
            &|entity| entity.as_any().is_some_and(|any| any.is::<ArmorStand>()) && compat::component_of(entity.get_name()).get_string() == name,
        );
        if !npcs.is_empty() {
            *POS.get_mut() = Some(({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &relative.offset(1, 0, 0)) }));
            *BOUNDING_BOX.get_mut() = RenderHelper::get_block_bounding_box(world, POS.get_mut().as_ref().unwrap());
            for entity in npcs.iter() {
                // the `EntityMixin` field (`SkyblockerEntity`, an attachment of the entity)
                let entity = mcrust::mcrust::modding::game::lock_entity(&**entity, "skyblocker ThreeWeirdos.checkForNPC");
                (&*entity as &dyn mcrust::net::minecraft::world::entity::entity::Entity).skyblocker_set_custom_name(&component::literal(name).with_style_format(ChatFormatting::Green));
            }
        }
    }

}

impl DungeonPuzzle for ThreeWeirdos {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for ThreeWeirdos {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        ThreeWeirdos::tick_dyn
    }

}

impl Renderable for ThreeWeirdos {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        ThreeWeirdos::extract_rendering_dyn
    }

}

impl Resettable for ThreeWeirdos {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        ThreeWeirdos::reset_dyn
    }

}

impl ThreeWeirdos {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
    }

    /// ThreeWeirdos.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if { let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) } && BOUNDING_BOX.get_mut().is_some() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, BOUNDING_BOX.get_mut().as_ref().unwrap(), &GREEN_COLOR_COMPONENTS, 0.5f32, false) });
        }
    }

    /// ThreeWeirdos.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        *POS.get_mut() = None;
    }

    /// ThreeWeirdos.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
