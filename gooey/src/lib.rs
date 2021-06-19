pub mod frontends {
    #[cfg(feature = "frontend-browser")]
    #[doc(inline)]
    pub use gooey_browser as browser;
    #[cfg(feature = "gooey-rasterizer")]
    #[doc(inline)]
    pub use gooey_rasterizer as rasterizer;
    pub mod renderers {
        #[cfg(all(feature = "frontend-kludgine", not(target_arch = "wasm32")))]
        #[doc(inline)]
        pub use gooey_kludgine as kludgine;
    }
}
use cfg_if::cfg_if;
#[doc(inline)]
pub use gooey_core as core;
#[doc(inline)]
pub use gooey_widgets as widgets;

#[cfg(all(feature = "frontend-kludgine", not(target_arch = "wasm32")))]
mod kludgine;
#[cfg(all(feature = "frontend-kludgine", not(target_arch = "wasm32")))]
pub use kludgine::{kludgine_main, kludgine_main_with};

#[cfg(feature = "frontend-browser")]
mod browser;
#[cfg(feature = "frontend-browser")]
pub use browser::{browser_main, browser_main_with};

cfg_if! {
    if #[cfg(all(target_arch = "wasm32", feature = "frontend-browser"))] {
        pub use browser_main as main;
        pub use browser_main_with as main_with;
        pub type ActiveFrontend = gooey_browser::WebSys;
    } else if #[cfg(feature = "frontend-kludgine")] {
        pub use kludgine_main as main;
        pub use kludgine_main_with as main_with;
        pub type ActiveFrontend = gooey_rasterizer::Rasterizer<gooey_kludgine::Kludgine>;
    }
}

mod app;

pub use app::App;
pub mod style;