//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRule by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::location;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::world::item::items;

//   java: @VisibleForTesting
//   java: 	static final Codec<EnumSet<Location>> LOCATION_FIXING_CODEC = Codec.either(Location.SET_CODEC, Codec.STRING).xmap(
//   java: 			either -> either.map(Function.identity(), ChatRule::encodeString),
//   java: 			Either::left
//   java: 	);
/// java: @VisibleForTesting static final Codec<EnumSet<Location>> LOCATION_FIXING_CODEC = Codec.either(Location.SET_CODEC, Codec.STRING).xmap( either -> either.map(Function.identity(), ChatRule::encodeString),
pub static LOCATION_FIXING_CODEC: compat::MainThread<CodecRef<std::collections::HashSet<Location>>> = compat::MainThread::new(|| codec::xmap(
    codec::either(location::SET_CODEC.get().clone(), codec::string_codec()),
    |either: mcrust::com::mojang::datafixers::util::either::Either<std::collections::HashSet<Location>, String>| either.map(|set| set, |string| ChatRule::encode_string(&string)),
    |set: &std::collections::HashSet<Location>| mcrust::com::mojang::datafixers::util::either::Either::left(set.clone()),
));
/// java: private static final UnaryOperator<Optional<String>> REMOVE_BLANK = opt -> opt.flatMap(s -> s.isBlank() ? Optional.empty() : Optional.of(s));
/// (read inside `CODEC`'s `xmap`, whose functions are `Send + Sync`: a thread-safe cell)
pub static REMOVE_BLANK: LazyLock<Arc<dyn Fn(Option<String>) -> Option<String> + Send + Sync>> = LazyLock::new(|| Arc::new(move |opt| opt.clone().and_then(|s| (if compat::is_blank(&s) { None } else { Some(s.clone()) }))));
//   java: private static final Codec<ChatRule> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(ChatRule::getName),
//   java: 			Codec.BOOL.fieldOf("enabled").forGetter(ChatRule::getEnabled),
//   java: 			Codec.BOOL.fieldOf("partialMatch").forGetter(ChatRule::getPartialMatch),
//   java: 			Codec.BOOL.fieldOf("regex").forGetter(ChatRule::getRegex),
//   java: 			Codec.BOOL.fieldOf("ignoreCase").forGetter(ChatRule::getIgnoreCase),
//   java: 			// Optional w/ default because this field wasn't always present.
//   java: 			Codec.BOOL.optionalFieldOf("includeFormatting", false).forGetter(ChatRule::getIncludeFormatting),
//   java: 			Codec.STRING.fieldOf("filter").forGetter(ChatRule::getFilter),
//   java: 			LOCATION_FIXING_CODEC.fieldOf("locations").forGetter(ChatRule::getValidLocations),
//   java: 			Codec.BOOL.fieldOf("hideOriginalMessage").forGetter(ChatRule::getHideMessage),
//   java: 			Codec.STRING.optionalFieldOf("chatMessage").xmap(REMOVE_BLANK, REMOVE_BLANK).forGetter(ChatRule::getChatMessageOptional),
//   java: 			Codec.STRING.optionalFieldOf("actionbarMessage").xmap(REMOVE_BLANK, REMOVE_BLANK).forGetter(ChatRule::getActionBarMessageOptional),
//   java: 			AnnouncementMessage.CODEC.optionalFieldOf("announcementMessage").forGetter(ChatRule::getAnnouncementMessageOptional),
//   java: 			ToastMessage.CODEC.optionalFieldOf("toastMessage").forGetter(ChatRule::getToastMessageOptional),
//   java: 			SoundEvent.DIRECT_CODEC.optionalFieldOf("customSound").forGetter(ChatRule::getCustomSoundOptional)
//   java: 	).apply(instance, (name, enabled, isPartialMatch, isRegex, isIgnoreCase, includeFormatting, filter, validLocations, hideMessage, chatMessage, actionBarMessage, announcementMessage, toastMessage, customSound) ->
//   java: 			new ChatRule(name, enabled, isPartialMatch, isRegex, isIgnoreCase, includeFormatting, filter, validLocations, hideMessage, chatMessage.orElse(null), actionBarMessage.orElse(null), announcementMessage.orElse(null), toastMessage.orElse(null), customSound.orElse(null))
//   java: 	));
/// java: private static final Codec<ChatRule> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(ChatRule::getName), Codec.BOOL.fieldOf("enabled").forGetter(Ch
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<ChatRule>>> = compat::MainThread::new(|| {
    // port shape: `RecordCodecBuilder` builds a record of `Send + Sync` values and a rule handle is not one, so the record
    // is over the getters' values (the two message objects carried in render-thread cells) and `xmap`ped to the rule -
    // `apply(instance, (..) -> new ChatRule(..))` on decode, the `forGetter`s on encode
    type Fields = (
        String,
        bool,
        bool,
        bool,
        bool,
        bool,
        String,
        std::collections::HashSet<Location>,
        bool,
        Option<String>,
        Option<String>,
        compat::RenderThread<Option<compat::Shared<AnnouncementMessage>>>,
        compat::RenderThread<Option<compat::Shared<ToastMessage>>>,
        Option<SoundEvent>,
    );
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply14(
        // Codec.STRING.fieldOf("name").forGetter(ChatRule::getName)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(codec::string_codec(), "name")),
        // Codec.BOOL.fieldOf("enabled").forGetter(ChatRule::getEnabled)
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::bool_codec(), "enabled")),
        // Codec.BOOL.fieldOf("partialMatch").forGetter(ChatRule::getPartialMatch)
        record_codec_builder::of(|o: &Fields| o.2, codec::field_of(codec::bool_codec(), "partialMatch")),
        // Codec.BOOL.fieldOf("regex").forGetter(ChatRule::getRegex)
        record_codec_builder::of(|o: &Fields| o.3, codec::field_of(codec::bool_codec(), "regex")),
        // Codec.BOOL.fieldOf("ignoreCase").forGetter(ChatRule::getIgnoreCase)
        record_codec_builder::of(|o: &Fields| o.4, codec::field_of(codec::bool_codec(), "ignoreCase")),
        // Codec.BOOL.optionalFieldOf("includeFormatting", false).forGetter(ChatRule::getIncludeFormatting)
        record_codec_builder::of(|o: &Fields| o.5, codec::optional_field_of(codec::bool_codec(), "includeFormatting", false)),
        // Codec.STRING.fieldOf("filter").forGetter(ChatRule::getFilter)
        record_codec_builder::of(|o: &Fields| o.6.clone(), codec::field_of(codec::string_codec(), "filter")),
        // LOCATION_FIXING_CODEC.fieldOf("locations").forGetter(ChatRule::getValidLocations)
        record_codec_builder::of(|o: &Fields| o.7.clone(), codec::field_of(LOCATION_FIXING_CODEC.get().clone(), "locations")),
        // Codec.BOOL.fieldOf("hideOriginalMessage").forGetter(ChatRule::getHideMessage)
        record_codec_builder::of(|o: &Fields| o.8, codec::field_of(codec::bool_codec(), "hideOriginalMessage")),
        // Codec.STRING.optionalFieldOf("chatMessage").xmap(REMOVE_BLANK, REMOVE_BLANK).forGetter(ChatRule::getChatMessageOptional)
        record_codec_builder::of(|o: &Fields| o.9.clone(), map_codec::xmap(codec::optional_field_of_opt(codec::string_codec(), "chatMessage"), |o: Option<String>| (**REMOVE_BLANK)(o), |o: &Option<String>| (**REMOVE_BLANK)(o.clone()))),
        // Codec.STRING.optionalFieldOf("actionbarMessage").xmap(REMOVE_BLANK, REMOVE_BLANK).forGetter(ChatRule::getActionBarMessageOptional)
        record_codec_builder::of(|o: &Fields| o.10.clone(), map_codec::xmap(codec::optional_field_of_opt(codec::string_codec(), "actionbarMessage"), |o: Option<String>| (**REMOVE_BLANK)(o), |o: &Option<String>| (**REMOVE_BLANK)(o.clone()))),
        // AnnouncementMessage.CODEC.optionalFieldOf("announcementMessage").forGetter(ChatRule::getAnnouncementMessageOptional)
        record_codec_builder::of(|o: &Fields| o.11.clone(), map_codec::xmap(codec::optional_field_of_opt(ANNOUNCEMENT_MESSAGE_CODEC.get().clone(), "announcementMessage"), |o: Option<compat::Shared<AnnouncementMessage>>| compat::RenderThread::new(o), |o: &compat::RenderThread<Option<compat::Shared<AnnouncementMessage>>>| o.get().clone())),
        // ToastMessage.CODEC.optionalFieldOf("toastMessage").forGetter(ChatRule::getToastMessageOptional)
        record_codec_builder::of(|o: &Fields| o.12.clone(), map_codec::xmap(codec::optional_field_of_opt(TOAST_MESSAGE_CODEC.get().clone(), "toastMessage"), |o: Option<compat::Shared<ToastMessage>>| compat::RenderThread::new(o), |o: &compat::RenderThread<Option<compat::Shared<ToastMessage>>>| o.get().clone())),
        // SoundEvent.DIRECT_CODEC.optionalFieldOf("customSound").forGetter(ChatRule::getCustomSoundOptional)
        record_codec_builder::of(|o: &Fields| o.13.clone(), codec::optional_field_of_opt(mcrust::net::minecraft::sounds::sound_event::direct_codec(), "customSound")),
        |a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13| (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13),
    ));
    codec::xmap(
        record,
        // (name, enabled, ..) -> new ChatRule(name, enabled, .., chatMessage.orElse(null), actionBarMessage.orElse(null), announcementMessage.orElse(null), toastMessage.orElse(null), customSound.orElse(null))
        |(name, enabled, is_partial_match, is_regex, is_ignore_case, include_formatting, filter, valid_locations, hide_message, chat_message, action_bar_message, announcement_message, toast_message, custom_sound): Fields| {
            ChatRule::new_14(name, enabled, is_partial_match, is_regex, is_ignore_case, include_formatting, filter, valid_locations, hide_message, chat_message, action_bar_message, announcement_message.get().clone(), toast_message.get().clone(), custom_sound)
        },
        |rule: &compat::Shared<ChatRule>| (
            ChatRule::get_name(rule),
            ChatRule::get_enabled(rule),
            compat::read(rule, |t| t.is_partial_match),
            compat::read(rule, |t| t.is_regex),
            compat::read(rule, |t| t.is_ignore_case),
            compat::read(rule, |t| t.include_formatting),
            ChatRule::get_filter(rule),
            ChatRule::get_valid_locations(rule),
            ChatRule::get_hide_message(rule),
            ChatRule::get_chat_message_optional(rule),
            ChatRule::get_action_bar_message_optional(rule),
            compat::RenderThread::new(ChatRule::get_announcement_message_optional(rule)),
            compat::RenderThread::new(ChatRule::get_toast_message_optional(rule)),
            ChatRule::get_custom_sound_optional(rule),
        ),
    )
});
/// java: public static final Codec<List<ChatRule>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<ChatRule>>>> = compat::MainThread::new(|| codec::list(CODEC.get_mut().clone()));
//   java: static final Codec<ToastMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				FlexibleItemStack.CODEC.optionalFieldOf("icon").forGetter(o -> o.icon),
//   java: 				Codec.STRING.fieldOf("message").forGetter(o -> o.message),
//   java: 				Codec.LONG.fieldOf("displayDuration").forGetter(o -> o.displayDuration)
//   java: 		).apply(instance, ToastMessage::new));
/// java: static final Codec<ToastMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( FlexibleItemStack.CODEC.optionalFieldOf("icon").forGetter(o -> o.icon), Codec.STRING.fieldOf("message").f
pub static TOAST_MESSAGE_CODEC: compat::MainThread<CodecRef<compat::Shared<ToastMessage>>> = compat::MainThread::new(|| {
    // port shape: the record is over the fields' values (the icon handle in a render-thread cell), `xmap`ped to the
    // message (see `CODEC`)
    type Fields = (compat::RenderThread<Option<compat::Shared<FlexibleItemStack>>>, String, i64);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply3(
        // FlexibleItemStack.CODEC.optionalFieldOf("icon").forGetter(o -> o.icon)
        record_codec_builder::of(|o: &Fields| o.0.clone(), map_codec::xmap(codec::optional_field_of_opt(flexible_item_stack::CODEC.get().clone(), "icon"), |o: Option<compat::Shared<FlexibleItemStack>>| compat::RenderThread::new(o), |o: &compat::RenderThread<Option<compat::Shared<FlexibleItemStack>>>| o.get().clone())),
        // Codec.STRING.fieldOf("message").forGetter(o -> o.message)
        record_codec_builder::of(|o: &Fields| o.1.clone(), codec::field_of(codec::string_codec(), "message")),
        // Codec.LONG.fieldOf("displayDuration").forGetter(o -> o.displayDuration)
        record_codec_builder::of(|o: &Fields| o.2, codec::field_of(codec::long_codec(), "displayDuration")),
        |icon, message, display_duration| (icon, message, display_duration),
    ));
    codec::xmap(
        record,
        |(icon, message, display_duration): Fields| ToastMessage::new(icon.get().clone(), message, display_duration),
        |toast: &compat::Shared<ToastMessage>| compat::read(toast, |t| (compat::RenderThread::new(t.icon.clone()), t.message.clone(), t.display_duration)),
    )
});
//   java: static final Codec<AnnouncementMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.fieldOf("message").forGetter(o -> o.message),
//   java: 				Codec.LONG.fieldOf("displayDuration").forGetter(o -> o.displayDuration)
//   java: 		).apply(instance, AnnouncementMessage::new));
/// java: static final Codec<AnnouncementMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("message").forGetter(o -> o.message), Codec.LONG.fieldOf("displayDuration").f
pub static ANNOUNCEMENT_MESSAGE_CODEC: compat::MainThread<CodecRef<compat::Shared<AnnouncementMessage>>> = compat::MainThread::new(|| {
    // port shape: the record is over the fields' values, `xmap`ped to the message (see `CODEC`)
    type Fields = (String, i64);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply2(
        // Codec.STRING.fieldOf("message").forGetter(o -> o.message)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(codec::string_codec(), "message")),
        // Codec.LONG.fieldOf("displayDuration").forGetter(o -> o.displayDuration)
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::long_codec(), "displayDuration")),
        |message, display_duration| (message, display_duration),
    ));
    codec::xmap(
        record,
        |(message, display_duration): Fields| AnnouncementMessage::new(message, display_duration),
        |announcement: &compat::Shared<AnnouncementMessage>| compat::read(announcement, |t| (t.message.clone(), t.display_duration)),
    )
});

/// java: public class ChatRule
#[derive(Clone)]
pub struct ChatRule {
    /// java: private String name;
    pub name: String,
    /// java: private @Nullable Pattern pattern;
    pub pattern: Option<compat::regex::Pattern>,
    /// java: private boolean enabled;
    pub enabled: bool,
    /// java: private boolean isPartialMatch;
    pub is_partial_match: bool,
    /// java: private boolean isRegex;
    pub is_regex: bool,
    /// java: private boolean isIgnoreCase;
    pub is_ignore_case: bool,
    /// java: private boolean includeFormatting;
    pub include_formatting: bool,
    /// java: private String filter;
    pub filter: String,
    /// java: private EnumSet<Location> validLocations;
    pub valid_locations: std::collections::HashSet<Location>,
    /// java: private boolean hideMessage;
    pub hide_message: bool,
    /// java: private @Nullable String chatMessage;
    pub chat_message: Option<String>,
    /// java: private @Nullable String actionBarMessage;
    pub action_bar_message: Option<String>,
    /// java: private @Nullable AnnouncementMessage announcementMessage;
    pub announcement_message: Option<compat::Shared<AnnouncementMessage>>,
    /// java: private @Nullable ToastMessage toastMessage;
    pub toast_message: Option<compat::Shared<ToastMessage>>,
    /// java: private @Nullable SoundEvent customSound;
    pub custom_sound: Option<SoundEvent>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ChatRule {
    /// java: protected ChatRule()
    pub fn new() -> compat::Shared<Self> {
        let __init_name = "New Rule".to_string();
        let __init_enabled = true;
        let __init_is_partial_match = false;
        let __init_is_regex = false;
        let __init_is_ignore_case = true;
        let __init_include_formatting = false;
        let __init_filter = "".to_string();
        let __init_valid_locations = std::collections::HashSet::<_>::new();
        let __init_hide_message = true;
        let __init_chat_message = None;
        let __init_action_bar_message = None;
        let __init_announcement_message = None;
        let __init_toast_message = None;
        let __init_custom_sound = None;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            name: __init_name,
            pattern: None,
            enabled: __init_enabled,
            is_partial_match: __init_is_partial_match,
            is_regex: __init_is_regex,
            is_ignore_case: __init_is_ignore_case,
            include_formatting: __init_include_formatting,
            filter: __init_filter,
            valid_locations: __init_valid_locations,
            hide_message: __init_hide_message,
            chat_message: __init_chat_message,
            action_bar_message: __init_action_bar_message,
            announcement_message: __init_announcement_message,
            toast_message: __init_toast_message,
            custom_sound: __init_custom_sound,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: ChatRule(String name, boolean enabled, boolean isPartialMatch, boolean isRegex, boolean isIgnoreCase, boolean includeFormatting, String filter, EnumSet<Location> validLocations, boolean hideMessage, @
    pub fn new_14(mut name: String, mut enabled: bool, mut is_partial_match: bool, mut is_regex: bool, mut is_ignore_case: bool, mut include_formatting: bool, mut filter: String, mut valid_locations: std::collections::HashSet<Location>, mut hide_message: bool, mut chat_message: Option<String>, mut action_bar_message: Option<String>, mut announcement_message: Option<compat::Shared<AnnouncementMessage>>, mut toast_message: Option<compat::Shared<ToastMessage>>, mut custom_sound: Option<SoundEvent>) -> compat::Shared<Self> {
        let __init_name = name.clone();
        let __init_enabled = enabled;
        let __init_is_partial_match = is_partial_match;
        let __init_is_regex = is_regex;
        let __init_is_ignore_case = is_ignore_case;
        let __init_include_formatting = include_formatting;
        let __init_filter = filter.clone();
        let __init_valid_locations = valid_locations.clone();
        let __init_hide_message = hide_message;
        let __init_chat_message = chat_message.clone();
        let __init_action_bar_message = action_bar_message.clone();
        let __init_announcement_message = announcement_message.clone();
        let __init_toast_message = toast_message.clone();
        let __init_custom_sound = custom_sound.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            name: __init_name,
            pattern: None,
            enabled: __init_enabled,
            is_partial_match: __init_is_partial_match,
            is_regex: __init_is_regex,
            is_ignore_case: __init_is_ignore_case,
            include_formatting: __init_include_formatting,
            filter: __init_filter,
            valid_locations: __init_valid_locations,
            hide_message: __init_hide_message,
            chat_message: __init_chat_message,
            action_bar_message: __init_action_bar_message,
            announcement_message: __init_announcement_message,
            toast_message: __init_toast_message,
            custom_sound: __init_custom_sound,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: private static EnumSet<Location> encodeString(String string)
    pub fn encode_string(string: &str) -> std::collections::HashSet<Location> {
        if string.is_empty() {
            return std::collections::HashSet::<_>::new();
        }
        if string.contains("!") {
            //   java: return EnumSet.complementOf(
            //   java: 				Arrays.stream(string.split(", ?"))
            //   java: 						.filter(s1 -> s1.startsWith("!")) // Filter out the non-negated locations because the negation of any element in the list already implies those non-negated locations being valid.
            //   java: 						.map(s -> s.substring(1)) // Skip the `!`
            //   java: 						.map(Location::fromFriendlyName)
            //   java: 						.collect(CollectionUtils.enumSetCollector(Location.class))
            //   java: 		);
            let negated: std::collections::HashSet<Location> = compat::regex::split(string, ", ?").iter().filter(|s1| s1.starts_with("!")).map(|s| compat::substring(s, 1)).map(|s| Location::from_friendly_name(&s)).collect();
            return Location::values().into_iter().filter(|location| !negated.contains(location)).collect();
        }
        //   java: return Arrays.stream(string.split(", ?"))
        //   java: 				.map(Location::fromFriendlyName)
        //   java: 				.collect(CollectionUtils.enumSetCollector(Location.class));
        return compat::regex::split(string, ", ?").iter().map(|s| Location::from_friendly_name(s)).collect();
    }

    /// java: protected String getName()
    pub fn get_name(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// java: protected void setName(String name)
    pub fn set_name(__this: &compat::Shared<Self>, mut name: String) {
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
    }

    /// java: protected boolean getEnabled()
    pub fn get_enabled(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.enabled);
    }

    /// java: protected void setEnabled(boolean enabled)
    pub fn set_enabled(__this: &compat::Shared<Self>, mut enabled: bool) {
        { let __v = enabled; compat::write(__this, |__t| __t.enabled = __v); }
    }

    /// java: protected boolean getPartialMatch()
    pub fn get_partial_match(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.is_partial_match);
    }

    /// java: protected void setPartialMatch(boolean partialMatch)
    pub fn set_partial_match(__this: &compat::Shared<Self>, mut partial_match: bool) {
        { let __v = partial_match; compat::write(__this, |__t| __t.is_partial_match = __v); }
    }

    /// java: protected boolean getRegex()
    pub fn get_regex(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.is_regex);
    }

    /// java: protected void setRegex(boolean regex)
    pub fn set_regex(__this: &compat::Shared<Self>, mut regex: bool) {
        { let __v = regex; compat::write(__this, |__t| __t.is_regex = __v); }
        { let __v = None; compat::write(__this, |__t| __t.pattern = __v); }
    }

    /// java: protected boolean getIgnoreCase()
    pub fn get_ignore_case(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.is_ignore_case);
    }

    /// java: protected void setIgnoreCase(boolean ignoreCase)
    pub fn set_ignore_case(__this: &compat::Shared<Self>, mut ignore_case: bool) {
        { let __v = ignore_case; compat::write(__this, |__t| __t.is_ignore_case = __v); }
        { let __v = None; compat::write(__this, |__t| __t.pattern = __v); }
    }

    /// java: protected boolean getIncludeFormatting()
    pub fn get_include_formatting(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.include_formatting);
    }

    /// java: protected void setIncludeFormatting(boolean includeFormatting)
    pub fn set_include_formatting(__this: &compat::Shared<Self>, mut include_formatting: bool) {
        { let __v = include_formatting; compat::write(__this, |__t| __t.include_formatting = __v); }
    }

    /// java: protected String getFilter()
    pub fn get_filter(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.filter.clone());
    }

    /// java: protected void setFilter(String filter)
    pub fn set_filter(__this: &compat::Shared<Self>, mut filter: String) {
        { let __v = filter.clone(); compat::write(__this, |__t| __t.filter = __v); }
        { let __v = None; compat::write(__this, |__t| __t.pattern = __v); }
    }

    /// java: protected boolean getHideMessage()
    pub fn get_hide_message(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.hide_message);
    }

    /// java: protected void setHideMessage(boolean hideMessage)
    pub fn set_hide_message(__this: &compat::Shared<Self>, mut hide_message: bool) {
        { let __v = hide_message; compat::write(__this, |__t| __t.hide_message = __v); }
    }

    /// java: @Nullable String getActionBarMessage()
    pub fn get_action_bar_message(__this: &compat::Shared<Self>) -> Option<String> {
        return compat::read(__this, |__t| __t.action_bar_message.clone());
    }

    /// java: private Optional<String> getActionBarMessageOptional()
    pub fn get_action_bar_message_optional(__this: &compat::Shared<Self>) -> Option<String> {
        return ChatRule::get_action_bar_message(__this);
    }

    /// java: void setActionBarMessage(@Nullable String actionBarMessage)
    pub fn set_action_bar_message(__this: &compat::Shared<Self>, mut action_bar_message: Option<String>) {
        if action_bar_message.is_some() && compat::is_blank(&action_bar_message.as_ref().unwrap()) {
            action_bar_message = None;
        }
        { let __v = action_bar_message.clone(); compat::write(__this, |__t| __t.action_bar_message = __v); }
    }

    /// java: @Nullable String getChatMessage()
    pub fn get_chat_message(__this: &compat::Shared<Self>) -> Option<String> {
        return compat::read(__this, |__t| __t.chat_message.clone());
    }

    /// java: private Optional<String> getChatMessageOptional()
    pub fn get_chat_message_optional(__this: &compat::Shared<Self>) -> Option<String> {
        return ChatRule::get_chat_message(__this);
    }

    /// java: void setChatMessage(@Nullable String chatMessage)
    pub fn set_chat_message(__this: &compat::Shared<Self>, mut chat_message: Option<String>) {
        if chat_message.is_some() && compat::is_blank(&chat_message.as_ref().unwrap()) {
            chat_message = None;
        }
        { let __v = chat_message.clone(); compat::write(__this, |__t| __t.chat_message = __v); }
    }

    /// java: @Nullable AnnouncementMessage getAnnouncementMessage()
    pub fn get_announcement_message(__this: &compat::Shared<Self>) -> Option<compat::Shared<AnnouncementMessage>> {
        return compat::read(__this, |__t| __t.announcement_message.clone());
    }

    /// java: private Optional<AnnouncementMessage> getAnnouncementMessageOptional()
    pub fn get_announcement_message_optional(__this: &compat::Shared<Self>) -> Option<compat::Shared<AnnouncementMessage>> {
        return ChatRule::get_announcement_message(__this);
    }

    /// java: void setAnnouncementMessage(@Nullable AnnouncementMessage announcementMessage)
    pub fn set_announcement_message(__this: &compat::Shared<Self>, mut announcement_message: Option<compat::Shared<AnnouncementMessage>>) {
        { let __v = announcement_message.clone(); compat::write(__this, |__t| __t.announcement_message = __v); }
    }

    /// java: @Nullable ToastMessage getToastMessage()
    pub fn get_toast_message(__this: &compat::Shared<Self>) -> Option<compat::Shared<ToastMessage>> {
        return compat::read(__this, |__t| __t.toast_message.clone());
    }

    /// java: private Optional<ToastMessage> getToastMessageOptional()
    pub fn get_toast_message_optional(__this: &compat::Shared<Self>) -> Option<compat::Shared<ToastMessage>> {
        return ChatRule::get_toast_message(__this);
    }

    /// java: void setToastMessage(@Nullable ToastMessage toastMessage)
    pub fn set_toast_message(__this: &compat::Shared<Self>, mut toast_message: Option<compat::Shared<ToastMessage>>) {
        { let __v = toast_message.clone(); compat::write(__this, |__t| __t.toast_message = __v); }
    }

    /// java: protected @Nullable SoundEvent getCustomSound()
    pub fn get_custom_sound(__this: &compat::Shared<Self>) -> Option<SoundEvent> {
        return compat::read(__this, |__t| __t.custom_sound.clone());
    }

    /// java: private Optional<SoundEvent> getCustomSoundOptional()
    pub fn get_custom_sound_optional(__this: &compat::Shared<Self>) -> Option<SoundEvent> {
        return ChatRule::get_custom_sound(__this);
    }

    /// java: protected void setCustomSound(@Nullable SoundEvent customSound)
    pub fn set_custom_sound(__this: &compat::Shared<Self>, mut custom_sound: Option<SoundEvent>) {
        { let __v = custom_sound.clone(); compat::write(__this, |__t| __t.custom_sound = __v); }
    }

    /// java: protected EnumSet<Location> getValidLocations()
    pub fn get_valid_locations(__this: &compat::Shared<Self>) -> std::collections::HashSet<Location> {
        return compat::read(__this, |__t| __t.valid_locations.clone());
    }

    /// java: protected void setValidLocations(EnumSet<Location> validLocations)
    pub fn set_valid_locations(__this: &compat::Shared<Self>, mut valid_locations: std::collections::HashSet<Location>) {
        { let __v = valid_locations.clone(); compat::write(__this, |__t| __t.valid_locations = __v); }
    }

    /// java: private void compilePattern(String filterText)
    pub fn compile_pattern(__this: &compat::Shared<Self>, filter_text: &str) {
        if compat::read(__this, |__t| __t.pattern.clone()).is_some() {
            return;
        }
        // java: try {..} catch (PatternSyntaxException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let __v = Some(compat::regex::Pattern::compile(filter_text)); compat::write(__this, |__t| __t.pattern = __v); }
        }));
        if let Err(_) = __result {
            { let __v = false; compat::write(__this, |__t| __t.enabled = __v); }
            let client = compat::minecraft();
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.config.chat.chatRules.invalidRegex", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(compat::read(__this, |__t| __t.name.clone()))])).to_string()); });
        }
    }

    /// java: protected Match isMatch(String inputString)
    pub fn is_match(__this: &compat::Shared<Self>, input_string: &str) -> Match {
        if !(compat::read(__this, |__t| __t.enabled)) {
            return Match::no_match();
        }
        let mut test_string: String = (if compat::read(__this, |__t| __t.is_ignore_case) { input_string.to_lowercase() } else { input_string.to_string() });
        let mut test_filter: String = (if compat::read(__this, |__t| __t.is_ignore_case) { __this.lock().borrow().filter.to_lowercase() } else { compat::read(__this, |__t| __t.filter.clone()) });
        if compat::is_blank(&test_filter) {
            return Match::no_match();
        }
        let mut r#match: Match;
        if compat::read(__this, |__t| __t.is_regex) {
            ChatRule::compile_pattern(__this, &test_filter);
            if compat::read(__this, |__t| __t.pattern.clone()).is_none() {
                return Match::no_match();
            }
            let mut matcher: compat::regex::Matcher = __this.lock().borrow().pattern.as_ref().unwrap().matcher(&test_string);
            if compat::read(__this, |__t| __t.is_partial_match) {
                if matcher.find() {
                    r#match = Match::of_regex(&matcher);
                } else {
                    return Match::no_match();
                }
            } else {
                if matcher.matches() {
                    r#match = Match::of_regex(&matcher);
                } else {
                    return Match::no_match();
                }
            }
        } else {
            if compat::read(__this, |__t| __t.is_partial_match) {
                if test_string.contains(&test_filter) {
                    r#match = Match::of_string();
                } else {
                    return Match::no_match();
                }
            } else {
                if test_filter == test_string {
                    r#match = Match::of_string();
                } else {
                    return Match::no_match();
                }
            }
        }
        if __this.lock().borrow().valid_locations.is_empty() {
            return r#match;
        }
        if (__this.lock().borrow().valid_locations.len() as i32) == 1 && __this.lock().borrow().valid_locations.contains(&Location::Unknown) {
            return r#match;
        }
        return (if __this.lock().borrow().valid_locations.contains(&Utils::get_location()) { r#match.clone() } else { Match::no_match() });
    }

    /// java: @Override public final boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        let Some(chat_rule) = o.downcast_ref::<compat::Shared<ChatRule>>() else {
            return false;
        };
        let this = self.self_ref.upgrade().unwrap();
        // java: getEnabled() == .. && getName().equals(..) && getValidLocations().equals(..) && Objects.equals(getChatMessage(), ..) && ..
        return ChatRule::get_enabled(&this) == ChatRule::get_enabled(chat_rule)
            && ChatRule::get_partial_match(&this) == ChatRule::get_partial_match(chat_rule)
            && ChatRule::get_regex(&this) == ChatRule::get_regex(chat_rule)
            && ChatRule::get_ignore_case(&this) == ChatRule::get_ignore_case(chat_rule)
            && ChatRule::get_include_formatting(&this) == ChatRule::get_include_formatting(chat_rule)
            && ChatRule::get_hide_message(&this) == ChatRule::get_hide_message(chat_rule)
            && ChatRule::get_name(&this) == ChatRule::get_name(chat_rule)
            && ChatRule::get_filter(&this) == ChatRule::get_filter(chat_rule)
            && ChatRule::get_valid_locations(&this) == ChatRule::get_valid_locations(chat_rule)
            && ChatRule::get_chat_message(&this) == ChatRule::get_chat_message(chat_rule)
            && ChatRule::get_action_bar_message(&this) == ChatRule::get_action_bar_message(chat_rule)
            && objects_equals(&ChatRule::get_announcement_message(&this), &ChatRule::get_announcement_message(chat_rule), |a, b| compat::read(a, |a| compat::read(b, |b| a.fields_equal(b))))
            && objects_equals(&ChatRule::get_toast_message(&this), &ChatRule::get_toast_message(chat_rule), |a, b| compat::read(a, |a| compat::read(b, |b| a.fields_equal(b))))
            && ChatRule::get_custom_sound(&this) == ChatRule::get_custom_sound(chat_rule);
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        let this = self.self_ref.upgrade().unwrap();
        // java: return Objects.hash(getName(), getEnabled(), getPartialMatch(), getRegex(), getIgnoreCase(), getIncludeFormatting(),
        //   getFilter(), getValidLocations(), getHideMessage(), getChatMessage(), getActionBarMessage(), getActionBarMessage(),
        //   getAnnouncementMessage(), getToastMessage(), getCustomSound());
        return objects_hash(&[
            compat::string_hash_code(&ChatRule::get_name(&this)),
            boolean_hash_code(ChatRule::get_enabled(&this)),
            boolean_hash_code(ChatRule::get_partial_match(&this)),
            boolean_hash_code(ChatRule::get_regex(&this)),
            boolean_hash_code(ChatRule::get_ignore_case(&this)),
            boolean_hash_code(ChatRule::get_include_formatting(&this)),
            compat::string_hash_code(&ChatRule::get_filter(&this)),
            // `AbstractSet.hashCode`: the sum of the elements' hash codes
            ChatRule::get_valid_locations(&this).iter().fold(0i32, |h, l| h.wrapping_add(compat::hash_code(l))),
            boolean_hash_code(ChatRule::get_hide_message(&this)),
            ChatRule::get_chat_message(&this).as_deref().map(compat::string_hash_code).unwrap_or(0),
            ChatRule::get_action_bar_message(&this).as_deref().map(compat::string_hash_code).unwrap_or(0),
            ChatRule::get_action_bar_message(&this).as_deref().map(compat::string_hash_code).unwrap_or(0),
            ChatRule::get_announcement_message(&this).map(|m| compat::read(&m, |m| m.hash_code())).unwrap_or(0),
            ChatRule::get_toast_message(&this).map(|m| compat::read(&m, |m| m.hash_code())).unwrap_or(0),
            ChatRule::get_custom_sound(&this).map(|e| sound_event_hash_code(&e)).unwrap_or(0),
        ]);
    }

    /// java: @Override @GenToString public native String toString();
    /// The body the build's `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' toString -
    /// `ChatRule[f=String.valueOf(f), ..]` over every non-static, non-transient field in declaration order.
    pub fn to_string(&self) -> String {
        format!(
            "ChatRule[name={}, pattern={}, enabled={}, isPartialMatch={}, isRegex={}, isIgnoreCase={}, includeFormatting={}, filter={}, validLocations={}, hideMessage={}, chatMessage={}, actionBarMessage={}, announcementMessage={}, toastMessage={}, customSound={}]",
            self.name,
            // `Pattern.toString()` is its source
            self.pattern.as_ref().map_or_else(|| "null".to_string(), |p| p.pattern().to_string()),
            self.enabled,
            self.is_partial_match,
            self.is_regex,
            self.is_ignore_case,
            self.include_formatting,
            self.filter,
            enum_set_to_string(&self.valid_locations),
            self.hide_message,
            compat::opt_str(&self.chat_message),
            compat::opt_str(&self.action_bar_message),
            self.announcement_message.as_ref().map_or_else(|| "null".to_string(), |m| compat::read(m, |m| m.to_string())),
            self.toast_message.as_ref().map_or_else(|| "null".to_string(), |m| compat::read(m, |m| m.to_string())),
            self.custom_sound.as_ref().map_or_else(|| "null".to_string(), sound_event_to_string),
        )
    }

}

impl std::fmt::Display for ChatRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ChatRule::to_string(self))
    }
}

/// java: protected record Match(boolean matches, Optional<Matcher> matcher)
#[derive(Clone, Debug)]
pub struct Match {
    /// java: boolean matches
    pub matches: bool,
    /// java: Optional<Matcher> matcher
    pub matcher: Option<compat::regex::Matcher>,
}

impl Match {
    /// java: the canonical record constructor
    pub fn new(mut matches: bool, mut matcher: Option<compat::regex::Matcher>) -> Self {
        Self { matches, matcher }
    }

    /// java: the record accessor `matches()`
    pub fn matches(&self) -> bool {
        self.matches
    }

    /// java: the record accessor `matcher()`
    pub fn matcher(&self) -> Option<compat::regex::Matcher> {
        self.matcher.clone()
    }

    /// java: protected static Match noMatch()
    pub fn no_match() -> Match {
        return Match::new(false, None);
    }

    /// java: protected static Match ofString()
    pub fn of_string() -> Match {
        return Match::new(true, None);
    }

    /// java: protected static Match ofRegex(Matcher matcher)
    pub fn of_regex(matcher: &compat::regex::Matcher) -> Match {
        return Match::new(true, Some(matcher.clone()));
    }

    /// java: protected String insertCaptureGroups(String replaceMessage)
    pub fn insert_capture_groups(&self, replace_message: &str) -> String {
        if !self.matches || self.matcher.is_none() {
            return replace_message.to_string();
        }
        let mut sb: String = String::new();
        let mut m: compat::regex::Matcher = self.matcher.clone().unwrap();
        m.reset();
        if !m.find() {
            return replace_message.to_string();
        }
        // java: try {..} catch (IndexOutOfBoundsException | IllegalArgumentException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): Matcher.appendReplacement
            //   java: m.appendReplacement(sb, replaceMessage);
        }));
        if let Err(_) = __result {
            return replace_message.to_string();
        }
        return compat::substring(&sb, m.start(0));
    }

}

/// java: static class ToastMessage
#[derive(Clone)]
pub struct ToastMessage {
    /// java: Optional<FlexibleItemStack> icon;
    pub icon: Option<compat::Shared<FlexibleItemStack>>,
    /// java: String message;
    pub message: String,
    /// java: long displayDuration;
    pub display_duration: i64,
}

impl ToastMessage {
    /// java: ToastMessage(Optional<FlexibleItemStack> icon, String message, long displayDuration)
    pub fn new(mut icon: Option<compat::Shared<FlexibleItemStack>>, mut message: String, mut display_duration: i64) -> compat::Shared<Self> {
        let __init_message = message.clone();
        let __init_icon = icon.clone();
        let __init_display_duration = display_duration;
        let __this = compat::shared(Self {
            icon: __init_icon,
            message: __init_message,
            display_duration: __init_display_duration,
        });
        __this
    }

    /// java: ToastMessage()
    pub fn new_0() -> compat::Shared<Self> {
        let __this = Self::new(Some(FlexibleItemStack::new2(items::items().get("painting"))), "".to_string(), ((1000) as i64) as i64);
        __this
    }

    /// java: @Override @GenToString public native String toString();
    /// The body the build's `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' toString -
    /// `ToastMessage[f=String.valueOf(f), ..]` over every non-static, non-transient field in declaration order.
    pub fn to_string(&self) -> String {
        // `Optional.toString()`: "Optional[" + value + "]" / "Optional.empty"; `FlexibleItemStack` keeps `Object.toString`
        let icon = match &self.icon {
            Some(icon) => format!("Optional[de.hysky.skyblocker.utils.FlexibleItemStack@{:x}]", std::sync::Arc::as_ptr(icon) as *const () as usize as u32),
            None => "Optional.empty".to_string(),
        };
        format!("ToastMessage[icon={}, message={}, displayDuration={}]", icon, self.message, self.display_duration)
    }

    /// java: @Override @GenEquals public native boolean equals(Object obj);
    /// (the body `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' equals - the same class, every field
    /// `Objects.equals`)
    pub fn equals(&self, obj: &compat::Object) -> bool {
        let Some(other) = obj.downcast_ref::<compat::Shared<ToastMessage>>() else {
            return false;
        };
        return compat::read(other, |other| self.fields_equal(other));
    }

    /// The generated equals' field comparison: `icon` (an `Optional<FlexibleItemStack>`, a class that keeps
    /// `Object.equals`: identity), `message`, `displayDuration`.
    pub fn fields_equal(&self, other: &ToastMessage) -> bool {
        objects_equals(&self.icon, &other.icon, |a, b| std::sync::Arc::ptr_eq(a, b)) && self.message == other.message && self.display_duration == other.display_duration
    }

    /// java: @Override @GenHashCode public native int hashCode();
    /// (`ObjectMethods`' hashCode: `31 * h + hash(field)` over the fields in order, from 0)
    pub fn hash_code(&self) -> i32 {
        let icon: i32 = self.icon.as_ref().map(|i| std::sync::Arc::as_ptr(i) as *const () as usize as i32).unwrap_or(0);
        return [icon, compat::string_hash_code(&self.message), long_hash_code(self.display_duration)].iter().fold(0i32, |h, e| h.wrapping_mul(31).wrapping_add(*e));
    }

}

impl std::fmt::Display for ToastMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ToastMessage::to_string(self))
    }
}

/// java: static class AnnouncementMessage
#[derive(Clone)]
pub struct AnnouncementMessage {
    /// java: String message;
    pub message: String,
    /// java: long displayDuration;
    pub display_duration: i64,
}

impl AnnouncementMessage {
    /// java: AnnouncementMessage(String message, long displayDuration)
    pub fn new(mut message: String, mut display_duration: i64) -> compat::Shared<Self> {
        let __init_message = message.clone();
        let __init_display_duration = display_duration;
        let __this = compat::shared(Self {
            message: __init_message,
            display_duration: __init_display_duration,
        });
        __this
    }

    /// java: AnnouncementMessage()
    pub fn new_0() -> compat::Shared<Self> {
        let __this = Self::new("".to_string(), (SkyblockerConfigManager::get().chat.chat_rule_config.announcement_length as i64).wrapping_mul(50));
        __this
    }

    /// java: @Override @GenToString public native String toString();
    /// The body the build's `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' toString -
    /// `AnnouncementMessage[f=String.valueOf(f), ..]` over every non-static, non-transient field in declaration order.
    pub fn to_string(&self) -> String {
        format!("AnnouncementMessage[message={}, displayDuration={}]", self.message, self.display_duration)
    }

    /// java: @Override @GenEquals public native boolean equals(Object obj);
    /// (the body `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' equals)
    pub fn equals(&self, obj: &compat::Object) -> bool {
        let Some(other) = obj.downcast_ref::<compat::Shared<AnnouncementMessage>>() else {
            return false;
        };
        return compat::read(other, |other| self.fields_equal(other));
    }

    /// The generated equals' field comparison: `message`, `displayDuration`.
    pub fn fields_equal(&self, other: &AnnouncementMessage) -> bool {
        self.message == other.message && self.display_duration == other.display_duration
    }

    /// java: @Override @GenHashCode public native int hashCode();
    /// (`ObjectMethods`' hashCode: `31 * h + hash(field)` over the fields in order, from 0)
    pub fn hash_code(&self) -> i32 {
        return [compat::string_hash_code(&self.message), long_hash_code(self.display_duration)].iter().fold(0i32, |h, e| h.wrapping_mul(31).wrapping_add(*e));
    }

}

impl std::fmt::Display for AnnouncementMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", AnnouncementMessage::to_string(self))
    }
}

/// java: `Objects.equals(a, b)` over two nullable references, with the class's own `equals` for two non-null ones
fn objects_equals<T: ?Sized>(a: &Option<compat::Shared<T>>, b: &Option<compat::Shared<T>>, equals: impl Fn(&compat::Shared<T>, &compat::Shared<T>) -> bool) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b) || equals(a, b),
        _ => false,
    }
}

/// java: `Objects.hash(values...)` - `Arrays.hashCode`: `31 * h + hashCode(e)` from 1
fn objects_hash(hashes: &[i32]) -> i32 {
    hashes.iter().fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(*e))
}

/// java: `Boolean.hashCode(b)`
fn boolean_hash_code(b: bool) -> i32 {
    if b { 1231 } else { 1237 }
}

/// java: `Long.hashCode(v)` - `(int) (v ^ (v >>> 32))`
fn long_hash_code(v: i64) -> i32 {
    (v ^ ((v as u64) >> 32) as i64) as i32
}

/// java: `SoundEvent.hashCode()` (a record: `31 * location.hashCode() + fixedRange.hashCode()`)
fn sound_event_hash_code(e: &mcrust::net::minecraft::sounds::sound_event::SoundEvent) -> i32 {
    let fixed_range: i32 = e.fixed_range().map(|f| f.to_bits() as i32).unwrap_or(0);
    e.location().java_hash_code().wrapping_mul(31).wrapping_add(fixed_range)
}

/// java: `EnumSet.toString()` (`AbstractCollection.toString`): "[" + the elements in ordinal order, ", "-joined + "]"
fn enum_set_to_string(set: &std::collections::HashSet<Location>) -> String {
    let mut elements: Vec<&Location> = set.iter().collect();
    elements.sort();
    format!("[{}]", elements.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", "))
}

/// java: `SoundEvent.toString()` (a record: `SoundEvent[location=.., fixedRange=Optional[..]]`)
fn sound_event_to_string(e: &SoundEvent) -> String {
    let fixed_range = match e.fixed_range() {
        Some(f) => format!("Optional[{}]", mcrust::javacompat::java_float_to_string(f)),
        None => "Optional.empty".to_string(),
    };
    format!("SoundEvent[location={}, fixedRange={}]", e.location(), fixed_range)
}
