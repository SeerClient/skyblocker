//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.CombatPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::ruler_widget::RulerWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public final class CombatPage implements ProfileViewerPage<LoadingInformation>
pub struct CombatPage {
    /// java: private final List<AbstractWidget> widgets = new ArrayList<>(); (held in a `Screen`, the port's holder
    /// of game widgets: see `ProfileViewerPageAny::widgets_mut`)
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl CombatPage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        this
    }

}

impl ProfileViewerPageAny for CombatPage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FlexibleItemStack getIcon()
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return ico::MADDOX_BATPHONE.get_mut().clone();
    }

    /// java: @Override public Component getName()
    fn get_name(&self) -> MutableComponent {
        return component::literal("Combat");
    }

    /// java: @Override public CompletableFuture<LayoutElement> load(LoadingInformation info) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        CombatPage::load_dyn
    }

    /// java: @Override public List<AbstractWidget> getWidgets()
    fn get_widgets(&self) -> Vec<AbstractWidget> {
        return self.widgets.children().to_vec();
    }

    fn widgets_mut(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.widgets
    }

}

impl ProfileViewerPage<crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation> for CombatPage {
    /// java: @Override public LayoutElement buildWidgets(LoadingInformation data)
    fn build_widgets(&self, data: &LoadingInformation) -> LayoutElement {
        let mut vertical: LinearLayout = LinearLayout::vertical();
        vertical.add_child(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::WidgetAdapter::widget(RulerWidget::new())));
        vertical.add_child_adjusted(crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::layout_widget(compat::string_widget(0, 0, -1, 9, self.get_name())), &mut |l| { *l = l.padding_left(18).padding_top(18); });
        // java: vertical.visitWidgets(this.widgets::add); - every widget the layout holds was collected as it was put in
        // it (`layout_widget`); `load` hands them to the page
        return LayoutElement::Grid(vertical.wrapped);
    }

}

impl CombatPage {
    /// CombatPage.load as the dispatch of ProfileViewerPage.load
    pub fn load_dyn(__this: &compat::Shared<dyn ProfileViewerPageAny>, info: &LoadingInformation) -> compat::LocalFuture<LayoutElement> {
        // java: return CompletableFuture.completedFuture(info).thenApplyAsync(this::buildWidgets, Minecraft.getInstance());
        let __h: compat::Shared<CombatPage> = compat::handle_as::<CombatPage, _>(__this);
        return compat::LocalFuture::completed_future(info.clone()).then_apply_async(move |__a0| {
            let (__layout, __widgets) = crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::collect_widgets(|| compat::read(&__h, |__t| __t.build_widgets(&__a0)));
            compat::write(&__h, |__t| __t.widgets = __widgets);
            __layout
        });
    }
}
