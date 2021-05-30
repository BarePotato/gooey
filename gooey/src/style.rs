use gooey_core::styles::{
    style_sheet::{Rule, StyleSheet},
    BackgroundColor, ColorPair, Srgba, TextColor,
};
use gooey_rasterizer::CONTROL_CLASS;
use palette::Hsla;

pub fn default_stylesheet() -> StyleSheet {
    // Palette from https://flatuicolors.com/palette/defo
    // let light_green = Srgba::new(0.333, 0.937, 0.769, 1.);
    // let green = Srgba::new(0.000, 0.722, 0.580, 1.);
    // let light_yellow = Srgba::new(1.000, 0.918, 0.655, 1.);
    // let yellow = Srgba::new(0.992, 0.796, 0.431, 1.);
    // let light_teal = Srgba::new(0.506, 0.925, 0.925, 1.);
    // let teal = Srgba::new(0.000, 0.808, 0.788, 1.);
    // let light_peach = Srgba::new(0.980, 0.694, 0.627, 1.);
    // let peach = Srgba::new(0.882, 0.439, 0.333, 1.);
    // let light_blue = Srgba::new(0.455, 0.725, 1.000, 1.);
    // let blue = Srgba::new(0.035, 0.518, 0.890, 1.);
    // let light_red = Srgba::new(1.000, 0.463, 0.459, 1.);
    // let red = Srgba::new(0.839, 0.188, 0.192, 1.);
    // let light_purple = Srgba::new(0.635, 0.608, 0.996, 1.);
    // let purple = Srgba::new(0.424, 0.361, 0.906, 1.);
    // let light_pink = Srgba::new(0.992, 0.475, 0.659, 1.);
    // let pink = Srgba::new(0.910, 0.263, 0.576, 1.);
    let white = Srgba::from(Hsla::new(0., 0., 1., 1.));
    let gray80 = Srgba::from(Hsla::new(0., 0., 0.8, 1.));
    let gray60 = Srgba::from(Hsla::new(0., 0., 0.6, 1.));
    let gray40 = Srgba::from(Hsla::new(0., 0., 0.4, 1.));
    let gray20 = Srgba::from(Hsla::new(0., 0., 0.2, 1.));
    let black = Srgba::from(Hsla::new(0., 0., 0., 1.));

    StyleSheet::default()
        .with(Rule::for_classes(CONTROL_CLASS).with_styles(|style| {
            style
                .with(TextColor(ColorPair {
                    light_color: black,
                    dark_color: white,
                }))
                .with(BackgroundColor(ColorPair {
                    light_color: gray60,
                    dark_color: gray20,
                }))
        }))
        .with(
            Rule::for_classes(CONTROL_CLASS)
                .when_hovered()
                .with_styles(|style| {
                    style.with(BackgroundColor(ColorPair {
                        light_color: gray80,
                        dark_color: gray40,
                    }))
                }),
        )
}
