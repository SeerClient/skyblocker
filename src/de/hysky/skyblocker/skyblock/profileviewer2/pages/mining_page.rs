//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.MiningPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::hotm_tree_builder;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder::SkillTreeBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public final class MiningPage implements ProfileViewerPage<LoadingInformation>
#[derive(Clone)]
pub struct MiningPage {
    /// java: private List<AbstractWidget> widgets = new ArrayList<>();
    pub widgets: Vec<AbstractWidget>,
}

impl MiningPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: Vec::new(),
        };
        this
    }

}

impl ProfileViewerPageAny for MiningPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::STONE_PICKAXE.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Mining");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        MiningPage::load_dyn
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.clone();
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for MiningPage {
    /// java: @Override public LayoutElement buildWidgets(LoadingInformation info)
    fn build_widgets(&self, info: &LoadingInformation) -> LayoutElement {
        let mut member: compat::Shared<ProfileMember> = info.member();
        let mut page_layout: LinearLayout = LinearLayout::vertical();
        let mut hotm_tree: LayoutElement = ({ let __r = hotm_tree_builder::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.build_layout()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SkillTreeBuilder>), &member, member.lock().borrow().skill_tree.selected_skill_tree_slot.mining) });
        page_layout.add_child(hotm_tree);
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|__a0| { self.widgets.push(__a0.clone()); });
        return LayoutElement::Grid(page_layout.wrapped);
    }

}

impl MiningPage {
    /// MiningPage.load as the dispatch of ProfileViewerPage.load
    pub fn load_dyn(__this: &compat::Shared<dyn ProfileViewerPageAny>, info: &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        // java: return CompletableFuture.completedFuture(info).thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        let __h: compat::Shared<MiningPage> = compat::handle_as::<MiningPage, _>(__this);
        return compat::LocalFuture::completed_future(info.clone()).then_apply_async(move |__a0| compat::read(&__h, |__t| __t.build_widgets(&__a0)));
    }
}
