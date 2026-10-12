//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.tree.SkillTreeBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::lisp_extensions;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::skill_tree_node_widget::SkillTreeNodeWidget;
use crate::de::hysky::skyblocker::utils::codec_utils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::core::holder::Holder;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: private static final Set<String> PERK_PROGRAM_PREFIXES = Set.of("cost", "item", "stat");
pub static PERK_PROGRAM_PREFIXES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["cost".to_string(), "item".to_string(), "stat".to_string()]));
/// java: protected static final Pattern LORE_TEMPLATE_PATTERN = Pattern.compile("\\{(?<name>[\\w\\-]+)\\}");
pub static LORE_TEMPLATE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\{(?<name>[\\w\\-]+)\\}"));
/// java: public static final int MAX_SLOTS = 5;
pub const MAX_SLOTS: i32 = 5;
/// java: public static final int FREE_SLOTS = 2;
pub const FREE_SLOTS: i32 = 2;
/// java: protected static final Codec<Hotx> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.STRING, Perk.CODEC).fieldOf("perks").forGetter(Hotx::perks), Codec.unboundedMa
pub static HOTX_CODEC: compat::MainThread<CodecRef<Hotx>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Hotx| o.perks(), codec::field_of(string_map_codec(PERK_CODEC.get().clone()), "perks")), record_codec_builder::of(|o: &Hotx| o.powders(), codec::field_of(string_map_codec(POWDER_INFO_CODEC.get().clone()), "powders")), move |__a0, __a1| Hotx::new(__a0, __a1))));
/// `Codec.unboundedMap(Codec.STRING, valueCodec)` as the map Java decodes it to (the port's codec answers its entries)
fn string_map_codec<V: Clone + 'static>(value_codec: CodecRef<V>) -> CodecRef<std::collections::HashMap<String, V>> {
    codec::xmap(
        codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), value_codec),
        |v: Vec<(String, V)>| v.into_iter().collect::<std::collections::HashMap<_, _>>(),
        |m: &std::collections::HashMap<String, V>| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>(),
    )
}
/// java: protected static final Codec<Perk> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Perk::name), Codec.INT.fieldOf("x").forGetter(Perk::x), Codec.IN
pub static PERK_CODEC: compat::MainThread<CodecRef<Perk>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply7(record_codec_builder::of(|o: &Perk| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Perk| o.x(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "x")), record_codec_builder::of(|o: &Perk| o.y(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "y")), record_codec_builder::of(|o: &Perk| o.max_level(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "maxLevel")), record_codec_builder::of(|o: &Perk| o.powder(), codec::field_of(lisp_extensions::PROGRAM_CODEC.get_mut().clone(), "powder")), record_codec_builder::of(|o: &Perk| o.lore(), codec::field_of(LORE_LINE_LIST_CODEC.get_mut().clone(), "lore")), record_codec_builder::of(|o: &Perk| o.extra_data(), codec_utils::MAP_PASSTHROUGH.get_mut().clone()), move |__a0, __a1, __a2, __a3, __a4, __a5, __a6| Perk::new(__a0, __a1, __a2, __a3, __a4, __a5, __a6))));
/// java: private static final Codec<LoreLine> OBJECT_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("text").forGetter(LoreLine::template), LispExtensions.PROGRAM_CODEC.optio
pub static LORE_LINE_OBJECT_CODEC: compat::MainThread<CodecRef<LoreLine>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &LoreLine| o.template(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "text")), record_codec_builder::of(|o: &LoreLine| o.condition(), codec::optional_field_of_opt(lisp_extensions::PROGRAM_CODEC.get_mut().clone(), "onlyIf")), move |__a0, __a1| LoreLine::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<LoreLine> CODEC = Codec.withAlternative(OBJECT_CODEC, Codec.STRING, template -> new LoreLine(template, Optional.empty()));
pub static LORE_LINE_CODEC: compat::MainThread<CodecRef<LoreLine>> = compat::MainThread::new(|| codec::with_alternative_mapped(LORE_LINE_OBJECT_CODEC.get_mut().clone(), mcrust::com::mojang::serialization::codec::string_codec(), |template| LoreLine::new(template.clone(), None)));
/// java: protected static final Codec<List<LoreLine>> LIST_CODEC = CODEC.listOf();
pub static LORE_LINE_LIST_CODEC: compat::MainThread<CodecRef<Vec<LoreLine>>> = compat::MainThread::new(|| codec::list(LORE_LINE_CODEC.get_mut().clone()));
/// java: protected static final Codec<PowderInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("costLine").forGetter(PowderInfo::costLine) ).apply(instance, PowderInfo::ne
pub static POWDER_INFO_CODEC: compat::MainThread<CodecRef<PowderInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|o: &PowderInfo| o.cost_line(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "costLine")), move |__a0| PowderInfo::new(__a0.clone()))));
/// java: protected static final PowderInfo EMPTY = new PowderInfo("<lisp error>");
pub static POWDER_INFO_EMPTY: LazyLock<PowderInfo> = LazyLock::new(|| PowderInfo::new("<lisp error>".to_string()));

/// java: public abstract sealed class SkillTreeBuilder permits HotmTreeBuilder
#[derive(Clone)]
pub struct SkillTreeBuilderBase {}

impl SkillTreeBuilderBase {
    /// java: protected SkillTreeBuilder() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn SkillTreeBuilder>, __R)) -> (compat::Shared<dyn SkillTreeBuilder>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        ({ let __f = compat::dispatch(&__this, |__t| __t.init()); __f(&__this) });
        (__this, __r)
    }

    /// java: protected static Map<String, LispAst.Program> precompilePrograms(Dynamic<?> extraData)
    pub fn precompile_programs(extra_data: &Dynamic) -> std::collections::HashMap<String, Arc<crate::unresolved::Program>> {
        let mut cache: std::collections::HashMap<String, Arc<crate::unresolved::Program>> = std::collections::HashMap::new();
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: @SuppressWarnings({ "unchecked", "rawtypes" })
        //   java: 		Map<Dynamic<?>, Dynamic<?>> allFields = (Map) extraData.asMap(Function.identity(), Function.identity());
        let mut all_fields: std::collections::HashMap<Dynamic, Dynamic> = todo!("fabric2seer: the initialiser of `allFields` (TODO above)");
        for entry in all_fields.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut name: String = entry.0.as_string_or("");
            let mut source: String = entry.1.as_string_or("");
            if !name.is_empty() && PERK_PROGRAM_PREFIXES.iter().cloned().any(|prefix| name.starts_with(&prefix)) && !source.is_empty() {
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static field LispParser.Companion
                    //   java: cache.put(name, LispParser.Companion.parse(name, source));
                }));
                if let Err(e) = __result {
                    logger().warn(&format!("[Skyblocker Profile Viewer] Failed to compile HOTM program {}", name));
                }
            }
        }
        return cache.clone();
    }

}

/// java: the virtual methods of SkillTreeBuilder (implemented by SkillTreeBuilderBase and every subclass)
pub trait SkillTreeBuilder: compat::DynAny {
    fn skill_tree_builder_base(&self) -> &SkillTreeBuilderBase;
    fn skill_tree_builder_base_mut(&mut self) -> &mut SkillTreeBuilderBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: private void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) {
        <dyn SkillTreeBuilder>::init_body
    }

    /// java: protected abstract void loadLayout(); (dispatch: the body for this class)
    fn load_layout(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>);

    /// java: public abstract LayoutElement buildLayout(ProfileMember member, int slot); (dispatch: the body for this class)
    fn build_layout(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>, i32) -> LayoutElement;

    /// java: protected final LayoutElement buildGrid(List<Node> nodes) (dispatch: the body for this class)
    fn build_grid(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &Vec<Node>) -> LayoutElement {
        <dyn SkillTreeBuilder>::build_grid_body
    }

    /// java: protected abstract Map<String, Object> getTreeNodes(ProfileMember member, int slot); (dispatch: the body for this class)
    fn get_tree_nodes(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>, i32) -> std::collections::HashMap<String, compat::Object>;

    /// java: protected abstract int getHeartOfTheXLevel(ProfileMember member); (dispatch: the body for this class)
    fn get_heart_of_the_x_level(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, &compat::Shared<ProfileMember>) -> i32;

    /// java: protected abstract String getCoreOfTheXId(); (dispatch: the body for this class)
    fn get_core_of_the_x_id(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) -> String;

    /// java: protected abstract ItemStack getDisabledPerkIcon(); (dispatch: the body for this class)
    fn get_disabled_perk_icon(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack;

    /// java: protected final ItemStack getIcon(boolean enabled, Map<String, LispData> values) (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn SkillTreeBuilder>, bool, &std::collections::HashMap<String, Arc<crate::unresolved::LispData>>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn SkillTreeBuilder>::get_icon_body
    }

}

impl dyn SkillTreeBuilder {
    /// java: private void init()
    pub fn init_body(__this: &compat::Shared<dyn SkillTreeBuilder>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        NEURepoManager::run_async_after_load(Arc::new(move || { ({ let __f = compat::dispatch(__this_1.get(), |__t| __t.load_layout()); __f(__this_1.get()) }); }));
    }

    /// java: protected final LayoutElement buildGrid(List<Node> nodes)
    pub fn build_grid_body(__this: &compat::Shared<dyn SkillTreeBuilder>, nodes: &Vec<Node>) -> LayoutElement {
        let mut layout: GridLayout = GridLayout::new_default();
        for node in nodes.iter() {
            let mut widget: compat::Shared<SkillTreeNodeWidget> = SkillTreeNodeWidget::new(node.icon(), node.tooltip(), node.tooltip_style());
            // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
            // rust: layout.add_child_at(widget.clone(), node.row(), node.column()).unwrap();
        }
        return LayoutElement::Grid(layout);
    }

    /// java: protected final ItemStack getIcon(boolean enabled, Map<String, LispData> values)
    pub fn get_icon_body(__this: &compat::Shared<dyn SkillTreeBuilder>, mut enabled: bool, values: &std::collections::HashMap<String, Arc<crate::unresolved::LispData>>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        if !enabled {
            return ({ let __f = compat::dispatch(__this, |__t| __t.get_disabled_perk_icon()); __f(__this) });
        }
        // TODO(fabric2seer): static field LispData.LispNil
        //   java: LispData itemId = values.getOrDefault("item", LispData.LispNil.INSTANCE);
        let mut item_id: Arc<crate::unresolved::LispData> = todo!("fabric2seer: the initialiser of `itemId` (TODO above)");
        // TODO(fabric2seer): a pattern switch expression
        //   java: String idPath = switch (itemId) {
        //   java: 			case LispData.Atom atom -> atom.getLabel();
        //   java: 			case LispData.LispString string -> string.getString();
        //   java: 			default -> "barrier";
        //   java: 		};
        let mut id_path: String = todo!("fabric2seer: the initialiser of `idPath` (TODO above)");
        // TODO(fabric2seer): `.value(..)` on a value of unknown type
        //   java: return BuiltInRegistries.ITEM.get(Identifier.withDefaultNamespace(idPath.toLowerCase(Locale.ENGLISH)))
        //   java: 				.map(Holder.Reference::value)
        //   java: 				.map(ItemStack::new)
        //   java: 				.orElseGet(() -> ItemUtils.getNamedPlaceholder(idPath).getStackOrThrow());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: protected record Node(ItemStack icon, List<Component> tooltip, @Nullable Identifier tooltipStyle, int row, int column)
#[derive(Clone, Debug)]
pub struct Node {
    /// java: ItemStack icon
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: List<Component> tooltip
    pub tooltip: Vec<MutableComponent>,
    /// java: Identifier tooltipStyle
    pub tooltip_style: Option<Identifier>,
    /// java: int row
    pub row: i32,
    /// java: int column
    pub column: i32,
}

impl Node {
    /// java: the canonical record constructor
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>, mut row: i32, mut column: i32) -> Self {
        Self { icon, tooltip, tooltip_style, row, column }
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.icon.clone()
    }

    /// java: the record accessor `tooltip()`
    pub fn tooltip(&self) -> Vec<MutableComponent> {
        self.tooltip.clone()
    }

    /// java: the record accessor `tooltipStyle()`
    pub fn tooltip_style(&self) -> Option<Identifier> {
        self.tooltip_style.clone()
    }

    /// java: the record accessor `row()`
    pub fn row(&self) -> i32 {
        self.row
    }

    /// java: the record accessor `column()`
    pub fn column(&self) -> i32 {
        self.column
    }

}

/// java: protected interface AbstractLayout
pub trait AbstractLayout: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: List<String> prelude(); (dispatch: the body for this class)
    fn prelude(&self) -> fn(&compat::Shared<dyn AbstractLayout>) -> Vec<String>;

    /// java: Hotx hotx(); (dispatch: the body for this class)
    fn hotx(&self) -> fn(&compat::Shared<dyn AbstractLayout>) -> Hotx;

}

/// java: protected record Hotx(Map<String, Perk> perks, Map<String, PowderInfo> powders)
#[derive(Clone)]
pub struct Hotx {
    /// java: Map<String, Perk> perks
    pub perks: std::collections::HashMap<String, Perk>,
    /// java: Map<String, PowderInfo> powders
    pub powders: std::collections::HashMap<String, PowderInfo>,
}

impl Hotx {
    /// java: the canonical record constructor
    pub fn new(mut perks: std::collections::HashMap<String, Perk>, mut powders: std::collections::HashMap<String, PowderInfo>) -> Self {
        Self { perks, powders }
    }

    /// java: the record accessor `perks()`
    pub fn perks(&self) -> std::collections::HashMap<String, Perk> {
        self.perks.clone()
    }

    /// java: the record accessor `powders()`
    pub fn powders(&self) -> std::collections::HashMap<String, PowderInfo> {
        self.powders.clone()
    }

}

/// java: protected record Perk(String name, int x, int y, int maxLevel, LispAst.Program powder, List<LoreLine> lore, Dynamic<?> extraData, Map<String, LispAst.Program> programCache)
#[derive(Clone)]
pub struct Perk {
    /// java: String name
    pub name: String,
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: int maxLevel
    pub max_level: i32,
    /// java: LispAst.Program powder
    pub powder: Arc<crate::unresolved::Program>,
    /// java: List<LoreLine> lore
    pub lore: Vec<LoreLine>,
    /// java: Dynamic<?> extraData
    pub extra_data: Dynamic,
    /// java: Map<String, LispAst.Program> programCache
    pub program_cache: std::collections::HashMap<String, Arc<crate::unresolved::Program>>,
}

impl Perk {
    /// java: private Perk(String name, int x, int y, int maxLevel, LispAst.Program powder, List<LoreLine> lore, Dynamic<?> extraData)
    pub fn new(mut name: String, mut x: i32, mut y: i32, mut max_level: i32, mut powder: Arc<crate::unresolved::Program>, mut lore: Vec<LoreLine>, mut extra_data: Dynamic) -> Self {
        // java: this(name, x, y, maxLevel, powder, lore, extraData, precompilePrograms(extraData));
        let program_cache = SkillTreeBuilderBase::precompile_programs(&extra_data);
        Self::new_canonical(name, x, y, max_level, powder, lore, extra_data, program_cache)
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut name: String, mut x: i32, mut y: i32, mut max_level: i32, mut powder: Arc<crate::unresolved::Program>, mut lore: Vec<LoreLine>, mut extra_data: Dynamic, mut program_cache: std::collections::HashMap<String, Arc<crate::unresolved::Program>>) -> Self {
        Self { name, x, y, max_level, powder, lore, extra_data, program_cache }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: the record accessor `maxLevel()`
    pub fn max_level(&self) -> i32 {
        self.max_level
    }

    /// java: the record accessor `powder()`
    pub fn powder(&self) -> Arc<crate::unresolved::Program> {
        self.powder.clone()
    }

    /// java: the record accessor `lore()`
    pub fn lore(&self) -> Vec<LoreLine> {
        self.lore.clone()
    }

    /// java: the record accessor `extraData()`
    pub fn extra_data(&self) -> Dynamic {
        self.extra_data.clone()
    }

    /// java: the record accessor `programCache()`
    pub fn program_cache(&self) -> std::collections::HashMap<String, Arc<crate::unresolved::Program>> {
        self.program_cache.clone()
    }

}

/// java: protected record LoreLine(String template, Optional<LispAst.Program> condition)
#[derive(Clone)]
pub struct LoreLine {
    /// java: String template
    pub template: String,
    /// java: Optional<LispAst.Program> condition
    pub condition: Option<Arc<crate::unresolved::Program>>,
}

impl LoreLine {
    /// java: the canonical record constructor
    pub fn new(mut template: String, mut condition: Option<Arc<crate::unresolved::Program>>) -> Self {
        Self { template, condition }
    }

    /// java: the record accessor `template()`
    pub fn template(&self) -> String {
        self.template.clone()
    }

    /// java: the record accessor `condition()`
    pub fn condition(&self) -> Option<Arc<crate::unresolved::Program>> {
        self.condition.clone()
    }

}

/// java: protected record PowderInfo(String costLine)
#[derive(Clone, Debug, PartialEq)]
pub struct PowderInfo {
    /// java: String costLine
    pub cost_line: String,
}

impl PowderInfo {
    /// java: the canonical record constructor
    pub fn new(mut cost_line: String) -> Self {
        Self { cost_line }
    }

    /// java: the record accessor `costLine()`
    pub fn cost_line(&self) -> String {
        self.cost_line.clone()
    }

}
