pub struct PhotoCardStyle {
    pub(crate) is_selected: bool,
}

impl iced::widget::button::StyleSheet for PhotoCardStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let base_color = if self.is_selected {
            iced::Color::from_rgb(0.95, 0.97, 1.0)
        } else {
            iced::Color::WHITE
        };

        let border_width = if self.is_selected { 3.0 } else { 1.0 };
        let border_color = if self.is_selected {
            iced::Color::from_rgb(0.2, 0.5, 0.9)
        } else {
            iced::Color::from_rgb(0.9, 0.9, 0.9)
        };

        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(base_color)),
            border_radius: 12.0.into(),
            border_width,
            border_color,
            shadow_offset: iced::Vector::new(0.0, 2.0),
            text_color: iced::Color::BLACK,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
            border_radius: 12.0.into(),
            border_width: 2.0,
            border_color: iced::Color::from_rgb(0.2, 0.5, 0.9),
            shadow_offset: iced::Vector::new(0.0, 4.0),
            text_color: iced::Color::BLACK,
        }
    }

    fn pressed(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
            border_radius: 12.0.into(),
            border_width: 2.0,
            border_color: iced::Color::from_rgb(0.2, 0.5, 0.9),
            shadow_offset: iced::Vector::new(0.0, 1.0),
            text_color: iced::Color::BLACK,
        }
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }
}
