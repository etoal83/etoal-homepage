mod dm_waving_dots_square;
mod mdn_clock_example;

use std::borrow::Cow;
use std::str::FromStr;
use strum::EnumString;
use strum_macros::Display;
use zoon::{named_color::*, *};

#[derive(Clone, Copy, Debug, Display, PartialEq, PartialOrd, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Slug {
    Root,
    MdnClockExample,
    DmWavingDotsSquare,
}

impl RouteSegment for Slug {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Slug::from_str(segment).ok()
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn slug() -> &'static Mutable<Slug> {
    Mutable::new(Slug::Root)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_slug(new_slug: Slug) {
    slug().set_neq(new_slug)
}

// ------ ------
//     View
// ------ ------

pub fn page_content() -> impl Element {
    El::new().child_signal(slug().signal().map(|slug| match slug {
        Slug::Root => El::new().child("WorkRoot").into_raw_element(),
        Slug::MdnClockExample => mdn_clock_example::page_content().into_raw_element(),
        Slug::DmWavingDotsSquare => dm_waving_dots_square::page_content().into_raw_element(),
    }))
}
