#![allow(non_snake_case)]
pub mod BaseComponents;
pub mod CollectionDisplay;
pub mod Collections;
pub mod MainPage;
pub mod SideBar;

use dioxus::desktop::tao::dpi::PhysicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::html::input_data::MouseButton;
use rust_lib::api::shared_resources::collection::{Collection, CollectionId};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tailwind_fuse::*;

use dioxus::prelude::*;
use log::LevelFilter;
use BaseComponents::{subModalProps, ComponentPointer, Switcher};

use crate::BaseComponents::{Button, ContentType, FillMode, Modal, Roundness};
use crate::CollectionDisplay::CollectionDisplay;
use crate::Collections::Collections;
use crate::MainPage::MainPage;
use crate::SideBar::SideBar;

pub const HOME: &str = manganis::mg!(file("./public/home.svg"));
pub const EXPLORE: &str = manganis::mg!(file("./public/explore.svg"));
pub const SIDEBAR_COLLECTION: &str = manganis::mg!(file("./public/collections.svg"));
pub const ARROW_RIGHT: &str = manganis::mg!(file("./public/keyboard_arrow_right.svg"));
pub const SIM_CARD: &str = manganis::mg!(file("./public/sim_card_download.svg"));
pub const TAILWIND_STR_: &str = manganis::mg!(file("./public/tailwind.css"));

/// `(Pages)`: Current active page
/// `Option<Pages>`: Previous page
static ACTIVE_PAGE: GlobalSignal<(Pages, Option<Pages>)> =
    GlobalSignal::new(|| (Pages::MainPage, None));
pub static TOP_LEVEL_COMPONENT: GlobalSignal<Vec<ComponentPointer<subModalProps>>> =
    GlobalSignal::new(Vec::new);

use rust_lib::api::shared_resources::entry::{self, DOWNLOAD_PROGRESS, STORAGE};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_window(
            WindowBuilder::new()
                .with_decorations(true)
                .with_inner_size(PhysicalSize::new(1600, 920)),
        )
        .with_menu(None);
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Pages {
    MainPage,
    Explore,
    Collections,
    DownloadProgress,
    CollectionPage(Arc<str>),
}

impl Pages {
    fn new_collection_page(s: CollectionId) -> Self {
        let s = s.0;
        Self::CollectionPage(s.into())
    }
}

impl Switcher for Pages {
    fn hashed_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn compare(&self) -> bool {
        &ACTIVE_PAGE().0 == self
    }

    fn switch_active_to_self(&self) {
        let prev = ACTIVE_PAGE().0;
        if &prev != self {
            ACTIVE_PAGE.write().1 = Some(prev);
        }
        ACTIVE_PAGE.write().0 = self.clone();
    }
}

impl Pages {
    pub fn slide_in_id(&self) -> String {
        format!("flyinout-{}", self.to_string())
    }

    /// Applies slide-in animations to HTML elements based on data attributes.
    ///
    /// This function dynamically applies CSS animations to elements within a webpage
    /// using Tailwind CSS-defined animations. It targets elements with a specific class ('.group')
    /// and adjusts their styles according to their data attributes.
    ///
    /// ## Attributes
    /// The function expects HTML elements to have certain attributes and structure:
    /// * Top level element should have the class `group`.
    /// * Each `group` element should contain at least one child element with an `id` that is acquired by `self.slide_in_id()`
    ///
    /// ## Data Attributes
    /// * `data-prev`: This attribute specifies whether the element was the previous element in a
    ///   sequence. If `true`, the `slideRight` animation is applied.
    /// * `data-selected`: This attribute indicates if the element is the currently selected one.
    ///   If `true`, the `slideLeft` animation is applied.
    ///
    /// ## Usage
    /// To use this function, ensure that your HTML elements are set up correctly with the
    /// required `id` and data attributes. Additionally, for most use cases involving animations or transitions,
    /// it's essential to manage the positioning context correctly:
    ///
    /// - The parent container should have a **relative** positioning to serve as the positioning context for its children.
    /// - Child elements, which are the targets of the animations, should be styled with **absolute** positioning to overlay
    ///   within the relative container seamlessly.
    ///
    /// It is crucial to call this function at the start of each component's lifecycle to properly initialize
    /// the animations.
    ///
    /// Here is an example element setup:
    ///
    /// ```rust
    /// fn Component() {
    ///     Pages::DownloadProgress.apply_slide_in();
    ///     rsx! {
    ///         div {
    ///             "data-selected": selected.to_string(),
    ///             "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
    ///             div { class: "w-full min-h-screen relative",
    ///                 div { class: "absolute inset-0 z-0 min-h-full min-w-full", id: Pages::DownloadProgress.slide_in_id(), LayoutContainer { DownloadProgress {} } }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn apply_slide_in(&self) -> anyhow::Result<()> {
        eval(
            r#"
                function applyStyles(dataValue) {
                    const groups = document.querySelectorAll('.group');            
                    groups.forEach(group => {
                        const prev = group.getAttribute('data-prev') === dataValue;
                        const selected = group.getAttribute('data-selected') === dataValue;
                        const target = group.querySelector('#flyinout-' + dataValue);

                        // Reset styles first
                        target.style.insetInlineStart = '';
                        target.style.zIndex = '0';
                        target.style.display = 'none';
                        target.style.animation = '';

                        if (prev) {
                            target.style.insetInlineStart = '100dvw';
                            target.style.zIndex = '100';
                            target.style.display = 'block';                        
                            target.style.animation = 'slideRight 500ms';
                        } else if (selected) {
                            target.style.zIndex = '50';
                            target.style.display = 'block';                        
                            target.style.animation = 'slideLeft 500ms';
                        }
                    });
                }
                applyStyles(await dioxus.recv());
            "#,
        )
        .send(self.to_string().into())
        .map_err(|x| anyhow::anyhow!("{x:?}"))
    }
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Self::MainPage => "main-page".into(),
            Self::Explore => "explore".into(),
            Self::Collections => "collections".into(),
            Self::DownloadProgress => "progress".into(),
            Self::CollectionPage(x) => {
                let mut hasher = DefaultHasher::new();
                x.hash(&mut hasher);
                let hash = hasher.finish();
                format!("collection-page-{hash}")
            }
        }
    }
}

pub static COLLECT: GlobalSignal<Vec<Collection>> =
    GlobalSignal::new(|| Collection::scan().unwrap_or_default());

#[component]
fn App() -> Element {
    let error_active = use_signal(|| true);
    spawn(async move {
        let mut last = None;
        loop {
            let collections = STORAGE.collections.clone().read_owned().await.to_owned();
            if last.as_ref() != Some(&collections) {
                *COLLECT.write() = collections;
            }
            last = Some(COLLECT());
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    spawn(async move {
        let versions = rust_lib::api::backend_exclusive::vanilla::version::get_versions()
            .await
            .unwrap();
        let version = versions.into_iter().find(|x| x.id == "1.20.1").unwrap();
        entry::create_collection("weird test", version, None, None)
            .await
            .unwrap();
    });

    rsx! {
        div { class: "font-['GenSenRounded TW'] bg-deep-background min-h-screen min-w-full font-display leading-normal",
            {
                TOP_LEVEL_COMPONENT().into_iter().map(|x| (x.pointer)(x.props))
            },
            ErrorBoundary {
                handle_error: move |error| { rsx! {
                    Modal { active: error_active, name: "error_modal", close_on_outer_click: false,
                        div {
                            div { class: "flex flex-col space-y-3",
                                div { class: "text-red text-2xl font-bold",
                                    "Hmm, something went wrong. Please copy the following error to the developer."
                                }
                                Button {
                                    roundness: Roundness::Pill,
                                    extended_css_class: "text-[13px] font-bold",
                                    string_placements: rsx! { "{error} " },
                                    fill_mode: FillMode::Fit,
                                    clickable: false
                                }
                            }
                        }
                    }
                } },
                div { class: "[&_*]:transform-gpu", Layout {} }
            }
        }
    }
}

#[component]
fn Layout() -> Element {
    let selected = ACTIVE_PAGE().0;
    let prev = ACTIVE_PAGE().1;

    let collections = COLLECT();

    for collection in &collections {
        Pages::new_collection_page(collection.get_collection_id())
            .apply_slide_in()
            .unwrap();
    }
    Pages::DownloadProgress.apply_slide_in().throw()?;
    rsx! {
        div {
            class: "w-screen inline-flex self-stretch group flex overflow-hidden",
            "data-selected": selected.to_string(),
            "data-prev": prev.map_or_else(String::new, |x| x.to_string()),
            onmousedown: move |x| {
                if let Some(x) = x.data().trigger_button() {
                    if x == MouseButton::Fourth  {
                        if let Some(x) = ACTIVE_PAGE().1 {
                            x.switch_active_to_self();
                        }
                    }
                }
            },
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
                div {
                    class: "absolute inset-0 z-0 min-h-full min-w-full",
                    id: Pages::DownloadProgress.slide_in_id(),
                    LayoutContainer { DownloadProgress {} }
                }
                CollectionContainer {
                    collections,
                }
            }
        }
    }
}

#[component]
fn CollectionContainer(collections: ReadOnlySignal<Vec<Collection>>) -> Element {
    rsx! {
        for (name, collection) in collections().into_iter().map(|x| (x.get_collection_id(), x)) {
            div {
                class: "absolute inset-0 z-0 min-h-full min-w-full",
                id: Pages::new_collection_page(name).slide_in_id(),
                LayoutContainer { extended_class: "p-0",
                    CollectionDisplay { collection }
                }
            }
        }
    }
}

/// Does dynmaic rendering
/// do not wrap the children in another div
#[component]
fn LayoutContainer(children: Element, #[props(default)] extended_class: String) -> Element {
    rsx! {
        div { class: tw_merge!("bg-background min-h-screen rounded-xl p-8 min-w-full", extended_class),
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
    let progress = use_resource(move || async move {
        use_reactive(&*DOWNLOAD_PROGRESS, |x| x)()
            .get_all()
            .await
            .unwrap()
    });
    let display = progress().map(|x| {
        x.into_iter()
            .map(|x| ContentType::text(x.percentages.to_string()).align_left())
            .collect::<Vec<_>>()
    });
    rsx! {
        div {
            if let Some(p) = display {
                Button {
                    roundness: Roundness::Top,
                    string_placements: p,
                }
            }
        }
    }
}
