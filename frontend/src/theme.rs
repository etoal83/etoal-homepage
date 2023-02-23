use catppuccin::{Colour, Flavour};
use hsluv::hex_to_hsluv;
use zoon::*;

#[static_ref]
pub fn theme() -> &'static Mutable<Theme> {
    Mutable::new(Theme::Dark)
}

#[derive(Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
}

fn hsluv(colour: Colour) -> HSLuv {
    let (h, s, l) = hex_to_hsluv(&colour.hex());

    HSLuv::new_unchecked(h, s, l, 100.)
}

// ------ colors ------

macro_rules! assign_color {
    ($color:ident => $schema:ident) => {
        pub fn $color() -> impl Signal<Item = HSLuv> {
            theme().signal().map(|theme| match theme {
                Theme::Light => hsluv(Flavour::Latte.$schema()),
                Theme::Dark => hsluv(Flavour::Mocha.$schema()),
            })
        }
    };
}

assign_color!(primary_text_color => text);
assign_color!(primary_background_color => base);
assign_color!(secondary_background_color => mantle);
