use iced::{
    application,
    overlay::menu,
    widget::{
        button, checkbox, container, pane_grid, pick_list, progress_bar,
        radio, rule, scrollable, slider, svg, text, text_input, toggler,
    },
};
use iced_core::{Background, BorderRadius, Color, Vector};

use super::widgets::svg_button;

macro_rules! hex_to_color {
    ($x:literal) => {
        Color::from_rgb(
            (($x & 0xFF0000) >> 16) as f32 / 255.,
            (($x & 0xFF00) >> 8) as f32 / 255.,
            ($x & 0xFF) as f32 / 255.,
        )
    };
}

/// Background colors
const BG: Color = hex_to_color!(0x1b1b1b);
const BG_LIGHT: Color = hex_to_color!(0x252525);
const BG_DARK: Color = hex_to_color!(0x121212);

/// Foreground color
const FG: Color = hex_to_color!(0xdddddd);
const FG_LIGHT: Color = hex_to_color!(0xffffff);
const FG_DARK: Color = hex_to_color!(0xc8c8c8);
const FG_DARKER: Color = hex_to_color!(0x737373);

// Primary color
const PRIM: Color = hex_to_color!(0x3acbaf);
const PRIM_DARK: Color = hex_to_color!(0x2bb599);

const OUTLINE: Color = hex_to_color!(0x333333);

#[derive(Default, Clone)]
pub struct Theme {}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BG,
            text_color: PRIM,
        }
    }
}

#[derive(Default, PartialEq)]
pub enum Button {
    #[default]
    Default,
    Primary,
    Item,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let default = button::Appearance {
            shadow_offset: Vector::ZERO,
            background: None,
            border_radius: BorderRadius::from(0.),
            border_width: 0.,
            border_color: Color::TRANSPARENT,
            text_color: FG,
        };

        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(PRIM)),
                border_radius: BorderRadius::from(6.),
                text_color: Color::BLACK,
                ..default
            },
            Button::Item => button::Appearance {
                text_color: FG,
                ..default
            },
            _ => default,
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(PRIM_DARK)),
                ..self.active(style)
            },
            Button::Item => button::Appearance {
                background: Some(Background::Color(BG_LIGHT)),
                border_radius: BorderRadius::from(6.),
                ..self.active(style)
            },
            _ => button::Appearance {
                text_color: PRIM,
                ..self.active(style)
            },
        }
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = ();

    fn active(
        &self,
        _style: &Self::Style,
        _is_checked: bool,
    ) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(BG_DARK),
            icon_color: PRIM,
            border_radius: BorderRadius::from(6.),
            border_width: 0.,
            border_color: BG_LIGHT,
            text_color: Some(FG),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_checked: bool,
    ) -> checkbox::Appearance {
        self.active(style, is_checked)
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub enum Container {
    #[default]
    Default,
    Dark,
    Separate,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Dark => container::Appearance {
                background: Some(Background::Color(BG_DARK)),
                ..container::Appearance::default()
            },
            Container::Separate => container::Appearance {
                background: Some(Background::Color(OUTLINE)),
                ..container::Appearance::default()
            },
            _ => container::Appearance::default(),
        }
    }
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: FG,
            background: Background::Color(BG_LIGHT),
            border_width: 0.,
            border_radius: BorderRadius::from(6.),
            border_color: BG_LIGHT,
            selected_text_color: PRIM,
            selected_background: Background::Color(BG),
        }
    }
}

impl pane_grid::StyleSheet for Theme {
    type Style = ();

    fn hovered_region(&self, _style: &Self::Style) -> pane_grid::Appearance {
        pane_grid::Appearance {
            background: Background::Color(BG_LIGHT),
            border_width: 0.,
            border_color: BG_LIGHT,
            border_radius: BorderRadius::from(6.),
        }
    }

    fn picked_split(&self, _style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: PRIM,
            width: 0.,
        })
    }

    fn hovered_split(&self, _style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: PRIM,
            width: 0.,
        })
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(
        &self,
        _style: &<Self as pick_list::StyleSheet>::Style,
    ) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: FG,
            placeholder_color: FG_DARK,
            handle_color: PRIM,
            background: Background::Color(BG),
            border_radius: BorderRadius::from(6.),
            border_width: 0.,
            border_color: BG,
        }
    }

    fn hovered(
        &self,
        style: &<Self as pick_list::StyleSheet>::Style,
    ) -> pick_list::Appearance {
        self.active(style)
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: Background::Color(BG_LIGHT),
            bar: Background::Color(PRIM),
            border_radius: BorderRadius::from(6.),
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(
        &self,
        _style: &Self::Style,
        _is_selected: bool,
    ) -> radio::Appearance {
        radio::Appearance {
            background: Background::Color(BG_DARK),
            dot_color: PRIM,
            border_width: 0.,
            border_color: BG_LIGHT,
            text_color: Some(FG),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_selected: bool,
    ) -> radio::Appearance {
        self.active(style, is_selected)
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: PRIM,
            width: 10,
            radius: BorderRadius::from(6.),
            fill_mode: rule::FillMode::Full,
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: BorderRadius::from(6.),
            border_width: 0.,
            border_color: BG_LIGHT,
            scroller: scrollable::Scroller {
                color: BG_LIGHT,
                border_radius: BorderRadius::from(6.),
                border_width: 0.,
                border_color: BG_LIGHT,
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> scrollable::Scrollbar {
        self.active(style)
    }
}

impl slider::StyleSheet for Theme {
    type Style = ();

    fn active(
        &self,
        _style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (PRIM, BG_LIGHT),
                width: 4.,
                border_radius: BorderRadius::from(2.),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 0. },
                color: Color::TRANSPARENT,
                border_width: 0.,
                border_color: BG_LIGHT,
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        let active = self.active(style);
        slider::Appearance {
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 5. },
                color: FG,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(
        &self,
        style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        let active = self.active(style);
        slider::Appearance {
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 5. },
                color: FG_DARK,
                ..active.handle
            },
            ..active
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub enum Svg {
    #[default]
    Default,
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        match style {
            _ => svg::Appearance::default(),
        }
    }
}

#[derive(Clone, Default, Copy)]
pub enum Text {
    /// The default text style
    #[default]
    Default,
    Normal,
    Light,
    Dark,
    Darker,
    Prim,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: match style {
                Text::Light => Some(FG_LIGHT),
                Text::Normal => Some(FG),
                Text::Dark => Some(FG_DARK),
                Text::Darker => Some(FG_DARKER),
                Text::Prim => Some(PRIM),
                _ => None,
            },
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(BG_LIGHT),
            border_radius: BorderRadius::from(6.),
            border_width: 0.,
            border_color: BG_LIGHT,
            icon_color: BG_DARK,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        FG
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        FG
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        FG
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        FG
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}

impl toggler::StyleSheet for Theme {
    type Style = ();

    fn active(
        &self,
        _style: &Self::Style,
        _is_active: bool,
    ) -> toggler::Appearance {
        toggler::Appearance {
            background: BG_DARK,
            background_border: None,
            foreground: FG,
            foreground_border: None,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_active: bool,
    ) -> toggler::Appearance {
        self.active(style, is_active)
    }
}

// Custom Widgets
#[derive(Clone, Default, Copy)]
pub enum SvgButton {
    #[default]
    Transparent,
}

impl svg_button::StyleSheet for Theme {
    type Style = SvgButton;

    fn active(&self, style: &Self::Style) -> svg_button::Appearance {
        let transparent = svg_button::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border_color: Color::TRANSPARENT,
            border_radius: BorderRadius::from(0.),
            border_thickness: 0.,
            color: None,
        };

        match style {
            _ => transparent,
        }
    }

    fn hovered(&self, style: &Self::Style) -> svg_button::Appearance {
        let active = self.active(style);

        match style {
            _ => svg_button::Appearance {
                color: Some(PRIM),
                ..active
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> svg_button::Appearance {
        let active = self.active(style);

        match style {
            _ => svg_button::Appearance {
                color: Some(PRIM),
                ..active
            },
        }
    }
}