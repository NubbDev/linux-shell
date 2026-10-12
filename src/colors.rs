use iced::{Color, color};

pub const BACKGROUND: Color = color!(0x282c34);
pub const TEXT: Color = color!(0xabb2bf);
pub const PRIMARY: Color = color!(0x61afef);
pub const SUCCESS: Color = color!(0x98c379);
pub const DANGER: Color = color!(0xe06c75);
pub const WARNING: Color = color!(0xe5c07b);

pub const COLOR1: Color = color!(0xbe5046);
pub const COLOR2: Color = color!(0x56b6c2);
pub const COLOR3: Color = color!(0xd19a66);
pub const COLOR4: Color = color!(0x393e48);
pub const COLOR5: Color = color!(0x979eab);

pub fn mix_colors(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_rgba(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
