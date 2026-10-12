//! Ported from de.hysky.skyblocker.utils.OkLabColor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: @SuppressWarnings("UnaryPlus") public class OkLabColor
#[derive(Clone)]
pub struct OkLabColor {}

impl OkLabColor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Lab linearSRGB2OkLab(float r, float g, float b)
    pub fn linear_srgb2_ok_lab(mut r: f32, mut g: f32, mut b: f32) -> Lab {
        // java: float l = Math.fma(0.4122214708f, r, Math.fma(0.5363325363f, g, 0.0514459929f * b));
        let mut l: f32 = 0.4122214708f32.mul_add(r, 0.5363325363f32.mul_add(g, 0.0514459929f32 * b));
        // java: float m = Math.fma(0.2119034982f, r, Math.fma(0.6806995451f, g, 0.1073969566f * b));
        let mut m: f32 = 0.2119034982f32.mul_add(r, 0.6806995451f32.mul_add(g, 0.1073969566f32 * b));
        // java: float s = Math.fma(0.0883024619f, r, Math.fma(0.2817188376f, g, 0.6299787005f * b));
        let mut s: f32 = 0.0883024619f32.mul_add(r, 0.2817188376f32.mul_add(g, 0.6299787005f32 * b));
        let mut l_: f32 = (((l as f64).cbrt()) as f32);
        let mut m_: f32 = (((m as f64).cbrt()) as f32);
        let mut s_: f32 = (((s as f64).cbrt()) as f32);
        // java: float L = Math.fma(0.2104542553f, l_, Math.fma(+0.7936177850f, m_, -0.0040720468f * s_));
        let mut l: f32 = 0.2104542553f32.mul_add(l_, 0.7936177850f32.mul_add(m_, -0.0040720468f32 * s_));
        // java: float A = Math.fma(1.9779984951f, l_, Math.fma(-2.4285922050f, m_, +0.4505937099f * s_));
        let mut a: f32 = 1.9779984951f32.mul_add(l_, (-2.4285922050f32).mul_add(m_, 0.4505937099f32 * s_));
        // java: float B = Math.fma(0.0259040371f, l_, Math.fma(+0.7827717662f, m_, -0.8086757660f * s_));
        let mut b: f32 = 0.0259040371f32.mul_add(l_, 0.7827717662f32.mul_add(m_, -0.8086757660f32 * s_));
        return Lab::new(l, a, b);
    }

    /// java: public static RGB okLab2LinearSRGB(float L, float A, float B)
    pub fn ok_lab2_linear_srgb(mut l: f32, mut a: f32, mut b: f32) -> RGB {
        let mut l_: f32 = ((l + (0.3963377774f32 * a)) + (0.2158037573f32 * b));
        let mut m_: f32 = ((l - (0.1055613458f32 * a)) - (0.0638541728f32 * b));
        let mut s_: f32 = ((l - (0.0894841775f32 * a)) - (1.291485548f32 * b));
        let mut l: f32 = ((l_ * l_) * l_);
        let mut m: f32 = ((m_ * m_) * m_);
        let mut s: f32 = ((s_ * s_) * s_);
        // java: float r = Math.fma(+4.0767416621f, l, Math.fma(-3.3077115913f, m, +0.2309699292f * s));
        let mut r: f32 = 4.0767416621f32.mul_add(l, (-3.3077115913f32).mul_add(m, 0.2309699292f32 * s));
        // java: float g = Math.fma(-1.2684380046f, l, Math.fma(+2.6097574011f, m, -0.3413193965f * s));
        let mut g: f32 = (-1.2684380046f32).mul_add(l, 2.6097574011f32.mul_add(m, -0.3413193965f32 * s));
        // java: float b = Math.fma(-0.0041960863f, l, Math.fma(-0.7034186147f, m, +1.7076147010f * s));
        let mut b: f32 = (-0.0041960863f32).mul_add(l, (-0.7034186147f32).mul_add(m, 1.7076147010f32 * s));
        return RGB::new(r, g, b);
    }

    /// java: private static float linearize(float channel)
    pub fn linearize(mut channel: f32) -> f32 {
        return (if channel <= 0.04045f32 { (channel / 12.92f32) } else { (((((channel + 0.055f32) / 1.055f32) as f64).powf((2.4f32 as f64))) as f32) });
    }

    /// java: private static float delinearize(float channel)
    pub fn delinearize(mut channel: f32) -> f32 {
        // java: return channel <= 0.0031308f ? channel * 12.92f : Math.fma(1.055f, (float) Math.pow(channel, 1.0f / 2.4f), -0.055f);
        return if channel <= 0.0031308f32 { channel * 12.92f32 } else { 1.055f32.mul_add((channel as f64).powf((1.0f32 / 2.4f32) as f64) as f32, -0.055f32) };
    }

    /// java: public static int interpolate(int firstColor, int secondColor, float progress)
    pub fn interpolate(mut first_color: i32, mut second_color: i32, mut progress: f32) -> i32 {
        let mut normalized_r1: f32 = (((first_color.wrapping_shr((16) as u32) & 0xFF) as f32) / 255.0f32);
        let mut normalized_g1: f32 = (((first_color.wrapping_shr((8) as u32) & 0xFF) as f32) / 255.0f32);
        let mut normalized_b1: f32 = (((first_color & 0xFF) as f32) / 255.0f32);
        let mut normalized_r2: f32 = (((second_color.wrapping_shr((16) as u32) & 0xFF) as f32) / 255.0f32);
        let mut normalized_g2: f32 = (((second_color.wrapping_shr((8) as u32) & 0xFF) as f32) / 255.0f32);
        let mut normalized_b2: f32 = (((second_color & 0xFF) as f32) / 255.0f32);
        let mut lab1: Lab = OkLabColor::linear_srgb2_ok_lab(OkLabColor::linearize(normalized_r1), OkLabColor::linearize(normalized_g1), OkLabColor::linearize(normalized_b1));
        let mut lab2: Lab = OkLabColor::linear_srgb2_ok_lab(OkLabColor::linearize(normalized_r2), OkLabColor::linearize(normalized_g2), OkLabColor::linearize(normalized_b2));
        // java: float L = Math.fma(progress, (lab2.l - lab1.l), lab1.l);
        let mut l: f32 = progress.mul_add(lab2.l - lab1.l, lab1.l);
        // java: float A = Math.fma(progress, (lab2.a - lab1.a), lab1.a);
        let mut a: f32 = progress.mul_add(lab2.a - lab1.a, lab1.a);
        // java: float B = Math.fma(progress, (lab2.b - lab1.b), lab1.b);
        let mut b: f32 = progress.mul_add(lab2.b - lab1.b, lab1.b);
        let mut rgb: RGB = OkLabColor::ok_lab2_linear_srgb(l, a, b);
        let mut r: i32 = compat::java_round_f32((OkLabColor::delinearize(rgb.r) * 255.0f32).clamp((0 as f32), (255 as f32)));
        let mut g: i32 = compat::java_round_f32((OkLabColor::delinearize(rgb.g) * 255.0f32).clamp((0 as f32), (255 as f32)));
        let mut b: i32 = compat::java_round_f32((OkLabColor::delinearize(rgb.b) * 255.0f32).clamp((0 as f32), (255 as f32)));
        return ((r.wrapping_shl((16) as u32) | g.wrapping_shl((8) as u32)) | b);
    }

}

/// java: public record Lab(float l, float a, float b)
#[derive(Clone, Debug, PartialEq)]
pub struct Lab {
    /// java: float l
    pub l: f32,
    /// java: float a
    pub a: f32,
    /// java: float b
    pub b: f32,
}

impl Lab {
    /// java: the canonical record constructor
    pub fn new(mut l: f32, mut a: f32, mut b: f32) -> Self {
        Self { l, a, b }
    }

    /// java: the record accessor `l()`
    pub fn l(&self) -> f32 {
        self.l
    }

    /// java: the record accessor `a()`
    pub fn a(&self) -> f32 {
        self.a
    }

    /// java: the record accessor `b()`
    pub fn b(&self) -> f32 {
        self.b
    }

    /// java: public Vec3 toVec3()
    pub fn to_vec3(&self) -> Vec3 {
        return Vec3::new((self.l) as f64, (self.a) as f64, (self.b) as f64);
    }

}

/// java: public record RGB(float r, float g, float b)
#[derive(Clone, Debug, PartialEq)]
pub struct RGB {
    /// java: float r
    pub r: f32,
    /// java: float g
    pub g: f32,
    /// java: float b
    pub b: f32,
}

impl RGB {
    /// java: the canonical record constructor
    pub fn new(mut r: f32, mut g: f32, mut b: f32) -> Self {
        Self { r, g, b }
    }

    /// java: the record accessor `r()`
    pub fn r(&self) -> f32 {
        self.r
    }

    /// java: the record accessor `g()`
    pub fn g(&self) -> f32 {
        self.g
    }

    /// java: the record accessor `b()`
    pub fn b(&self) -> f32 {
        self.b
    }

}
