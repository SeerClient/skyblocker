//! Ported from de.hysky.skyblocker.utils.render.title.Title by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class Title
#[derive(Clone)]
pub struct Title {
    /// java: private MutableComponent text;
    pub text: Option<MutableComponent>,
    /// java: protected float x = -1;
    pub x: f32,
    /// java: protected float y = -1;
    pub y: f32,
}

impl Title {
    /// java: public Title(@Translatable String textKey, ChatFormatting formatting)
    pub fn new(mut text_key: String, mut formatting: ChatFormatting) -> compat::Shared<Self> {
        let __this = Self::new_1(component::translatable(&text_key).with_style_format(formatting));
        __this
    }

    /// java: public Title(MutableComponent text)
    pub fn new_1(mut text: MutableComponent) -> compat::Shared<Self> {
        let __init_text = Some(text.clone());
        let __this = compat::shared(Self {
            text: __init_text,
            x: ((-1) as f32) as f32,
            y: ((-1) as f32) as f32,
        });
        __this
    }

    /// java: public MutableComponent getText()
    pub fn get_text(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.text.clone()).unwrap();
    }

    /// java: public Title setText(MutableComponent text)
    pub fn set_text(__this: &compat::Shared<Self>, mut text: MutableComponent) -> compat::Shared<Title> {
        { let __v = Some(text.clone()); compat::write(__this, |__t| __t.text = __v); }
        return __this.clone();
    }

    /// java: protected boolean isDefaultPos()
    pub fn is_default_pos(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(__this, |__t| __t.x) == (-1 as f32) && compat::read(__this, |__t| __t.y) == (-1 as f32));
    }

    /// java: protected void resetPos()
    pub fn reset_pos(__this: &compat::Shared<Self>) {
        { let __v = ((-1) as f32) as f32; compat::write(__this, |__t| __t.x = __v); }
        { let __v = ((-1) as f32) as f32; compat::write(__this, |__t| __t.y = __v); }
    }

}
