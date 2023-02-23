mod theme;
mod works;

use zoon::{named_color::*, *};

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
    Column::new()
        .s(Align::center())
        .s(Gap::both(20))
        .s(Clip::both())
        .s(Font::new().color_signal(theme::primary_text_color()))
        .s(Background::new().color_signal(theme::secondary_background_color()))
        .item(page_content())
}

fn page_content() -> impl Element {
    El::new().child_signal(page().signal().map(|page| match page {
        Page::Home => El::new().child("Hello💚").into_raw_element(),
        Page::Work => works::page_content().into_raw_element(),
        Page::Unknown => El::new().child("Unknown").into_raw_element(),
    }))
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
