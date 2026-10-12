//! Ported from de.hysky.skyblocker.utils.ColorUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;

/// java: public class ColorUtils
#[derive(Clone)]
pub struct ColorUtils {}

impl ColorUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static float[] getFloatComponents(int color)
    pub fn get_float_components(mut color: i32) -> Vec<f32> {
        return vec![argb::red_float(color), argb::green_float(color), argb::blue_float(color)];
    }

    /// java: public static float[] getFloatComponents(DyeColor dye)
    pub fn get_float_components_dye_color(dye: &DyeColor) -> Vec<f32> {
        return ColorUtils::get_float_components(dye.get_texture_diffuse_color());
    }

    /// java: public static int percentToColor(float pcnt)
    pub fn percent_to_color(mut pcnt: f32) -> i32 {
        return mth::hsv_to_rgb((pcnt / (300 as f32)), (1) as f32, (1) as f32);
    }

    /// java: public static int interpolate(int firstColor, int secondColor, double percentage)
    pub fn interpolate(mut first_color: i32, mut second_color: i32, mut percentage: f64) -> i32 {
        return OkLabColor::interpolate(first_color, second_color, ((percentage) as f32));
    }

    /// java: public static int interpolate(double percentage, int... colors)
    pub fn interpolate_2(mut percentage: f64, colors: &Vec<i32>) -> i32 {
        let mut color_count: i32 = (colors.len() as i32);
        if color_count == 0 {
            return 0;
        }
        if color_count == 1 || percentage <= (0 as f64) {
            return colors[(0) as usize];
        }
        if percentage >= (1 as f64) {
            return colors[(color_count.wrapping_sub(1)) as usize];
        }
        let mut scaled_percentage: f64 = (percentage * (color_count.wrapping_sub(1) as f64));
        let mut index: i32 = ((scaled_percentage.floor()) as i32);
        let mut remainder: f64 = (scaled_percentage - (index as f64));
        return ColorUtils::interpolate(colors[(index) as usize], colors[(index.wrapping_add(1)) as usize], remainder);
    }

    /// java: public static boolean isBright(int color)
    pub fn is_bright(mut color: i32) -> bool {
        let mut components: Vec<f32> = ColorUtils::get_float_components(color);
        let mut red: f32 = components[(0) as usize];
        let mut green: f32 = components[(1) as usize];
        let mut blue: f32 = components[(2) as usize];
        let mut luminance: f64 = ColorUtils::luminance(red, green, blue);
        let mut white_contrast: f64 = ColorUtils::contrast_ratio(luminance, 1.0);
        let mut black_contrast: f64 = ColorUtils::contrast_ratio(luminance, 0.0);
        return (white_contrast < black_contrast);
    }

    /// java: private static double luminance(float red, float green, float blue)
    pub fn luminance(mut red: f32, mut green: f32, mut blue: f32) -> f64 {
        let mut r: f64 = (if red <= 0.04045f32 { ((red / 12.92f32)) as f64 } else { (((red + 0.055f32) / 1.055f32) as f64).powf((2.4f32 as f64)) });
        let mut g: f64 = (if green <= 0.04045f32 { ((green / 12.92f32)) as f64 } else { (((green + 0.055f32) / 1.055f32) as f64).powf((2.4f32 as f64)) });
        let mut b: f64 = (if blue <= 0.04045f32 { ((blue / 12.92f32)) as f64 } else { (((blue + 0.055f32) / 1.055f32) as f64).powf((2.4f32 as f64)) });
        // TODO(fabric2seer): Math.fma
        //   java: return Math.fma(0.2126, r, Math.fma(0.7152, g, 0.0722 * b));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static double contrastRatio(double background, double content)
    pub fn contrast_ratio(mut background: f64, mut content: f64) -> f64 {
        let mut brightest: f64 = background.max(content);
        let mut darkest: f64 = background.min(content);
        return ((brightest + 0.05) / (darkest + 0.05));
    }

}
