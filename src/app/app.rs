use iced::{Application, Command, Element, Settings, executor, Subscription, theme, Color};
use iced::widget::{Column, Row, Scrollable, Container, Button, Text, Space, Image, TextInput, Checkbox, Radio};
use iced::{Alignment, Length, Padding};
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
    filtered_photos: Vec<Photo>,
    selected_photo: Option<usize>,
    loading: bool,
    row_count: usize,
    search_term: String,
    file_types: std::collections::HashMap<String, bool>,
    size_filter: SizeFilter,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SizeFilter {
    All,
    Small,
    Medium,
    Large,
    Custom(u32),
}

#[derive(Debug, Clone)]
enum Message {
    PhotosLoaded(Vec<Photo>),
    PhotoSelected(usize),
    PhotoDeselected,
    SearchInput(String),
    ToggleFileType(String),
    SelectSizeFilter(SizeFilter),
}

impl Application for PhotoOrganizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut file_types = std::collections::HashMap::new();
        for ext in ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"] {
            file_types.insert(ext.to_string(), true);
        }

        (
            PhotoOrganizer {
                photos: Vec::new(),
                filtered_photos: Vec::new(),
                selected_photo: None,
                loading: true,
                row_count: 0,
                search_term: String::new(),
                file_types,
                size_filter: SizeFilter::All,
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
                self.photos = list;
                self.apply_filters();
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
            Message::SearchInput(term) => {
                self.search_term = term;
                self.apply_filters();
                Command::none()
            }
            Message::ToggleFileType(ext) => {
                if let Some(value) = self.file_types.get(&ext) {
                    self.file_types.insert(ext, !*value);
                }
                self.apply_filters();
                Command::none()
            }
            Message::SelectSizeFilter(filter) => {
                self.size_filter = filter;
                self.apply_filters();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = create_header();
        let filters = create_filters(self);
        
        let content = if self.loading {
            create_loading_view()
        } else {
            create_photo_grid(&self.filtered_photos, self.selected_photo)
        };

        Column::new()
            .push(header)
            .push(filters)
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
        .align_items(Alignment::Center)
        .padding(Padding::new(20.0));

    Container::new(header_content)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(theme::Container::Custom(Box::new(HeaderStyle)))
}

fn create_filters<'a>(app: &'a PhotoOrganizer) -> Container<'a, Message> {
    let search_input = Text::new("Search:")
        .size(14)
        .style(theme::Text::Color(Color::from_rgb(0.6, 0.6, 0.6)));

    let search_field = TextInput::new("Search by filename...", &app.search_term)
        .on_input(Message::SearchInput)
        .padding(Padding::new(8.0));

    let mut file_type_filters = Row::new().spacing(10);
    for (ext, &enabled) in &app.file_types {
        let ext_clone = ext.clone();
        file_type_filters = file_type_filters.push(
            Checkbox::new(ext.clone(), enabled, move |_| Message::ToggleFileType(ext_clone.clone()))
        );
    }

    let size_filters = Row::new().spacing(10)
        .push(Text::new("Size:").size(14))
        .push(
            Radio::new("All", SizeFilter::All, Some(app.size_filter), |_| Message::SelectSizeFilter(SizeFilter::All))
        )
        .push(
            Radio::new("Small (<100K)", SizeFilter::Small, Some(app.size_filter), |_| Message::SelectSizeFilter(SizeFilter::Small))
        )
        .push(
            Radio::new("Medium (100K-500K)", SizeFilter::Medium, Some(app.size_filter), |_| Message::SelectSizeFilter(SizeFilter::Medium))
        )
        .push(
            Radio::new("Large (>500K)", SizeFilter::Large, Some(app.size_filter), |_| Message::SelectSizeFilter(SizeFilter::Large))
        );

    Container::new(
        Column::new()
            .push(Row::new().push(search_input).push(search_field).spacing(10))
            .push(file_type_filters)
            .push(size_filters)
            .spacing(15)
            .padding(Padding::new(20.0))
    )
    .width(Length::Fill)
    .style(theme::Container::Custom(Box::new(BackgroundStyle)))
}

fn create_loading_view() -> Element<'static, Message> {
    let loading_text = Text::new("Loading photos...")
        .size(16)
        .style(theme::Text::Color(Color::from_rgb(0.6, 0.6, 0.6)));

    Container::new(loading_text)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}

fn create_empty_view() -> Element<'static, Message> {
    let empty_text = Text::new("No photos found in your Pictures folder")
        .size(16)
        .style(theme::Text::Color(Color::from_rgb(0.6, 0.6, 0.6)));

    Container::new(empty_text)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
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

impl PhotoOrganizer {
    fn apply_filters(&mut self) {
        let search_term = self.search_term.to_lowercase();
        
        self.filtered_photos = self.photos.iter()
            .filter(|photo| {
                // Search filter
                if !search_term.is_empty() {
                    photo.name.to_lowercase().contains(&search_term)
                } else {
                    true
                }
            })
            .filter(|photo| {
                // File type filter
                let extension = photo.path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default()
                    .to_lowercase();
                
                self.file_types.get(&extension).copied().unwrap_or(false)
            })
            .filter(|photo| {
                match self.size_filter {
                    SizeFilter::All => true,
                    SizeFilter::Small => photo.width * photo.height <= 100_000,
                    SizeFilter::Medium => photo.width * photo.height > 100_000 && photo.width * photo.height <= 500_000,
                    SizeFilter::Large => photo.width * photo.height > 500_000,
                    SizeFilter::Custom(min_size) => photo.width * photo.height >= min_size,
                }
            })
            .cloned()
            .collect();
        
        self.row_count = (self.filtered_photos.len() + 5) / 6;
    }
}
