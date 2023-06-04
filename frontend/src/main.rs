mod theme;
mod works;

use zoon::*;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Page {
    Home,
    Work,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn page() -> &'static Mutable<Page> {
    Mutable::new(Page::Unknown)
}

// ------ ------
//   Commands
// ------ ------

fn set_page(new_page: Page) {
    page().set_neq(new_page)
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Stack::new()
        .s(Width::fill())
        .s(Height::screen())
        .s(Background::new().color_signal(theme::secondary_background_color()))
        .layer(Column::new()
            .s(Height::screen())
            .s(Font::new().color_signal(theme::primary_text_color()))
            .item(header())
            .item(page_content()))
}

fn page_content() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Gap::both(20))
        .s(Clip::both())
        .child_signal(page().signal().map(|page| match page {
            Page::Home => El::new().child("Hello💚").into_raw_element(),
            Page::Work => works::page_content().into_raw_element(),
            Page::Unknown => El::new().child("Unknown").into_raw_element(),
        }))
}

fn header() -> impl Element {
    Row::with_tag(Tag::Nav)
        .s(Height::exact(64))
        .s(Padding::new().x(12))
        .item(logo())
}

fn logo() -> impl Element {
    Link::new()
        .to("/")
        .label(Row::new()
            .item(logo_svg())
            .item(Row::new()
                .s(Padding::new().left(12))
                .s(Font::new()
                    .size(24)
                    .weight(FontWeight::Bold)
                    .family([
                        FontFamily::new("Futura"),
                        FontFamily::new("Century Gothic"),
                        FontFamily::new("CenturyGothic"),
                        FontFamily::new("Apple Sans"),
                        FontFamily::SansSerif,
                    ]))
                .item("EtoAlium")
            )
        )
}

fn logo_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
    RawSvgEl::from_markup(include_str!("../../public/logo.svg"))
        .unwrap_throw()
        .attr("width", "50")
        .attr("height", "40")
        .style_signal("fill", theme::primary_text_color())
}

// ------ ------
//    Routes
// ------ ------

#[route]
#[derive(Clone)]
enum Route {
    #[route()]
    Home,

    #[route("works")]
    Works,
    #[route("works", slug)]
    Work { slug: works::Slug },
}

#[static_ref]
fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async {
        let Some(route) = route else { return set_page(Page::Unknown) };

        match route {
            Route::Home => set_page(Page::Home),
            Route::Works => {
                set_page(Page::Work);
                works::set_slug(works::Slug::Root);
            }
            Route::Work { slug } => {
                set_page(Page::Work);
                works::set_slug(slug);
            }
        }
    })
}

// ------ ------
//     Start
// ------ ------

fn main() {
    router();
    start_app("app", root);
}
