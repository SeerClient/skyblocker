//! Ported from de.hysky.skyblocker.utils.FunUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::cat_picture::CatPicture;
use mcrust::java::time::local_date::LocalDate;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;

/// java: private static final boolean FUN_DISABLED = Boolean.parseBoolean(System.getProperty("skyblocker.iAmABoringPersonAndHateFun", "false"));
pub static FUN_DISABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("skyblocker.iAmABoringPersonAndHateFun").ok().unwrap().eq_ignore_ascii_case("true"));
/// java: public static final Random RANDOM = new Random();
pub static RANDOM: compat::MainThread<compat::Random> = compat::MainThread::new(|| compat::Random::new());
/// java: private static final String OBFUSCATED_NAME = "Corruptedblocker";
pub const OBFUSCATED_NAME: &str = "Corruptedblocker";
/// java: public static final List<String> FUN_NAMES = List.of( "Skibidiblocker", "Skiblocker", OBFUSCATED_NAME, "AAAAAAAAblocker", "camelCaseBlocker" );
pub static FUN_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Skibidiblocker".to_string(), "Skiblocker".to_string(), OBFUSCATED_NAME.to_string(), "AAAAAAAAblocker".to_string(), "camelCaseBlocker".to_string()]);

/// java: public class FunUtils
#[derive(Clone)]
pub struct FunUtils {}

impl FunUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Component getRandomFunName()
    pub fn get_random_fun_name() -> MutableComponent {
        // TODO(fabric2seer): `.nextInt(..)` on a Random
        //   java: return createUnpleasantGradient(FUN_NAMES.get(RANDOM.nextInt(FUN_NAMES.size())));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean shouldEnableFun()
    pub fn should_enable_fun() -> bool {
        if *FUN_DISABLED {
            return false;
        }
        let mut date: LocalDate = LocalDate::now();
        return (date.get_month_value() == 4 && date.get_day_of_month() == 1);
    }

    /// java: public static boolean shouldEnableChristmasFun()
    pub fn should_enable_christmas_fun() -> bool {
        if *FUN_DISABLED {
            return false;
        }
        let mut date: LocalDate = LocalDate::now();
        return (date.get_month_value() == 12 && date.get_day_of_month() == 24 || date.get_day_of_month() == 25);
    }

    /// java: public static boolean shouldEnableSpookyFun()
    pub fn should_enable_spooky_fun() -> bool {
        if *FUN_DISABLED {
            return false;
        }
        let mut date: LocalDate = LocalDate::now();
        return (date.get_month_value() == 10 && date.get_day_of_month() == 31);
    }

    /// java: public static Component createUnpleasantGradient(String string)
    pub fn create_unpleasant_gradient(string: &str) -> MutableComponent {
        let mut obfuscated: bool = (string == OBFUSCATED_NAME);
        let mut component: MutableComponent = component::empty();
        let mut half_size: i32 = (compat::str_len(&string)).wrapping_div(2);
        for i in 0..compat::str_len(&string) {
            let mut c: char = compat::char_at(&string, i);
            let mut start_color: i32 = (if i >= half_size { 0xFD2EF6 } else { 0x22D71D });
            let mut end_color: i32 = (if i >= half_size { 0x9B5300 } else { 0xFD2EF6 });
            let mut progress: f32 = ((i.wrapping_rem(half_size) as f32) / ((half_size) as f32)).clamp((0 as f32), (1 as f32));
            component = component.append((component::literal(&c.to_string()).with_color(argb::srgb_lerp(progress, start_color, end_color))).with_style_fn(|style| style.with_obfuscated(Some(obfuscated))));
        }
        return component;
    }

    /// java: public static void recipeBookEasterEgg(String search)
    pub fn recipe_book_easter_egg(search: &str) {
        if search == "hestalks" {
            CatPicture::recipe_book();
        }
    }

}
