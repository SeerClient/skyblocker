//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.tree.HotmTreeBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::primitive_type_utils::PrimitiveTypeUtils;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::AbstractLayout;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::Hotx;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::Node;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::Perk;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::PowderInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::SkillTreeBuilder;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::SkillTreeBuilderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::sky_block_tooltip_styles;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public static final HotmTreeBuilder INSTANCE = new HotmTreeBuilder();
pub static INSTANCE: compat::MainThread<compat::Shared<HotmTreeBuilder>> = compat::MainThread::new(|| HotmTreeBuilder::new());
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<Layout> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.listOf().fieldOf("prelude").forGetter(Layout::prelude),
//   java: 				Hotx.CODEC.fieldOf("hotm").forGetter(Layout::hotm)
//   java: 				).apply(instance, Layout::new));
/// java: private static final Codec<Layout> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.listOf().fieldOf("prelude").forGetter(Layout::prelude), Hotx.CODEC.fieldOf("hotm").forGett
pub static LAYOUT_CODEC: compat::MainThread<CodecRef<Layout>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public final class HotmTreeBuilder extends SkillTreeBuilder
pub struct HotmTreeBuilder {
    /// java: extends SkillTreeBuilder
    pub base: SkillTreeBuilderBase,
    /// java: private @Nullable Layout layout;
    pub layout: Option<Layout>,
    /// java: private @Nullable LispExecutionContext lispContext;
    pub lisp_context: Option<Arc<crate::unresolved::LispExecutionContext>>,
}

impl std::ops::Deref for HotmTreeBuilder {
    type Target = SkillTreeBuilderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HotmTreeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HotmTreeBuilder {
    /// java: private HotmTreeBuilder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SkillTreeBuilderBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                layout: None,
                lisp_context: None,
            });
            ((__h.clone() as compat::Shared<dyn SkillTreeBuilder>), __h)
        });
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
    }

    /// java: private void updateLayout(Layout layout)
    pub fn update_layout(__this: &compat::Shared<Self>, mut layout: Layout) {
        { let __v = Some(layout.clone()); compat::write(__this, |__t| __t.layout = __v); }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            HotmTreeBuilder::update_lisp_context(__this, &layout);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Failed to update LISP context."));
        }
    }

    /// java: private void updateLispContext(Layout layout)
    pub fn update_lisp_context(__this: &compat::Shared<Self>, layout: &Layout) {
        // TODO(fabric2seer): `new LispExecutionContext(..)`
        //   java: LispExecutionContext lec = new LispExecutionContext();
        let mut lec: Arc<crate::unresolved::LispExecutionContext> = todo!("fabric2seer: the initialiser of `lec` (TODO above)");
        // TODO(fabric2seer): `.setupStandardBindings(..)` on a LispExecutionContext
        //   java: lec.setupStandardBindings();
        // TODO(fabric2seer): `new AutoBinder(..)`
        //   java: new AutoBinder().bindTo(new LispExtensions(), lec.getRootStackFrame());
        for i in 0..(layout.prelude().len() as i32) {
            let mut name: String = format!("hotmlayout:prelude:{}", i);
            // TODO(fabric2seer): `.executeProgram(..)` on a LispExecutionContext
            //   java: lec.executeProgram(lec.getRootStackFrame(), LispParser.Companion.parse(name, layout.prelude().get(i)));
        }
        { let __v = Some(lec.clone()); compat::write(__this, |__t| __t.lisp_context = __v); }
    }

    /// java: private Node buildNode(String id, Perk perk, ProfileMember member, int slot)
    pub fn build_node(__this: &compat::Shared<Self>, id: &str, perk: &Perk, member: &compat::Shared<ProfileMember>, mut slot: i32) -> Node {
        let mut tree: std::collections::HashMap<String, compat::Object> = ({ let __f = compat::dispatch(__this, |__t| __t.get_tree_nodes()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>), member, slot) });
        let mut level: i32 = PrimitiveTypeUtils::coerce_int(Some(tree.get(id).cloned().unwrap_or_else(|| (std::sync::Arc::new(0) as compat::Object))));
        let mut enabled: bool = PrimitiveTypeUtils::coerce_boolean(Some(tree.get(&format!("toggle_{}", id)).cloned().unwrap_or_else(|| (std::sync::Arc::new(true) as compat::Object))));
        let mut is_max_level: bool = (level == perk.max_level());
        let mut hotm: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_heart_of_the_x_level()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>), member) });
        let mut cotm: i32 = PrimitiveTypeUtils::coerce_int(Some(tree.get(&({ let __f = compat::dispatch(__this, |__t| __t.get_core_of_the_x_id()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>)) })).cloned().unwrap_or_else(|| (std::sync::Arc::new(0) as compat::Object))));
        let mut values_and_bindings: (std::collections::HashMap<String, Arc<crate::unresolved::LispData>>, Arc<crate::unresolved::StackFrame>) = HotmTreeBuilder::calculate_perk_values(__this, perk, level, hotm, cotm);
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = ({ let __f = compat::dispatch(__this, |__t| __t.get_icon()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>), enabled, &values_and_bindings.0) });
        let mut tooltip: Vec<MutableComponent> = HotmTreeBuilder::get_tooltip(__this, perk, id, level, enabled, &values_and_bindings.0, &values_and_bindings.1, &__this.lock().borrow().layout.as_ref().unwrap().hotm().powders());
        // java: @Nullable Identifier tooltipStyle = isMaxLevel ? SkyBlockTooltipStyles.LEGENDARY : null;
        let mut tooltip_style: Option<Identifier> = (if is_max_level { Some(sky_block_tooltip_styles::LEGENDARY.get_mut().clone()) } else { None });
        let mut row: i32 = perk.y();
        let mut column: i32 = perk.x();
        return Node::new(icon.clone(), tooltip.clone(), tooltip_style, row, column);
    }

    /// java: private Pair<Map<String, LispData>, StackFrame> calculatePerkValues(Perk perk, int level, int hotm, int cotm)
    pub fn calculate_perk_values(__this: &compat::Shared<Self>, perk: &Perk, mut level: i32, mut hotm: i32, mut cotm: i32) -> (std::collections::HashMap<String, Arc<crate::unresolved::LispData>>, Arc<crate::unresolved::StackFrame>) {
        // TODO(fabric2seer): `.genBindings(..)` on a LispExecutionContext
        //   java: StackFrame bindings = this.lispContext.genBindings();
        let mut bindings: Arc<crate::unresolved::StackFrame> = todo!("fabric2seer: the initialiser of `bindings` (TODO above)");
        // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
        //   java: bindings.setValueLocal("hotm", new LispData.LispNumber(hotm));
        // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
        //   java: bindings.setValueLocal("potm", new LispData.LispNumber(cotm));
        // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
        //   java: bindings.setValueLocal("level", new LispData.LispNumber(level == 0 ? 1d : level));
        // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
        //   java: bindings.setValueLocal("maxLevel", new LispData.LispNumber(perk.maxLevel()));
        // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
        //   java: bindings.setValueLocal("level0", new LispData.LispNumber(level));
        // TODO(fabric2seer): static field Map.Entry
        //   java: Map<String, LispData> values = perk.programCache().entrySet().stream()
        //   java: 				.collect(Collectors.toMap(Map.Entry::getKey, entry -> this.lispContext.executeProgram(bindings.fork(), entry.getValue())));
        let mut values: std::collections::HashMap<String, Arc<crate::unresolved::LispData>> = todo!("fabric2seer: the initialiser of `values` (TODO above)");
        return (values.clone(), bindings.clone());
    }

    /// java: private List<Component> getTooltip(Perk perk, String id, int level, boolean enabled, Map<String, LispData> values, StackFrame bindings, Map<String, PowderInfo> powders)
    pub fn get_tooltip(__this: &compat::Shared<Self>, perk: &Perk, id: &str, mut level: i32, mut enabled: bool, values: &std::collections::HashMap<String, Arc<crate::unresolved::LispData>>, bindings: &Arc<crate::unresolved::StackFrame>, powders: &std::collections::HashMap<String, PowderInfo>) -> Vec<MutableComponent> {
        let mut max_level: bool = (perk.max_level() == level);
        let mut tooltip: Vec<MutableComponent> = Vec::new();
        let mut name_formatting: ChatFormatting = (if enabled && level != 0 { (if max_level { ChatFormatting::Green } else { ChatFormatting::Yellow }) } else { ChatFormatting::Red });
        tooltip.push(component::literal(&perk.name()).with_style_format(name_formatting));
        if perk.max_level() > 1 {
            if max_level {
                tooltip.push((component::literal(&format!("Level {}", perk.max_level()))).with_style_format(ChatFormatting::Gray));
            } else {
                let mut level_text: MutableComponent = (component::empty().append((component::literal(&format!("Level {}", level))).with_style_format(ChatFormatting::Gray))).append((component::literal(&format!("/{}", perk.max_level()))).with_style_format(ChatFormatting::DarkGray));
                tooltip.push(level_text.clone());
            }
        }
        tooltip.push(component::empty());
        let mut dynamic_tooltip: Vec<String> = Vec::new();
        {
            // TODO(fabric2seer): static field CoreBindings.INSTANCE
            //   java: List<String> applicableLines = perk.lore().stream()
            //   java: 					.filter(line -> line.condition().isPresent() ? CoreBindings.INSTANCE.isTruthy(this.lispContext.executeProgram(bindings.fork(), line.condition().get())) : true)
            //   java: 					.map(LoreLine::template)
            //   java: 					.toList();
            let mut applicable_lines: Vec<String> = todo!("fabric2seer: the initialiser of `applicableLines` (TODO above)");
            dynamic_tooltip.extend(applicable_lines.iter().cloned());
        }
        'powder: {
            if level == 0 || max_level {
                break 'powder;
            }
            dynamic_tooltip.push("".to_string());
            // TODO(fabric2seer): `.fork(..)` on a StackFrame
            //   java: StackFrame powderBindings = bindings.fork();
            let mut powder_bindings: Arc<crate::unresolved::StackFrame> = todo!("fabric2seer: the initialiser of `powderBindings` (TODO above)");
            'powder: for powder in powders.keys().cloned() {
                // TODO(fabric2seer): `.setValueLocal(..)` on a StackFrame
                //   java: powderBindings.setValueLocal(powder, new LispData.LispString(powder));
            }
            // TODO(fabric2seer): `.executeProgram(..)` on a LispExecutionContext
            //   java: String powderType = this.lispContext.executeProgram(powderBindings, perk.powder()) instanceof LispData.LispString string ? string.getString() : "";
            let mut powder_type: String = todo!("fabric2seer: the initialiser of `powderType` (TODO above)");
            let mut cost_line: String = (powders.get(&powder_type).cloned().unwrap_or_else(|| skill_tree_builder::POWDER_INFO_EMPTY.clone())).cost_line();
            dynamic_tooltip.push(cost_line.clone());
        }
        // TODO(fabric2seer): a lambda / method reference where this call does not take one
        //   java: dynamicTooltip.replaceAll(line -> LORE_TEMPLATE_PATTERN.matcher(line).replaceAll(match -> {
        //   java: 			String name = match.group("name");
        //   java:
        //   java: 			if (name != null) {
        //   java: 				return switch (values.get(name)) {
        //   java: 					case LispData.LispString string -> string.getString();
        //   java: 					case LispData.LispNumber number -> {
        //   java: 						double value = name.equals("cost") ? Math.floor(number.getValue()) : number.getValue();
        //   java: 						yield Formatters.DOUBLE_NUMBERS.format(value);
        //   java: 					}
        //   java: 					case null, default -> "<lisp error - " + name + ">";
        //   java: 				};
        //   java: 			}
        //   java:
        //   java: 			return line;
        //   java: 		}));
        for line in dynamic_tooltip.iter() {
            tooltip.push(component::literal(line));
        }
        if level > 0 {
            tooltip.push(component::empty());
            if enabled {
                let mut is_cotm: bool = (id == ({ let __f = compat::dispatch(__this, |__t| __t.get_core_of_the_x_id()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>)) }));
                tooltip.push((component::literal((if is_cotm { "UNLOCKED" } else { "ENABLED" }))).with_style_formats(&[ChatFormatting::Green, ChatFormatting::Bold]));
            } else {
                tooltip.push(component::literal("DISABLED").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]));
            }
        }
        return tooltip.iter().cloned().collect::<Vec<_>>();
    }

}

impl SkillTreeBuilder for HotmTreeBuilder {
    fn skill_tree_builder_base(&self) -> &SkillTreeBuilderBase {
        &self.base
    }
    fn skill_tree_builder_base_mut(&mut self) -> &mut SkillTreeBuilderBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void loadLayout() (dispatch: the body for this class)
    fn load_layout(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) {
        HotmTreeBuilder::load_layout_dyn
    }

    /// java: @Override public LayoutElement buildLayout(ProfileMember member, int slot) (dispatch: the body for this class)
    fn build_layout(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>, i32) -> LayoutElement {
        HotmTreeBuilder::build_layout_dyn
    }

    /// java: @Override protected Map<String, Object> getTreeNodes(ProfileMember member, int slot) (dispatch: the body for this class)
    fn get_tree_nodes(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>, i32) -> std::collections::HashMap<String, compat::Object> {
        HotmTreeBuilder::get_tree_nodes_dyn
    }

    /// java: @Override protected int getHeartOfTheXLevel(ProfileMember member) (dispatch: the body for this class)
    fn get_heart_of_the_x_level(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>) -> i32 {
        HotmTreeBuilder::get_heart_of_the_x_level_dyn
    }

    /// java: @Override protected String getCoreOfTheXId() (dispatch: the body for this class)
    fn get_core_of_the_x_id(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) -> String {
        HotmTreeBuilder::get_core_of_the_x_id_dyn
    }

    /// java: @Override protected ItemStack getDisabledPerkIcon() (dispatch: the body for this class)
    fn get_disabled_perk_icon(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        HotmTreeBuilder::get_disabled_perk_icon_dyn
    }

}

impl HotmTreeBuilder {
    /// java: @Override protected void loadLayout()
    pub fn load_layout_impl(__this: &compat::Shared<Self>) {
        {
            // TODO(fabric2seer): `.stream(..)` on a NEURepoFile
            //   java: InputStream stream = NEURepoManager.file(..).stream(..);
            let mut stream: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `stream` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.readAllBytes(..)` on a InputStream
                //   java: String content = new String(stream.readAllBytes());
                let mut content: String = todo!("fabric2seer: the initialiser of `content` (TODO above)");
                let mut layout: Layout = (LAYOUT_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&content).expect("JsonSyntaxException")))).get_or_throw().unwrap();
                let __this_1 = compat::RenderThread::new(__this.clone());
                CompletableFuture::<()>::run_async(move || (|| { HotmTreeBuilder::update_layout(__this_1.get(), layout.clone()); })(), |__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load HOTM layout."));
            }
        }
    }

    /// HotmTreeBuilder.loadLayout as the dispatch of SkillTreeBuilder.loadLayout
    pub fn load_layout_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>) {
        Self::load_layout_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public LayoutElement buildLayout(ProfileMember member, int slot)
    pub fn build_layout_impl(__this: &compat::Shared<Self>, member: &compat::Shared<ProfileMember>, mut slot: i32) -> LayoutElement {
        if compat::read(__this, |__t| __t.layout.is_none()) || compat::read(__this, |__t| __t.lisp_context.is_none()) {
            // java: return new StringWidget(Component.literal("Error loading HOTM layout :("), Minecraft.getInstance().font);
            // rust: port gap (GAPS_batchC #L3) - a widget object is not a layout element outside a screen: an empty layout
            // stands where the error text would be
            let _ = compat::string_widget(0, 0, -1, 9, component::literal("Error loading HOTM layout :("));
            return LayoutElement::Grid(mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout::new_default());
        }
        let __perks: Vec<(String, Perk)> = compat::read(__this, |__t| __t.layout.as_ref().unwrap().hotm().perks().iter().map(|(k, v)| (k.clone(), v.clone())).collect());
        let mut nodes: Vec<Node> = __perks.iter().map(|entry| HotmTreeBuilder::build_node(__this, &entry.0, &entry.1, member, slot)).collect::<Vec<_>>();
        return ({ let __f = compat::dispatch(__this, |__t| __t.build_grid()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>), &nodes) });
    }

    /// HotmTreeBuilder.buildLayout as the dispatch of SkillTreeBuilder.buildLayout
    pub fn build_layout_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>, __p0: &compat::Shared<ProfileMember>, __p1: i32) -> LayoutElement {
        Self::build_layout_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected Map<String, Object> getTreeNodes(ProfileMember member, int slot)
    pub fn get_tree_nodes_impl(__this: &compat::Shared<Self>, member: &compat::Shared<ProfileMember>, mut slot: i32) -> std::collections::HashMap<String, compat::Object> {
        return member.lock().borrow().skill_tree.nodes.get_mining_node(slot);
    }

    /// HotmTreeBuilder.getTreeNodes as the dispatch of SkillTreeBuilder.getTreeNodes
    pub fn get_tree_nodes_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>, __p0: &compat::Shared<ProfileMember>, __p1: i32) -> std::collections::HashMap<String, compat::Object> {
        Self::get_tree_nodes_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected int getHeartOfTheXLevel(ProfileMember member)
    pub fn get_heart_of_the_x_level_impl(__this: &compat::Shared<Self>, member: &compat::Shared<ProfileMember>) -> i32 {
        return member.lock().borrow().skill_tree.experience.get_hotm_level().level();
    }

    /// HotmTreeBuilder.getHeartOfTheXLevel as the dispatch of SkillTreeBuilder.getHeartOfTheXLevel
    pub fn get_heart_of_the_x_level_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>, __p0: &compat::Shared<ProfileMember>) -> i32 {
        Self::get_heart_of_the_x_level_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected String getCoreOfTheXId()
    pub fn get_core_of_the_x_id_impl(__this: &compat::Shared<Self>) -> String {
        return "core_of_the_mountain".to_string();
    }

    /// HotmTreeBuilder.getCoreOfTheXId as the dispatch of SkillTreeBuilder.getCoreOfTheXId
    pub fn get_core_of_the_x_id_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>) -> String {
        Self::get_core_of_the_x_id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected ItemStack getDisabledPerkIcon()
    pub fn get_disabled_perk_icon_impl(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return FlexibleItemStack::get_stack_or_throw(ico::REDSTONE_BLOCK.get_mut());
    }

    /// HotmTreeBuilder.getDisabledPerkIcon as the dispatch of SkillTreeBuilder.getDisabledPerkIcon
    pub fn get_disabled_perk_icon_dyn(__this: &compat::Shared<dyn SkillTreeBuilder>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        Self::get_disabled_perk_icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private record Layout(List<String> prelude, Hotx hotm) implements AbstractLayout
#[derive(Clone)]
pub struct Layout {
    /// java: List<String> prelude
    pub prelude: Vec<String>,
    /// java: Hotx hotm
    pub hotm: Hotx,
}

impl Layout {
    /// java: the canonical record constructor
    pub fn new(mut prelude: Vec<String>, mut hotm: Hotx) -> Self {
        Self { prelude, hotm }
    }

    /// java: the record accessor `prelude()`
    pub fn prelude(&self) -> Vec<String> {
        self.prelude.clone()
    }

    /// java: the record accessor `hotm()`
    pub fn hotm(&self) -> Hotx {
        self.hotm.clone()
    }

}

impl AbstractLayout for Layout {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: the record accessor `prelude()`, which implements AbstractLayout.prelude() (dispatch: the body for this class)
    fn prelude(&self) -> fn(&compat::Shared<dyn AbstractLayout>) -> Vec<String> {
        Layout::prelude_dyn
    }

    /// java: @Override public Hotx hotx() (dispatch: the body for this class)
    fn hotx(&self) -> fn(&compat::Shared<dyn AbstractLayout>) -> Hotx {
        Layout::hotx_dyn
    }

}

impl Layout {
    /// java: @Override public Hotx hotx()
    pub fn hotx_impl(&self) -> Hotx {
        return self.hotm();
    }

    /// Layout.prelude as the dispatch of AbstractLayout.prelude
    pub fn prelude_dyn(__this: &compat::Shared<dyn AbstractLayout>) -> Vec<String> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.prelude())
    }

    /// Layout.hotx as the dispatch of AbstractLayout.hotx
    pub fn hotx_dyn(__this: &compat::Shared<dyn AbstractLayout>) -> Hotx {
        compat::read_as::<Self, _, _>(__this, |__s| __s.hotx_impl())
    }

}
