#![allow(unused)]

use std::backtrace;

use iced::{
    application,
    overlay::menu,
    widget::{
        button, checkbox, container, pane_grid, pick_list, progress_bar,
        radio, rule, scrollable, slider, svg, text, text_input, toggler,
    },
};
use iced_core::{Background, BorderRadius, Color, Vector};

use super::widgets::{list_view, svg_button, text_ellipsis};

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
const OUTLINE_DARK: Color = hex_to_color!(0x2b2b2b);

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
    Menu(bool),
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
            Button::Menu(selected) => button::Appearance {
                text_color: if *selected { PRIM } else { FG },
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
            Button::Item | Button::Menu(_) => button::Appearance {
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

#[derive(Copy, Clone, Default, Debug)]
pub enum Rule {
    #[default]
    Default,
    Separate(u16),
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        let default = rule::Appearance {
            color: OUTLINE,
            width: 2,
            radius: BorderRadius::from(6.),
            fill_mode: rule::FillMode::Full,
        };

        match style {
            Rule::Separate(width) => rule::Appearance {
                width: *width,
                ..default
            },
            _ => default,
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
        is_active: bool,
    ) -> toggler::Appearance {
        let bg = if is_active {
            PRIM
        } else {
            OUTLINE
        };
        toggler::Appearance {
            background: bg,
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
        let bg = if is_active {
            PRIM_DARK
        } else {
            OUTLINE_DARK
        };
        toggler::Appearance {
            background: bg,
            ..self.active(style, is_active)
        }
    }
}

// Custom Widgets
#[derive(Clone, Default, Copy)]
pub enum SvgButton {
    #[default]
    Transparent,
    Circle(f32),
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
            SvgButton::Circle(size) => svg_button::Appearance {
                background: Background::Color(FG),
                border_radius: BorderRadius::from(size / 2.),
                color: Some(BG_DARK),
                ..transparent
            },
            _ => transparent,
        }
    }

    fn hovered(&self, style: &Self::Style) -> svg_button::Appearance {
        let active = self.active(style);

        match style {
            SvgButton::Circle(size) => svg_button::Appearance {
                background: Background::Color(PRIM),
                ..active
            },
            _ => svg_button::Appearance {
                color: Some(PRIM),
                ..active
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> svg_button::Appearance {
        let active = self.active(style);

        match style {
            SvgButton::Circle(size) => svg_button::Appearance {
                background: Background::Color(PRIM),
                ..active
            },
            _ => svg_button::Appearance {
                color: Some(PRIM),
                ..active
            },
        }
    }
}

#[derive(Default, Clone)]
pub enum WrapBox {
    #[default]
    Bright,
    Dark,
}

impl list_view::StyleSheet for Theme {
    type Style = WrapBox;

    fn background(
        &self,
        style: &Self::Style,
        _pos: list_view::MousePos,
    ) -> list_view::SquareStyle {
        let base = list_view::SquareStyle {
            background: Background::Color(BG_DARK),
            border: Color::TRANSPARENT,
            border_thickness: 0.,
            border_radius: 0.0.into(),
        };

        match style {
            WrapBox::Bright => list_view::SquareStyle {
                background: Background::Color(Color::TRANSPARENT),
                ..base
            },
            _ => base,
        }
    }

    fn button_style(
        &self,
        _style: &Self::Style,
        pos: list_view::MousePos,
        pressed: bool,
        is_start: bool,
        relative_scroll: f32,
    ) -> list_view::ButtonStyle {
        let square = list_view::SquareStyle {
            background: Background::Color(Color::TRANSPARENT),
            border: Color::TRANSPARENT,
            border_thickness: 0.0.into(),
            border_radius: 6.0.into(),
        };

        if is_start && relative_scroll == 0.
            || !is_start && relative_scroll == 1.
        {
            // inactive
            list_view::ButtonStyle {
                square: list_view::SquareStyle {
                    border_thickness: 0.,
                    ..square
                },
                foreground: BG_LIGHT,
            }
        } else {
            // active

            let foreground = if pressed {
                PRIM_DARK
            } else if pos == list_view::MousePos::DirectlyOver {
                PRIM
            } else {
                OUTLINE_DARK
            };

            list_view::ButtonStyle { square, foreground }
        }
    }

    fn thumb_style(
        &self,
        style: &Self::Style,
        pos: list_view::MousePos,
        pressed: bool,
        _relative_scroll: f32,
    ) -> list_view::SquareStyle {
        let mut square = list_view::SquareStyle {
            background: Background::Color(OUTLINE_DARK),
            border: Color::TRANSPARENT,
            border_thickness: 0.,
            border_radius: 6.0.into(),
        };

        square = match style {
            WrapBox::Bright => list_view::SquareStyle {
                background: Background::Color(OUTLINE_DARK),
                ..square
            },
            _ => square,
        };

        if pressed {
            list_view::SquareStyle {
                background: Background::Color(OUTLINE),
                ..square
            }
        } else if pos == list_view::MousePos::DirectlyOver {
            list_view::SquareStyle {
                background: Background::Color(OUTLINE),
                ..square
            }
        } else {
            square
        }
    }

    fn trough_style(
        &self,
        _style: &Self::Style,
        _pos: list_view::MousePos,
        is_start: bool,
        _relative_scroll: f32,
    ) -> list_view::SquareStyle {
        list_view::SquareStyle {
            background: Background::Color(Color::TRANSPARENT),
            border: OUTLINE,
            border_thickness: 0.0,
            border_radius: if is_start {
                [6.0, 6.0, 0., 0.].into()
            } else {
                [0., 0., 6.0, 6.0].into()
            },
        }
    }
}

impl list_view::LayoutStyleSheet<()> for Theme {
    fn layout(&self, _style: &()) -> list_view::LayoutStyle {
        list_view::LayoutStyle {
            padding: Some([0, 0, 0, 20].into()),
            spacing: (None, Some(1.)),
            ..list_view::LayoutStyle::default()
        }
    }
}

impl text_ellipsis::StyleSheet for Theme {
    type Style = Text;

    fn foreground(&self, style: &Self::Style) -> Option<Color> {
        match style {
            Text::Light => Some(FG_LIGHT),
            Text::Normal => Some(FG),
            Text::Dark => Some(FG_DARK),
            Text::Darker => Some(FG_DARKER),
            Text::Prim => Some(PRIM),
            _ => None,
        }
    }
}
