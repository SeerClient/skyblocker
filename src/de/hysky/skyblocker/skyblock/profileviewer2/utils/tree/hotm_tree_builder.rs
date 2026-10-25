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
/// java: private static final Codec<Layout> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.listOf().fieldOf("prelude").forGetter(Layout::prelude), Hotx.CODEC.fieldOf("hotm").forGetter(Layout::hotm) ).apply(instance, Layout::new));
pub static LAYOUT_CODEC: compat::MainThread<CodecRef<Layout>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Layout| o.prelude(), mcrust::com::mojang::serialization::codec::field_of(mcrust::com::mojang::serialization::codec::list(mcrust::com::mojang::serialization::codec::string_codec()), "prelude")), record_codec_builder::of(|o: &Layout| o.hotm(), mcrust::com::mojang::serialization::codec::field_of(skill_tree_builder::HOTX_CODEC.get().clone(), "hotm")), move |__a0, __a1| Layout::new(__a0.clone(), __a1.clone()))));

/// java: public final class HotmTreeBuilder extends SkillTreeBuilder
pub struct HotmTreeBuilder {
    /// java: extends SkillTreeBuilder
    pub base: SkillTreeBuilderBase,
    /// java: private @Nullable Layout layout;
    pub layout: Option<Layout>,
    /// java: private @Nullable LispExecutionContext lispContext;
    pub lisp_context: Option<std::rc::Rc<crate::compat::nea_lisp::LispExecutionContext>>,
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
        // Initialize LISP context
        // java: LispExecutionContext lec = new LispExecutionContext(); lec.setupStandardBindings();
        let mut lec: std::rc::Rc<crate::compat::nea_lisp::LispExecutionContext> = std::rc::Rc::new(crate::compat::nea_lisp::LispExecutionContext::new());
        lec.setup_standard_bindings();
        // Bind the extra methods we need
        // java: new AutoBinder().bindTo(new LispExtensions(), lec.getRootStackFrame());
        crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::lisp_extensions::LispExtensions::new().bind_to(&lec.get_root_stack_frame());
        // Bind the prelude functions
        for i in 0..(layout.prelude().len() as i32) {
            let mut name: String = format!("hotmlayout:prelude:{}", i);
            // java: lec.executeProgram(lec.getRootStackFrame(), LispParser.Companion.parse(name, layout.prelude().get(i)));
            // (the parser's exception leaves this method - `updateLayout` catches it)
            let __program = crate::compat::nea_lisp::LispParser::parse(&name, &layout.prelude()[(i) as usize]).unwrap_or_else(|e| panic!("{}", e));
            lec.execute_program(&lec.get_root_stack_frame(), &__program);
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
        let mut values_and_bindings: (std::collections::HashMap<String, crate::compat::nea_lisp::LispData>, crate::compat::nea_lisp::StackFrame) = HotmTreeBuilder::calculate_perk_values(__this, perk, level, hotm, cotm);
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = ({ let __f = compat::dispatch(__this, |__t| __t.get_icon()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkillTreeBuilder>), enabled, &values_and_bindings.0) });
        let mut tooltip: Vec<MutableComponent> = HotmTreeBuilder::get_tooltip(__this, perk, id, level, enabled, &values_and_bindings.0, &values_and_bindings.1, &__this.lock().borrow().layout.as_ref().unwrap().hotm().powders());
        // java: @Nullable Identifier tooltipStyle = isMaxLevel ? SkyBlockTooltipStyles.LEGENDARY : null;
        let mut tooltip_style: Option<Identifier> = (if is_max_level { Some(sky_block_tooltip_styles::LEGENDARY.get_mut().clone()) } else { None });
        let mut row: i32 = perk.y();
        let mut column: i32 = perk.x();
        return Node::new(icon.clone(), tooltip.clone(), tooltip_style, row, column);
    }

    /// java: private Pair<Map<String, LispData>, StackFrame> calculatePerkValues(Perk perk, int level, int hotm, int cotm)
    pub fn calculate_perk_values(__this: &compat::Shared<Self>, perk: &Perk, mut level: i32, mut hotm: i32, mut cotm: i32) -> (std::collections::HashMap<String, crate::compat::nea_lisp::LispData>, crate::compat::nea_lisp::StackFrame) {
        let lisp_context = compat::read(__this, |__t| __t.lisp_context.clone()).expect("NullPointerException: lispContext");
        // java: StackFrame bindings = this.lispContext.genBindings();
        let mut bindings: crate::compat::nea_lisp::StackFrame = lisp_context.gen_bindings();
        bindings.set_value_local("hotm", crate::compat::nea_lisp::LispData::LispNumber(hotm as f64));
        bindings.set_value_local("potm", crate::compat::nea_lisp::LispData::LispNumber(cotm as f64));
        bindings.set_value_local("level", crate::compat::nea_lisp::LispData::LispNumber(if level == 0 { 1.0 } else { level as f64 }));
        bindings.set_value_local("maxLevel", crate::compat::nea_lisp::LispData::LispNumber(perk.max_level() as f64));
        bindings.set_value_local("level0", crate::compat::nea_lisp::LispData::LispNumber(level as f64));
        // java: Map<String, LispData> values = perk.programCache().entrySet().stream()
        //   .collect(Collectors.toMap(Map.Entry::getKey, entry -> this.lispContext.executeProgram(bindings.fork(), entry.getValue())));
        let mut values: std::collections::HashMap<String, crate::compat::nea_lisp::LispData> = perk.program_cache().iter().map(|(key, program)| (key.clone(), lisp_context.execute_program(&bindings.fork(), program))).collect();
        return (values.clone(), bindings.clone());
    }

    /// java: private List<Component> getTooltip(Perk perk, String id, int level, boolean enabled, Map<String, LispData> values, StackFrame bindings, Map<String, PowderInfo> powders)
    pub fn get_tooltip(__this: &compat::Shared<Self>, perk: &Perk, id: &str, mut level: i32, mut enabled: bool, values: &std::collections::HashMap<String, crate::compat::nea_lisp::LispData>, bindings: &crate::compat::nea_lisp::StackFrame, powders: &std::collections::HashMap<String, PowderInfo>) -> Vec<MutableComponent> {
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
            // java: List<String> applicableLines = perk.lore().stream()
            //   .filter(line -> line.condition().isPresent() ? CoreBindings.INSTANCE.isTruthy(this.lispContext.executeProgram(bindings.fork(), line.condition().get())) : true)
            //   .map(LoreLine::template).toList();
            // (`isTruthy` answers a `Boolean` the filter unboxes: a non-boolean answer is a NullPointerException)
            let lisp_context = compat::read(__this, |__t| __t.lisp_context.clone()).expect("NullPointerException: lispContext");
            let mut applicable_lines: Vec<String> = perk.lore().into_iter().filter(|line| match line.condition() {
                Some(condition) => crate::compat::nea_lisp::is_truthy(&lisp_context.execute_program(&bindings.fork(), &condition)).expect("NullPointerException: CoreBindings.isTruthy answered null"),
                None => true,
            }).map(|line| line.template()).collect();
            dynamic_tooltip.extend(applicable_lines.iter().cloned());
        }
        'powder: {
            if level == 0 || max_level {
                break 'powder;
            }
            dynamic_tooltip.push("".to_string());
            // java: StackFrame powderBindings = bindings.fork();
            let mut powder_bindings: crate::compat::nea_lisp::StackFrame = bindings.fork();
            for powder in powders.keys().cloned() {
                // java: powderBindings.setValueLocal(powder, new LispData.LispString(powder));
                powder_bindings.set_value_local(&powder, crate::compat::nea_lisp::LispData::LispString(powder.clone()));
            }
            // java: String powderType = this.lispContext.executeProgram(powderBindings, perk.powder()) instanceof LispData.LispString string ? string.getString() : "";
            let lisp_context = compat::read(__this, |__t| __t.lisp_context.clone()).expect("NullPointerException: lispContext");
            let mut powder_type: String = match lisp_context.execute_program(&powder_bindings, &perk.powder()) {
                crate::compat::nea_lisp::LispData::LispString(string) => string,
                _ => "".to_string(),
            };
            let mut cost_line: String = (powders.get(&powder_type).cloned().unwrap_or_else(|| skill_tree_builder::POWDER_INFO_EMPTY.clone())).cost_line();
            dynamic_tooltip.push(cost_line.clone());
        }
        // Fill in the templates inside the dynamic lore (e.g. {cost}, {stat}, etc.)
        // java: dynamicTooltip.replaceAll(line -> LORE_TEMPLATE_PATTERN.matcher(line).replaceAll(match -> {
        //   String name = match.group("name");
        //   if (name != null) return switch (values.get(name)) {
        //     case LispData.LispString string -> string.getString();
        //     case LispData.LispNumber number -> { double value = name.equals("cost") ? Math.floor(number.getValue()) : number.getValue(); yield Formatters.DOUBLE_NUMBERS.format(value); }
        //     case null, default -> "<lisp error - " + name + ">"; };
        //   return line; }));
        // (`return line` is the group-less match's replacement: the whole line, as Java's lambda answers it)
        for line in dynamic_tooltip.iter_mut() {
            let original = line.clone();
            let mut matcher = skill_tree_builder::LORE_TEMPLATE_PATTERN.matcher(&original);
            let mut replaced = String::new();
            let mut last_end = 0;
            while matcher.find() {
                let (start, end) = (matcher.start(0), matcher.end(0));
                replaced.push_str(&compat::substring2(&original, last_end, start));
                let replacement: String = match matcher.group_named("name") {
                    Some(name) => match values.get(&name) {
                        Some(crate::compat::nea_lisp::LispData::LispString(string)) => string.clone(),
                        Some(crate::compat::nea_lisp::LispData::LispNumber(number)) => {
                            let value: f64 = if name == "cost" { number.floor() } else { *number };
                            crate::de::hysky::skyblocker::utils::formatters::DOUBLE_NUMBERS.get_mut().format(value)
                        }
                        _ => format!("<lisp error - {}>", name),
                    },
                    None => original.clone(),
                };
                replaced.push_str(&replacement);
                last_end = end;
            }
            replaced.push_str(&compat::substring(&original, last_end));
            *line = replaced;
        }
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
            // java: try (InputStream stream = NEURepoManager.file("constants/hotmlayout.json").stream()) {..} catch (Exception e) - the
            // resource is opened inside the try (a missing file is a null `file(..)`); an exception is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let stream = NEURepoManager::file("constants/hotmlayout.json")
                    .expect("java.lang.NullPointerException: NEURepoManager.file(\"constants/hotmlayout.json\")");
                //   java: String content = new String(stream.readAllBytes());
                let mut content: String = compat::neu_repo_file_read_all(&stream);
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
            // (the widget joins the list of the page whose layout this is)
            return crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::string_widget(0, 0, -1, 9, component::literal("Error loading HOTM layout :(")));
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
