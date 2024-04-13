#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod MainPage;
pub mod Collections;

use std::time::Duration;
use tailwind_fuse::*;

use dioxus::prelude::*;
use log::LevelFilter;

use crate::BaseComponents::{Alignment, Button, ContentType, Roundness};
use crate::MainPage::{MainPage, COLLECTION_PIC};
use crate::Collections::Collections;

pub const HOME: &str = manganis::mg!(file("./public/home.svg"));
pub const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
pub const SIDEBAR_COLLECTION: &str = manganis::mg!(file("./public/collections.svg"));
pub const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));
pub const SIM_CARD: &str = manganis::mg!(file("./public/sim_card_download.svg"));
pub const TAILWIND_STR_: &str = manganis::mg!(file("./public/tailwind.css"));



/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static ACTIVE: GlobalSignal<(Pages, Option<Pages>)> = GlobalSignal::new(|| (Pages::MainPage, None));

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        // .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
        .with_menu(None);
    // let cfg = dioxus::web::Config::new();
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    // LaunchBuilder::new().with_cfg(cfg).launch(App);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Pages {
    MainPage,
    Explore,
    Collections,
    DownloadProgress,
}

pub fn switch_active(x: Pages) {
    let prev = ACTIVE().0;
    if prev != x {
        ACTIVE.write().1 = Some(prev);
    }
    ACTIVE.write().0 = x;
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Self::MainPage => "main-page",
            Self::Explore => "explore",
            Self::Collections => "collections",
            Self::DownloadProgress => "progress",
        }
        .into()
    }
}

#[component]
fn App() -> Element {
    rsx! {
        // link { rel: "stylesheet", href: "public/tailwind.css" }
        div { class: "bg-deep-background min-h-screen min-w-full",
            div { class: "[&_*]:transform-gpu", Layout {} }
        }
    }
}


#[component]
fn Layout() -> Element {
    let selected = ACTIVE().0;
    let prev = ACTIVE().1;
    rsx! {
        div {
            class: "w-screen inline-flex self-stretch mt-[20px] group flex overflow-hidden",
            "data-selected": selected.to_string(),
            "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
            SideBar {}
            div { class: "w-full min-h-screen relative *:overflow-scroll",
                div { class: "absolute inset-0 z-0 min-h-full animation-[main-page^slideDown^explore^slideOutUp] animation-[main-page^slideDown^collections^slideOutUp]",
                    LayoutContainer { MainPage {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full animation-[explore^slideUp^main-page^slideOutDown] animation-[explore^slideDown^collections^slideOutUp]",
                    LayoutContainer { Explore {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full animation-[collections^slideUp^explore^slideOutDown] animation-[collections^slideUp^main-page^slideOutDown]",
                    LayoutContainer { Collections {} }
                }
                div { class: "absolute inset-0 z-0 min-h-full min-w-full flyinout-[progress]",
                    LayoutContainer { DownloadProgress {} }
                }
            }
        }
    }
}

/// Does dynmaic rendering
/// do not wrap the children in another div
#[component]
fn LayoutContainer(children: Element) -> Element {
    rsx! {
        div { class: "bg-background min-h-screen rounded-xl p-8 min-w-full",
            div { class: "flex flex-col space-y-[20px] transition-all xl:items-center xl:*:justify-center xl:*:max-w-[1180px] xl:*:w-full",
                {children}
            }
        }
    }
}

#[component]
fn Explore() -> Element {
    rsx! {
        div {
            Button {
                roundness: Roundness::Top,
                string_placements: vec![
                    ContentType::text("Explore").align_left(),
                    ContentType::text("thumbsup").align_right(),
                ]
            }
        }
    }
}

#[component]
fn DownloadProgress() -> Element {
    rsx! {
        Button {
            roundness: Roundness::Top,
            string_placements: vec![
                ContentType::text("Progress").align_left(),
                ContentType::text("stop").align_right(),
            ]
        }
    }
}



#[component]
fn SideBar() -> Element {
    let mut expanded = use_signal(|| false);
    let delayed_expanded = use_resource(move || async move {
        if expanded() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        } else {
            // tokio::time::sleep(Duration::from_millis(100)).await;
        }
        expanded()
    });
    let fat_button = |roundness, svg, string: &str, active, onclick: Option<EventHandler>| {
        rsx! {
            div {
                Button {
                    roundness,
                    string_placements: vec![
                        ContentType::svg(svg).align_left(),
                        ContentType::text(string).css("group-aria-busy:hidden").align_right(),
                    ],
                    signal: active,
                    onclick,
                    extended_css_class: "bg-background group-aria-expanded:pr-5"
                }
            }
        }
    };
    let onclick = move |()| {
        switch_active(Pages::Collections);
        expanded.toggle();
    };
    let p = rsx! {
        div { class: "transition-all",
            {ContentType::svg(HOME).css("hidden group-aria-expanded:block").get_element()},
            div { class: "flex items-center space-x-0",
                div { class: "flex space-x-[-20px]",
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-50 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()},
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-40 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()},
                    {ContentType::image(COLLECTION_PIC.to_string())
                        .css(
                            "z-30 w-10 h-10 object-cover shrink-0 inline-flex justify-center items-center rounded-full border-2 border-zinc-900 group-aria-expanded:hidden"
                        ).get_element()}
                }
                {
                    ContentType::svg(ARROW_RIGHT).css("flex items-center w-[25px] h-[25px] *:w-[25px] *:h-[25px] block group-aria-expanded:hidden").get_element()
                }
            }
        }
        div { class: tw_merge!(Alignment::Right.get_alignment_class(), "group-aria-busy:hidden"), {ContentType::text("我的錦集").css("text-lime-300").get_element()} }
    };
    rsx! {
        div { class: "flex flex-col place-content-start mx-5",
            div {
                class: "w-[300px] space-y-5 transition-all ease-linear duration-500 aria-expanded:w-[80px] group",
                aria_expanded: !expanded(),
                aria_busy: !delayed_expanded().unwrap_or(false),
                // top
                div { class: "flex flex-col space-y-1",
                    {fat_button(Roundness::Top, HOME, "首頁", Pages::MainPage, None)},
                    {fat_button(Roundness::None, EXPLORE, "探索", Pages::Explore, None)},
                    {fat_button(Roundness::Bottom, SIDEBAR_COLLECTION, "收藏庫", Pages::Collections, Some(onclick.into()))}
                }
                // middle
                div { class: "flex flex-col space-y-1",
                    Button { roundness: Roundness::Top, string_placements: p, extended_css_class: "bg-background" }
                    Button {
                        roundness: Roundness::None,
                        string_placements: vec![
                            ContentType::image(COLLECTION_PIC.to_string())
                                .css(
                                    "transition-all w-[50px] h-[50px] object-cover inline-flex items-center rounded-[15px] border-2 border-zinc-900 group-aria-expanded:w-20 group-aria-expanded:h-20",
                                )
                                .align_left(),
                            ContentType::text("新的收藏").align_right().css("group-aria-busy:hidden"),
                        ],
                        extended_css_class: "bg-background transition-all delay-[25ms] group-aria-expanded:w-20 group-aria-expanded:min-h-20 group-aria-expanded:p-0"
                    }
                }
                // bottom
                div { class: "flex flex-col space-y-1",
                    Button {
                        roundness: Roundness::Top,
                        string_placements: vec![
                            ContentType::svg(SIM_CARD).align_left(),
                            ContentType::text("返回")
                                .align_right()
                                .css(
                                    "hidden group-aria-[busy=false]:group-aria-selected/active:block group-aria-busy:hidden",
                                ),
                            ContentType::text("無下載佇列")
                                .align_right()
                                .css("group-aria-selected/active:hidden group-aria-busy:hidden text-hint"),
                        ],
                        signal: Pages::DownloadProgress,
                        extended_css_class: "bg-background group/active items-center",
                        onclick: move |()| {
                            let prev = ACTIVE().1;
                            if ACTIVE().0 == Pages::DownloadProgress {
                                if let Some(prev) = prev {
                                    switch_active(prev);
                                    ACTIVE.write().1 = Some(Pages::DownloadProgress);
                                }
                            } else {
                                switch_active(Pages::DownloadProgress);
                            }
                        }
                    }
                }
            }
        }
    }
}
