//! Ported from de.hysky.skyblocker.utils.render.state.gui.GuiOutlinedTextRenderState by fabric2seer.
//!
//! The port's `GuiTextRenderState` is prepared when it is built (the game's divergence 1 in `gui_text_render_state.rs`:
//! its `ensurePrepared` hands back the record it was built with), so this subclass prepares in its constructor what
//! Java prepares in its first `ensurePrepared()` - the same two `PreparedText`s from the same fields - and the base
//! state carries the resolved `OutlineGlyphDrawable` (the outline's renderables, then the text's: its `visit` order).

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::font;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::font::PreparedText;
use mcrust::net::minecraft::client::gui::font::PreparedTextBuilder;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::GuiTextRenderState;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;

/// java: public class GuiOutlinedTextRenderState extends GuiTextRenderState
#[derive(Clone)]
pub struct GuiOutlinedTextRenderState {
    /// java: extends net.minecraft.client.renderer.state.gui.GuiTextRenderState (pose, scissor and the prepared glyphs
    /// the game draws: `ensurePrepared()`'s `OutlineGlyphDrawable`)
    pub base: GuiTextRenderState,
    /// java: GuiTextRenderState.text
    pub text: Seq,
    /// java: GuiTextRenderState.x / y / color / dropShadow / includeEmpty / backgroundColor
    pub x: i32,
    pub y: i32,
    pub color: i32,
    pub drop_shadow: bool,
    pub include_empty: bool,
    pub background_color: i32,
    /// java: private final int outlineColor;
    pub outline_color: i32,
    /// java: private @Nullable OutlineGlyphDrawable preparation;
    pub preparation: Option<OutlineGlyphDrawable>,
    /// java: private @Nullable ScreenRectangle bounds;
    pub bounds: Option<ScreenRectangle>,
    /// PERF (port-only, not a Java field): whether `base`'s resolved record is a pure function of the text, the
    /// origin, the two colours and the font sets (`compat::leftovers_resolve_gui_text_cacheable`) - so a caller
    /// may keep it for a later frame (`GuiHelper::outlined_text_kept`).
    pub cacheable: bool,
}

impl std::ops::Deref for GuiOutlinedTextRenderState {
    type Target = GuiTextRenderState;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GuiOutlinedTextRenderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GuiOutlinedTextRenderState {
    /// java: public GuiOutlinedTextRenderState(Font textRenderer, FormattedCharSequence orderedText, Matrix3x2f matrix, int x, int y, int color, int outlineColor, boolean shadow, boolean trackEmpty, ScreenRectangle clipBounds)
    pub fn new(text_renderer: &Font<'_>, ordered_text: Seq, matrix: Matrix3x2f, x: i32, y: i32, color: i32, outline_color: i32, shadow: bool, track_empty: bool, clip_bounds: Option<ScreenRectangle>) -> Self {
        // java: super(textRenderer, orderedText, matrix, x, y, color, 0, shadow, trackEmpty, clipBounds);
        let mut this = Self {
            base: GuiTextRenderState::new(mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::PreparedText::new(), matrix, clip_bounds.clone()),
            text: ordered_text,
            x,
            y,
            color,
            drop_shadow: shadow,
            include_empty: track_empty,
            background_color: 0,
            outline_color,
            preparation: None,
            bounds: None,
            cacheable: false,
        };
        // java: ensurePrepared() - see the file header
        let preparation = this.ensure_prepared(text_renderer);
        let (resolved, cacheable) = compat::leftovers_resolve_gui_text_cacheable(&[&preparation.outline, &preparation.text]);
        this.base = GuiTextRenderState::new(resolved, matrix, clip_bounds);
        this.cacheable = cacheable;
        this
    }

    /// `this.font.new PreparedTextBuilder(..)` fed `text.accept(..)` with `Font.getGlyph(codePoint, style)` per codepoint
    /// (`Font.prepareText(FormattedCharSequence, ..)` / the outline's sink)
    fn accept_glyph(font: &Font<'_>, builder: &mut PreparedTextBuilder, index: i32, style: &Style, code_point: u32) -> bool {
        let source = font::get_glyph_source(style.get_font());
        let metrics = source.glyph_info(font.set, code_point);
        builder.accept_glyph(index, style, code_point, metrics, source);
        true
    }

    /// java: private PreparedText prepareOutline()
    ///
    /// The text outline drawing code from the `TextRenderer`.
    pub fn prepare_outline(&self, font: &Font<'_>) -> PreparedText {
        // java: Font.PreparedTextBuilder drawer = this.font.new PreparedTextBuilder(0.0f, 0.0f, this.outlineColor, false, this.includeEmpty);
        let mut drawer = PreparedTextBuilder::new(0.0f32, 0.0f32, self.outline_color, 0, false);
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    let mut fs: [f32; 1] = [self.x as f32];
                    let k: i32 = i;
                    let l: i32 = j;
                    let y = self.y;
                    let outline_color = self.outline_color;
                    self.text.accept(&mut |index: i32, style: &Style, code_point: u32| {
                        let bl: bool = style.is_bold();
                        // java: BakedGlyph bakedGlyph = accessor.invokeGetGlyph(codePoint, style);
                        let source = font::get_glyph_source(style.get_font());
                        let info = source.glyph_info(font.set, code_point);
                        // java: drawer.x = fs[0] + k * bakedGlyph.info().getShadowOffset(); drawer.y = y + l * bakedGlyph.info().getShadowOffset();
                        compat::leftovers_prepared_text_builder_move_pen(&mut drawer, fs[0] + (k as f32) * info.shadow_offset, (y as f32) + (l as f32) * info.shadow_offset);
                        // java: fs[0] += bakedGlyph.info().getAdvance(bl);
                        fs[0] += font::advance(info.advance, bl, info.bold_offset);
                        // java: return drawer.accept(index, style.withColor(this.outlineColor), bakedGlyph);
                        drawer.accept_glyph(index, &style.with_color(outline_color), code_point, info, source);
                        true
                    });
                }
            }
        }
        // java: return drawer;
        drawer.build()
    }

    /// java: private PreparedText prepareText()
    pub fn prepare_text(&mut self, font: &Font<'_>) -> PreparedText {
        // java: PreparedText textPreparation = this.font.prepareText(this.text, (float) this.x, (float) this.y, this.color, this.dropShadow, this.includeEmpty, this.backgroundColor);
        let mut builder = PreparedTextBuilder::new(self.x as f32, self.y as f32, self.color, self.background_color, self.drop_shadow);
        self.text.accept(&mut |index: i32, style: &Style, code_point: u32| GuiOutlinedTextRenderState::accept_glyph(font, &mut builder, index, style, code_point));
        let text_preparation: PreparedText = builder.build();
        // java: ScreenRectangle screenRect = textPreparation.bounds();
        let screen_rect: Option<ScreenRectangle> = compat::leftovers_resolve_gui_text(&[&text_preparation]).bounds();
        if let Some(screen_rect) = screen_rect {
            let screen_rect = screen_rect.transform_max_bounds(&self.base.pose);
            self.bounds = if let Some(scissor) = self.base.scissor.as_ref() { scissor.intersection(screen_rect) } else { Some(screen_rect) };
        }
        text_preparation
    }

    /// java: @Override public PreparedText ensurePrepared()
    pub fn ensure_prepared(&mut self, font: &Font<'_>) -> OutlineGlyphDrawable {
        if self.preparation.is_none() {
            let outline = self.prepare_outline(font);
            let text = self.prepare_text(font);
            self.preparation = Some(OutlineGlyphDrawable::new(outline, text));
        }
        self.preparation.clone().expect("prepared above")
    }

    /// java: @Override public @Nullable ScreenRectangle bounds()
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        // java: this.ensurePrepared(); - done by the constructor (see the file header)
        self.bounds.clone()
    }
}

/// java: private record OutlineGlyphDrawable(PreparedText outline, PreparedText text) implements PreparedText
#[derive(Clone, Debug)]
pub struct OutlineGlyphDrawable {
    /// java: PreparedText outline
    pub outline: PreparedText,
    /// java: PreparedText text
    pub text: PreparedText,
}

impl OutlineGlyphDrawable {
    /// java: the canonical record constructor
    pub fn new(outline: PreparedText, text: PreparedText) -> Self {
        Self { outline, text }
    }

    /// java: the record accessor `outline()`
    pub fn outline(&self) -> PreparedText {
        self.outline.clone()
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> PreparedText {
        self.text.clone()
    }

    /// java: @Override public void visit(GlyphVisitor glyphDrawer) - `outline.visit(glyphDrawer); text.visit(glyphDrawer);`:
    /// the two records resolved, in that order, into the GUI's text renderables
    pub fn visit(&self) -> mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::PreparedText {
        compat::leftovers_resolve_gui_text(&[&self.outline, &self.text])
    }

    /// java: @Override public ScreenRectangle bounds()
    pub fn bounds(&self) -> ScreenRectangle {
        //Shouldn't need an implementation but if it eventually does its probably best to use the outline rect
        //since it should be the bigger one
        panic!("UnsupportedOperationException: {}", "For drawing only");
    }
}
