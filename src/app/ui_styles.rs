use iced::{Background, Color};

pub struct HeaderStyle;

impl iced::widget::container::StyleSheet for HeaderStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::WHITE)),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::from_rgb(0.9, 0.9, 0.9),
            text_color: None,
        }
    }
}

pub struct BackgroundStyle;

impl iced::widget::container::StyleSheet for BackgroundStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: None,
        }
    }
}

pub struct ScrollableStyle;

impl iced::widget::scrollable::StyleSheet for ScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Scrollbar {
        iced::widget::scrollable::Scrollbar {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            border_radius: 5.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: iced::widget::scrollable::Scroller {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                border_radius: 5.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_mouse_over_scrollbar: bool) -> iced::widget::scrollable::Scrollbar {
        self.active(_style)
    }
}
