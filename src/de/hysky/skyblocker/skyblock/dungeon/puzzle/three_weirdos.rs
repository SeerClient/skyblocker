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
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			ClientLevel world = Minecraft.getInstance().level;
        //   java: 			if (overlay || !shouldSolve() || !SkyblockerConfigManager.get().dungeons.puzzleSolvers.solveThreeWeirdos || world == null || !DungeonManager.isCurrentRoomMatched()) return true;
        //   java:
        //   java: 			Matcher matcher = PATTERN.matcher(ChatFormatting.stripFormatting(message.getString()));
        //   java: 			if (!matcher.matches()) return true;
        //   java: 			String name = matcher.group(1);
        //   java: 			Room room = DungeonManager.getCurrentRoom();
        //   java: 			if (room == null || !room.isMatched()) return true;
        //   java:
        //   java: 			checkForNPC(world, room, new BlockPos(13, 69, 24), name);
        //   java: 			checkForNPC(world, room, new BlockPos(15, 69, 25), name);
        //   java: 			checkForNPC(world, room, new BlockPos(17, 69, 24), name);
        //   java:
        //   java: 			return true;
        //   java: 		});
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register((_, _, _, blockHitResult) -> {
        //   java: 			if (blockHitResult.getType() == HitResult.Type.BLOCK && blockHitResult.getBlockPos().equals(pos)) {
        //   java: 				pos = null;
        //   java: 			}
        //   java: 			return InteractionResult.PASS;
        //   java: 		});
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
    }

    /// java: private void checkForNPC(ClientLevel world, Room room, BlockPos relative, String name)
    pub fn check_for_npc(__this: &compat::Shared<Self>, world: &ClientLevel, room: &compat::Shared<dyn Room>, relative: &BlockPos, name: &str) {
        let mut npc_pos: BlockPos = ({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, relative) });
        // TODO(fabric2seer): `.getName(..)` on a T
        //   java: List<ArmorStand> npcs = world.getEntitiesOfClass(
        //   java: 				ArmorStand.class,
        //   java: 				AABB.encapsulatingFullBlocks(npcPos, npcPos),
        //   java: 				entity -> entity.getName().getString().equals(name)
        //   java: 		);
        let mut npcs: Vec<ArmorStand> = todo!("fabric2seer: the initialiser of `npcs` (TODO above)");
        if !npcs.is_empty() {
            *POS.get_mut() = Some(({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &relative.offset(1, 0, 0)) }));
            *BOUNDING_BOX.get_mut() = RenderHelper::get_block_bounding_box(world, POS.get_mut().as_ref().unwrap());
            for entity in npcs.iter() {
                // TODO(fabric2seer): `ArmorStand.skyblocker$setCustomName` has no Rust method in the port (looked for `skyblocker_set_custom_name` in ArmorStand and JavaSrc)
                // rust: entity.skyblocker_set_custom_name(&component::literal(name).with_style_format(ChatFormatting::Green));
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
