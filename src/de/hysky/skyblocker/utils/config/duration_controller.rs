//! Ported from de.hysky.skyblocker.utils.config.DurationController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::config::duration_controller_widget::DurationControllerWidget;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern SECONDS_PATTERN = Pattern.compile("(^|\\s)(\\d+)s(\\s|$)");
pub static SECONDS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)s(\\s|$)"));
/// java: private static final Pattern MINUTES_PATTERN = Pattern.compile("(^|\\s)(\\d+)m(\\s|$)");
pub static MINUTES_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)m(\\s|$)"));
/// java: private static final Pattern HOURS_PATTERN = Pattern.compile("(^|\\s)(\\d+)h(\\s|$)");
pub static HOURS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(^|\\s)(\\d+)h(\\s|$)"));

/// java: public class DurationController extends IntegerControllerImpl
pub struct DurationController {
    /// java: extends net.azureaaron.dandelion.impl.controllers.IntegerControllerImpl
    pub base: crate::unresolved::IntegerControllerImpl,
}

impl std::ops::Deref for DurationController {
    type Target = crate::unresolved::IntegerControllerImpl;
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
        // TODO(fabric2seer): `new IntegerControllerImpl(..)`: the port has no `IntegerControllerImpl::new`
        //   java: super(0, Integer.MAX_VALUE, 1, false);
        let mut this: Self = todo!("constructor");
        this
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
        if hours_matcher.find() {
            result = result.wrapping_add((hours_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_mul(3600));
        }
        if minutes_matcher.find() {
            result = result.wrapping_add((minutes_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_mul(60));
        }
        if seconds_matcher.find() {
            result = result.wrapping_add(seconds_matcher.group(2).unwrap().trim().parse::<i32>().expect("NumberFormatException"));
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

    /// java: @SuppressWarnings("UnstableApiUsage") @Override public @Nullable GuiOptionEditor controllerMoulConfig(Option<Integer> option, ProcessedOption moulConfigOption, MoulConfigDefinition configDefinition)
    pub fn controller_moul_config(&self, option: &crate::unresolved::Option, moul_config_option: &crate::unresolved::ProcessedOption, config_definition: &crate::unresolved::MoulConfigDefinition) -> Option<crate::unresolved::GuiOptionEditor> {
        // TODO(fabric2seer): `.binding(..)` on a Option
        //   java: return new DurationControllerMoul(moulConfigOption, min(), max(), option.binding().defaultValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override public ControllerBuilder<Integer> controllerYACL(dev.isxander.yacl3.api.Option<Integer> yaclOption, Class<Integer> type)
    pub fn controller_yacl(&self, yacl_option: &crate::unresolved::Option, r#type: &crate::unresolved::Class) -> crate::unresolved::ControllerBuilder {
        return Arc::new(move || DurationControllerYACL::new(yacl_option.clone()));
    }

}

// NOTE(fabric2seer): controllerMoulConfig, controllerYACL override(s) of IntegerControllerImpl - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public static class DurationControllerMoul extends DandelionNumberFieldEditor
pub struct DurationControllerMoul {
    /// java: extends net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberFieldEditor
    pub base: crate::unresolved::DandelionNumberFieldEditor,
}

impl std::ops::Deref for DurationControllerMoul {
    type Target = crate::unresolved::DandelionNumberFieldEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DurationControllerMoul {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DurationControllerMoul {
    /// java: public DurationControllerMoul(ProcessedOption option, float minValue, float maxValue, float defaultValue)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut min_value: f32, mut max_value: f32, mut default_value: f32) -> Self {
        // TODO(fabric2seer): `new DandelionNumberFieldEditor(..)`: the port has no `DandelionNumberFieldEditor::new`
        //   java: super(option, minValue, maxValue, defaultValue);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override protected String toString(Float floatValue)
    pub fn to_string(&self, mut float_value: f32) -> String {
        return DurationController::to_string((float_value) as i32);
    }

    /// java: @SuppressWarnings("UnstableApiUsage") @Override protected float parseNumber(String input, float minValue, float maxValue, float defaultValue)
    pub fn parse_number(&self, input: &str, mut min_value: f32, mut max_value: f32, mut default_value: f32) -> f32 {
        return ((DurationController::from_string(input)) as f32) as f32;
    }

}

// NOTE(fabric2seer): parseNumber, toString override(s) of DandelionNumberFieldEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public record DurationControllerYACL(dev.isxander.yacl3.api.Option<Integer> option) implements IStringController<Integer>
pub struct DurationControllerYACL {
    /// java: dev.isxander.yacl3.api.Option<Integer> option
    pub option: crate::unresolved::Option,
}

impl DurationControllerYACL {
    /// java: the canonical record constructor
    pub fn new(mut option: crate::unresolved::Option) -> Self {
        Self { option }
    }

    /// java: the record accessor `option()`
    pub fn option(&self) -> crate::unresolved::Option {
        self.option.clone()
    }

    /// java: @Override public String getString()
    pub fn get_string(&self) -> String {
        // TODO(fabric2seer): `.pendingValue(..)` on a Option
        //   java: return DurationController.toString(option.pendingValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void setFromString(String value)
    pub fn set_from_string(&self, value: &str) {
        // TODO(fabric2seer): `.requestSet(..)` on a Option
        //   java: option.requestSet(DurationController.fromString(value));
    }

    /// java: @Override public boolean isInputValid(String s)
    pub fn is_input_valid(&self, s: &str) -> bool {
        return DurationController::is_valid(s.to_string());
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    pub fn provide_widget(&self, screen: &crate::unresolved::YACLScreen, widget_dimension: &crate::unresolved::Dimension) -> crate::unresolved::AbstractWidget {
        return DurationControllerWidget::new(self.clone(), screen.clone(), widget_dimension.clone());
    }

}
