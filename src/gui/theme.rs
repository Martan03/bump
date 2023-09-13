use iced::{
    application,
    overlay::menu,
    widget::{
        button, checkbox, container, pane_grid, pick_list, progress_bar,
        radio, rule, scrollable, slider, svg, text, text_input, toggler,
    },
};
use iced_core::{Background, BorderRadius, Color, Vector};

/// Background colors
const BG: Color = Color::from_rgb(27. / 255., 27. / 255., 27. / 255.);
const BG_LIGHT: Color = Color::from_rgb(37. / 255., 37. / 255., 37. / 255.);
const BG_DARK: Color = Color::from_rgb(15. / 255., 15. / 255., 15. / 255.);

/// Foreground color
const FG: Color = Color::from_rgb(221. / 255., 221. / 255., 221. / 255.);
const _FG_LIGHT: Color =
    Color::from_rgb(240. / 240., 221. / 240., 221. / 255.);
const FG_DARK: Color = Color::from_rgb(200. / 255., 200. / 255., 200. / 255.);

const PRIM: Color = Color::from_rgb(58. / 255., 203. / 255., 175. / 255.);
const PRIM_DARK: Color = Color::from_rgb(43. / 255., 181. / 255., 153. / 255.);

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
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                shadow_offset: Vector::ZERO,
                background: Some(Background::Color(PRIM)),
                border_radius: BorderRadius::from(6.),
                border_width: 0.,
                border_color: Color::TRANSPARENT,
                text_color: Color::BLACK,
            },
            _ => button::Appearance {
                shadow_offset: Vector::ZERO,
                background: None,
                border_radius: BorderRadius::from(0.),
                border_width: 0.,
                border_color: Color::TRANSPARENT,
                text_color: FG,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(PRIM_DARK)),
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
        style: &Self::Style,
        is_checked: bool,
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
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Dark => container::Appearance {
                background: Some(Background::Color(BG_DARK)),
                ..container::Appearance::default()
            },
            _ => container::Appearance::default(),
        }
    }
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
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

    fn hovered_region(&self, style: &Self::Style) -> pane_grid::Appearance {
        pane_grid::Appearance {
            background: Background::Color(BG_LIGHT),
            border_width: 0.,
            border_color: BG_LIGHT,
            border_radius: BorderRadius::from(6.),
        }
    }

    fn picked_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: PRIM,
            width: 0.,
        })
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
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
        style: &<Self as pick_list::StyleSheet>::Style,
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

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
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
        is_selected: bool,
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

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
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

#[derive(Copy, Clone, Default, Debug)]
pub enum Slider {
    #[default]
    Default,
}

impl slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(
        &self,
        style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        match style {
            _ => slider::Appearance {
                rail: slider::Rail {
                    colors: (PRIM, PRIM),
                    width: 4.,
                    border_radius: 2.0.into(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 0. },
                    color: Color::TRANSPARENT,
                    border_width: 0.,
                    border_color: BG,
                },
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        todo!()
    }

    fn dragging(
        &self,
        style: &Self::Style,
    ) -> iced::widget::vertical_slider::Appearance {
        todo!()
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
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: match style {
                _ => None,
            },
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
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

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        FG
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        FG
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        FG
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
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
        style: &Self::Style,
        is_active: bool,
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
