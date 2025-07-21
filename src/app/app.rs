use iced::{Application, Command, Element, Settings, executor, Subscription, theme, Color};
use iced::widget::{Column, Row, Scrollable, Container, Button, Text, Space};
use iced::alignment::{Horizontal, Vertical};
use iced::{Length, Padding};
use iced::widget::Image;

pub use app::photo_loader::{load_photos, Photo};
pub use app::photo_card_style::PhotoCardStyle;
pub use app::ui_styles::{HeaderStyle, BackgroundStyle, ScrollableStyle};
use crate::app;

pub fn main() -> iced::Result {
    PhotoOrganizer::run(Settings {
        window: iced::window::Settings {
            size: (1200, 800),
            min_size: Some((800, 600)),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct PhotoOrganizer {
    photos: Vec<Photo>,
    selected_photo: Option<usize>,
    loading: bool,
    row_count: usize,
}

#[derive(Debug, Clone)]
enum Message {
    PhotosLoaded(Vec<Photo>),
    PhotoSelected(usize),
    PhotoDeselected,
}

impl Application for PhotoOrganizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            PhotoOrganizer {
                photos: Vec::new(),
                selected_photo: None,
                loading: true,
                row_count: 0,
            },
            Command::perform(load_photos(), Message::PhotosLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("POER - Photo Organizer & Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PhotosLoaded(list) => {
                self.row_count = (list.len() + 5) / 6;
                self.photos = list;
                self.loading = false;
                Command::none()
            }
            Message::PhotoSelected(index) => {
                self.selected_photo = Some(index);
                Command::none()
            }
            Message::PhotoDeselected => {
                self.selected_photo = None;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = create_header();

        let content = if self.loading {
            create_loading_view()
        } else if self.photos.is_empty() {
            create_empty_view()
        } else {
            create_photo_grid(&self.photos, self.selected_photo)
        };

        Column::new()
            .push(header)
            .push(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

fn create_header() -> Container<'static, Message> {
    let title = Text::new("POER")
        .size(28)
        .style(theme::Text::Color(Color::from_rgb(0.2, 0.5, 0.9)));

    let subtitle = Text::new("Photo Organizer & Editor")
        .size(14)
        .style(theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5)));

    let header_content = Row::new()
        .push(
            Column::new()
                .push(title)
                .push(subtitle)
                .spacing(2)
        )
        .push(Space::with_width(Length::Fill))
        .align_items(iced::Alignment::Center)
        .padding(Padding::new(20.0));

    Container::new(header_content)
        .width(Length::Fill)
        .style(theme::Container::Custom(Box::new(HeaderStyle)))
}

fn create_loading_view() -> Element<'static, Message> {
    let loading_text = Text::new("Loading photos...")
        .size(16)
        .style(theme::Text::Color(Color::from_rgb(0.6, 0.6, 0.6)));

    Container::new(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn create_empty_view() -> Element<'static, Message> {
    let empty_text = Text::new("No photos found in your Pictures folder")
        .size(16)
        .style(theme::Text::Color(Color::from_rgb(0.6, 0.6, 0.6)));

    Container::new(empty_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn create_photo_grid(photos: &[Photo], selected: Option<usize>) -> Element<Message> {
    let mut grid_content = Column::new().spacing(16).padding(Padding::new(20.0));

    // Group photos into rows of 6
    for (row_index, row_photos) in photos.chunks(6).enumerate() {
        let mut row = Row::new().spacing(12);

        for (col_index, photo) in row_photos.iter().enumerate() {
            let global_index = row_index * 6 + col_index;
            let is_selected = selected == Some(global_index);

            let photo_card = create_photo_card(photo, global_index, is_selected);
            row = row.push(photo_card);
        }

        // Fill remaining space in row if needed
        row = row.push(Space::with_width(Length::Fill));
        grid_content = grid_content.push(row);
    }

    let scrollable = Scrollable::new(grid_content)
        .style(theme::Scrollable::Custom(Box::new(ScrollableStyle)));

    Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(BackgroundStyle)))
        .into()
}

fn create_photo_card(photo: &Photo, index: usize, is_selected: bool) -> Button<Message> {
    let image = Image::new(photo.path.clone())
        .width(180)
        .height(180);

    let card_content = Container::new(image)
        .width(180)
        .height(180)
        .padding(Padding::new(8.0));

    Button::new(card_content)
        .style(theme::Button::Custom(Box::new(PhotoCardStyle { is_selected })))
        .on_press(if is_selected {
            Message::PhotoDeselected
        } else {
            Message::PhotoSelected(index)
        })
}
