//! Ported from de.hysky.skyblocker.utils.config.DurationController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::config::duration_controller_widget::DurationControllerWidget;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use dandelion::net::azureaaron::dandelion::api::controllers::controller::{Controller, ControllerKind, NumberControllerKind};
use dandelion::net::azureaaron::dandelion::api::controllers::integer_controller::IntegerController;
use dandelion::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use dandelion::net::azureaaron::dandelion::api::option::Option as DandelionOption;
use dandelion::net::azureaaron::dandelion::r#impl::controllers::integer_controller_impl::IntegerControllerImpl;
use dandelion::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_number_field_editor::{DandelionNumberFieldEditor, NumberFieldClass};
use dandelion::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use yacl::dev::isxander::yacl3::api::controller::Controller as YController;
use yacl::dev::isxander::yacl3::api::controller::controller_builder::{self, ControllerBuilder};
use yacl::dev::isxander::yacl3::api::option::Option as YOption;
use yacl::dev::isxander::yacl3::api::utils::dimension::Dimension;
use yacl::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget as YAbstractWidget;
use yacl::dev::isxander::yacl3::gui::controllers::string::i_string_controller::{self, IStringController};
use yacl::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: private static final Pattern SECONDS_PATTERN = Pattern.compile("(^|\\s)(\\d+)s(\\s|$)");
pub static SECONDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)s(\\s|$)"));
/// java: private static final Pattern MINUTES_PATTERN = Pattern.compile("(^|\\s)(\\d+)m(\\s|$)");
pub static MINUTES_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)m(\\s|$)"));
/// java: private static final Pattern HOURS_PATTERN = Pattern.compile("(^|\\s)(\\d+)h(\\s|$)");
pub static HOURS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)h(\\s|$)"));

/// java: public class DurationController extends IntegerControllerImpl
pub struct DurationController {
    /// java: extends net.azureaaron.dandelion.impl.controllers.IntegerControllerImpl
    pub base: IntegerControllerImpl,
}

impl std::ops::Deref for DurationController {
    type Target = IntegerControllerImpl;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DurationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DurationController {
    /// java: public DurationController()
    pub fn new() -> Self {
        // java: super(0, Integer.MAX_VALUE, 1, false);
        Self { base: IntegerControllerImpl::new(0, i32::MAX, 1, false) }
    }

    /// java: @SuppressWarnings("unused") private static String toString(int duration)
    pub fn to_string(mut duration: i32) -> String {
        return (SkyblockTime::format_time((duration) as f64)).get_string();
    }

    /// java: @SuppressWarnings("unused") public static int fromString(String value)
    pub fn from_string(value: &str) -> i32 {
        let mut hours_matcher: compat::regex::Matcher = HOURS_PATTERN.matcher(value);
        let mut minutes_matcher: compat::regex::Matcher = MINUTES_PATTERN.matcher(value);
        let mut seconds_matcher: compat::regex::Matcher = SECONDS_PATTERN.matcher(value);
        let mut result: i32 = 0;
        // java: Integer.parseInt(..) - no trim
        if hours_matcher.find() {
            result = result.wrapping_add((hours_matcher.group(2).unwrap().parse::<i32>().expect("NumberFormatException")).wrapping_mul(3600));
        }
        if minutes_matcher.find() {
            result = result.wrapping_add((minutes_matcher.group(2).unwrap().parse::<i32>().expect("NumberFormatException")).wrapping_mul(60));
        }
        if seconds_matcher.find() {
            result = result.wrapping_add(seconds_matcher.group(2).unwrap().parse::<i32>().expect("NumberFormatException"));
        }
        return result;
    }

    /// java: @SuppressWarnings("unused") public static boolean isValid(String s)
    pub fn is_valid(mut s: String) -> bool {
        let mut hours_matcher: compat::regex::Matcher = HOURS_PATTERN.matcher(&s);
        let mut minutes_matcher: compat::regex::Matcher = MINUTES_PATTERN.matcher(&s);
        let mut seconds_matcher: compat::regex::Matcher = SECONDS_PATTERN.matcher(&s);
        let mut hours_count: i32 = 0;
        while hours_matcher.find() {
            hours_count += 1;
        }
        let mut minutes_count: i32 = 0;
        while minutes_matcher.find() {
            minutes_count += 1;
        }
        let mut seconds_count: i32 = 0;
        while seconds_matcher.find() {
            seconds_count += 1;
        }
        if hours_count == 0 && minutes_count == 0 && seconds_count == 0 {
            return false;
        }
        if hours_count > 1 || minutes_count > 1 || seconds_count > 1 {
            return false;
        }
        s = compat::regex::replace_all(&s, &HOURS_PATTERN.pattern().to_string(), "");
        s = compat::regex::replace_all(&s, &MINUTES_PATTERN.pattern().to_string(), "");
        s = compat::regex::replace_all(&s, &SECONDS_PATTERN.pattern().to_string(), "");
        return compat::is_blank(&s);
    }
}

/// `IntegerControllerImpl implements IntegerController`, inherited: Dandelion's sealed `Controller` arm is the
/// integer one (`IntegerControllerImpl.kind`).
impl Controller<i32> for DurationController {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Number(NumberControllerKind::Integer(self))
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override public @Nullable GuiOptionEditor controllerMoulConfig(Option<Integer> option, ProcessedOption moulConfigOption, MoulConfigDefinition configDefinition)
    fn controller_moul_config(
        &self,
        option: Arc<dyn DandelionOption<i32>>,
        moul_config_option: Arc<dyn moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::ProcessedOption>,
        config_definition: &Arc<MoulConfigDefinition>,
    ) -> Arc<dyn moulconfig::io::github::notenoughupdates::moulconfig::gui::gui_option_editor::GuiOptionEditor> {
        let _ = config_definition;
        // java: return new DurationControllerMoul(moulConfigOption, min(), max(), option.binding().defaultValue());
        DurationControllerMoul::new(moul_config_option, NumberController::min(self) as f32, NumberController::max(self) as f32, option.binding().default_value() as f32)
    }

    /// java: @Override public ControllerBuilder<Integer> controllerYACL(dev.isxander.yacl3.api.Option<Integer> yaclOption, Class<Integer> type)
    fn controller_yacl(&self, yacl_option: Arc<dyn YOption<i32>>, r#type: yacl::compat::ClassOf<i32>) -> Box<dyn ControllerBuilder<i32>> {
        let _ = r#type;
        // java: return () -> new DurationControllerYACL(yaclOption);
        Box::new(controller_builder::of(move || Arc::new(DurationControllerYACL::new(yacl_option.clone())) as Arc<dyn YController<i32>>))
    }
}

impl NumberController<i32> for DurationController {
    /// `IntegerControllerImpl.min()`, inherited
    fn min(&self) -> i32 {
        NumberController::min(&self.base)
    }

    /// `IntegerControllerImpl.max()`, inherited
    fn max(&self) -> i32 {
        NumberController::max(&self.base)
    }

    /// `IntegerControllerImpl.step()`, inherited
    fn step(&self) -> i32 {
        NumberController::step(&self.base)
    }

    /// `IntegerControllerImpl.slider()`, inherited
    fn slider(&self) -> bool {
        NumberController::slider(&self.base)
    }
}

impl IntegerController for DurationController {}

/// java: public static class DurationControllerMoul extends DandelionNumberFieldEditor
///
/// The subclass overrides `toString(Float)` and `parseNumber(..)`, which `DandelionNumberFieldEditor` calls
/// virtually; the object is a `DandelionNumberFieldEditor` built with those two (`NumberFieldClass`).
pub struct DurationControllerMoul {}

impl DurationControllerMoul {
    /// java: public DurationControllerMoul(ProcessedOption option, float minValue, float maxValue, float defaultValue)
    pub fn new(
        option: Arc<dyn moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::ProcessedOption>,
        min_value: f32,
        max_value: f32,
        default_value: f32,
    ) -> Arc<DandelionNumberFieldEditor> {
        // java: super(option, minValue, maxValue, defaultValue);
        DandelionNumberFieldEditor::new_with_class(
            option,
            min_value,
            max_value,
            default_value,
            NumberFieldClass {
                to_string: Box::new(DurationControllerMoul::to_string),
                parse_number: Box::new(DurationControllerMoul::parse_number),
            },
        )
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override protected String toString(Float floatValue)
    pub fn to_string(float_value: f32) -> String {
        // java: DurationController.toString(floatValue.intValue())
        DurationController::to_string(float_value as i32)
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override protected float parseNumber(String input, float minValue, float maxValue, float defaultValue)
    pub fn parse_number(input: &str, min_value: f32, max_value: f32, default_value: f32) -> f32 {
        let _ = (min_value, max_value, default_value);
        DurationController::from_string(input) as f32
    }
}

/// java: public record DurationControllerYACL(dev.isxander.yacl3.api.Option<Integer> option) implements IStringController<Integer>
pub struct DurationControllerYACL {
    /// java: dev.isxander.yacl3.api.Option<Integer> option
    pub option: Arc<dyn YOption<i32>>,
}

impl DurationControllerYACL {
    /// java: the canonical record constructor
    pub fn new(option: Arc<dyn YOption<i32>>) -> Self {
        Self { option }
    }
}

impl YController<i32> for DurationControllerYACL {
    /// java: the record accessor `option()` (`Controller.option()`)
    fn option(&self) -> Arc<dyn YOption<i32>> {
        self.option.clone()
    }

    /// java: IStringController.formatValue() (the interface default)
    fn format_value(&self) -> MutableComponent {
        i_string_controller::format_value::<i32, Self>(self)
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn YAbstractWidget> {
        // java: return new DurationControllerWidget(this, screen, widgetDimension);
        Arc::new(DurationControllerWidget::new(self, screen, widget_dimension))
    }
}

impl IStringController<i32> for DurationControllerYACL {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        DurationController::to_string(self.option.pending_value())
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        self.option.request_set(DurationController::from_string(value));
    }

    /// java: @Override public boolean isInputValid(String s)
    fn is_input_valid(&self, s: &str) -> bool {
        DurationController::is_valid(s.to_string())
    }
}
