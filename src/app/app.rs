use iced::{Application, Command, Element, Settings, executor, Subscription, theme, Color};
use iced::widget::{Column, Row, Scrollable, Container, Button, Text, Space, Image, TextInput, Checkbox, Radio};
use iced::{Alignment, Length, Padding};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use std::cmp::Ordering;

pub use app::photo_loader::{load_photos, Photo};
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
    file_types: HashMap<String, bool>,
    size_filter: SizeFilter,
    sort_criteria: SortCriteria,
    sort_order: SortOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SizeFilter {
    All,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCriteria {
    Name,
    Date,
    Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub enum Message {
    PhotosLoaded(Vec<Photo>),
    PhotoSelected(usize),
    PhotoDeselected,
    SearchInput(String),
    ToggleFileType(String),
    SelectSizeFilter(SizeFilter),
    SortCriteriaChanged(SortCriteria),
    ToggleSortOrder,
}

impl Application for PhotoOrganizer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut file_types = HashMap::new();
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
                sort_criteria: SortCriteria::Name,
                sort_order: SortOrder::Ascending,
            },
            Command::perform(load_photos(), Message::PhotosLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("POER - Photo Organizer & Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PhotosLoaded(photos) => {
                self.photos = photos;
                self.apply_filters();
                self.loading = false;
            }
            Message::PhotoSelected(index) => {
                self.selected_photo = Some(index);
            }
            Message::PhotoDeselected => {
                self.selected_photo = None;
            }
            Message::SearchInput(term) => {
                self.search_term = term;
                self.apply_filters();
            }
            Message::ToggleFileType(ext) => {
                if let Some(enabled) = self.file_types.get_mut(&ext) {
                    *enabled = !*enabled;
                }
                self.apply_filters();
            }
            Message::SelectSizeFilter(size_filter) => {
                self.size_filter = size_filter;
                self.apply_filters();
            }
            Message::SortCriteriaChanged(criteria) => {
                self.sort_criteria = criteria;
                self.apply_filters();
            }
            Message::ToggleSortOrder => {
                self.sort_order = match self.sort_order {
                    SortOrder::Ascending => SortOrder::Descending,
                    SortOrder::Descending => SortOrder::Ascending,
                };
                self.apply_filters();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let header = create_header();
        let filters = create_filters(self);
        
        let content = if self.loading {
            create_loading_view()
        } else if self.filtered_photos.is_empty() {
            create_empty_view()
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
            Checkbox::new(ext, enabled, move |_| Message::ToggleFileType(ext_clone.clone()))
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

    let filters_column = Column::new()
        .push(Row::new().push(search_input).push(search_field).spacing(10))
        .push(file_type_filters)
        .push(size_filters)
        .push(create_sorting_controls(app))
        .spacing(15)
        .padding(Padding::new(20.0));

    Container::new(filters_column)
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

    for (row_index, row_photos) in photos.chunks(6).enumerate() {
        let mut row = Row::new().spacing(12);

        for (col_index, photo) in row_photos.iter().enumerate() {
            let global_index = row_index * 6 + col_index;
            let is_selected = selected == Some(global_index);

            let photo_card = create_photo_card(photo, global_index, is_selected);
            row = row.push(photo_card);
        }

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

struct PhotoCardStyle;

impl iced::widget::button::StyleSheet for PhotoCardStyle {
    type Style = iced::Theme;

    fn active(&self, theme: &Self::Style) -> iced::widget::button::Appearance {
        let _palette = theme.extended_palette();
        
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(0.95, 0.95, 0.95, 0.8))),
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_width: 1.0,
            shadow_offset: iced::Vector::new(0.0, 3.0),
            ..Default::default()
        }
    }

    fn hovered(&self, theme: &Self::Style) -> iced::widget::button::Appearance {
        let _palette = theme.extended_palette();
        
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(0.98, 0.98, 0.98, 0.9))),
            border_color: Color::from_rgb(0.4, 0.6, 0.8),
            shadow_offset: iced::Vector::new(0.0, 6.0),
            ..Default::default()
        }
    }

    fn pressed(&self, theme: &Self::Style) -> iced::widget::button::Appearance {
        let _palette = theme.extended_palette();
        
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(0.92, 0.92, 0.92, 0.85))),
            border_color: Color::from_rgb(0.3, 0.5, 0.7),
            shadow_offset: iced::Vector::new(0.0, 2.0),
            ..Default::default()
        }
    }
}

fn create_photo_card(photo: &Photo, index: usize, is_selected: bool) -> Button<Message> {
    let image = Image::new(photo.path.clone())
        .width(180)
        .height(120);

    let filename = Text::new(&photo.name)
        .size(14)
        .style(theme::Text::Color(Color::from_rgb(0.2, 0.2, 0.2)));

    let card_content = Column::new()
        .push(image)
        .push(filename)
        .spacing(12)
        .align_items(Alignment::Center)
        .padding(Padding::new(12.0));

    Button::new(card_content)
        .width(200)
        .height(240)
        .padding(Padding::new(0.0))
        .style(theme::Button::Custom(Box::new(PhotoCardStyle)))
        .on_press(if is_selected {
            Message::PhotoDeselected
        } else {
            Message::PhotoSelected(index)
        })
}

fn create_sorting_controls(app: &PhotoOrganizer) -> Container<'static, Message> {
    let sort_criteria = Row::new()
        .push(Text::new("Sort by:").size(14))
        .push(
            Radio::new("Name", SortCriteria::Name, Some(app.sort_criteria), move |v| Message::SortCriteriaChanged(v))
        )
        .push(
            Radio::new("Date", SortCriteria::Date, Some(app.sort_criteria), move |v| Message::SortCriteriaChanged(v))
        )
        .push(
            Radio::new("Size", SortCriteria::Size, Some(app.sort_criteria), move |v| Message::SortCriteriaChanged(v))
        );

    let sort_order = Button::new(
        Text::new(match app.sort_order {
            SortOrder::Ascending => "↑ Ascending",
            SortOrder::Descending => "↓ Descending",
        })
    )
    .on_press(Message::ToggleSortOrder);

    Container::new(
        Column::new()
            .push(sort_criteria)
            .push(sort_order)
            .spacing(15)
    )
    .width(Length::Fill)
    .style(theme::Container::Custom(Box::new(BackgroundStyle)))
}

impl PhotoOrganizer {
    fn apply_filters(&mut self) {
        let search_term = self.search_term.to_lowercase();

        let filtered = self.photos.iter()
            .filter(|photo| {
                if !search_term.is_empty() {
                    photo.name.to_lowercase().contains(&search_term)
                } else {
                    true
                }
            })
            .filter(|photo| {
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
                    SizeFilter::Medium => photo.width * photo.height > 100_000 && photo.width * photo.height < 500_000,
                    SizeFilter::Large => photo.width * photo.height >= 500_000,
                }
            })
            .cloned()
            .collect::<Vec<Photo>>();

        let mut photo_times: HashMap<PathBuf, SystemTime> = HashMap::new();

        let mut sorted_filtered = filtered;
        
        for photo in &sorted_filtered {
            if let Ok(metadata) = std::fs::metadata(&photo.path) {
                if let Ok(modified) = metadata.modified() {
                    photo_times.insert(photo.path.clone(), modified);
                }
            }
        }

        match (self.sort_criteria, self.sort_order) {
            (SortCriteria::Name, SortOrder::Ascending) => {
                sorted_filtered.sort_by(|a, b| a.name.cmp(&b.name));
            }
            (SortCriteria::Name, SortOrder::Descending) => {
                sorted_filtered.sort_by(|a, b| b.name.cmp(&a.name));
            }
            (SortCriteria::Date, SortOrder::Ascending) => {
                sorted_filtered.sort_by(|a, b| {
                    let a_time = photo_times.get(&a.path);
                    let b_time = photo_times.get(&b.path);
                    match (a_time, b_time) {
                        (Some(a), Some(b)) => a.cmp(&b),
                        _ => Ordering::Equal,
                    }
                });
            }
            (SortCriteria::Date, SortOrder::Descending) => {
                sorted_filtered.sort_by(|a, b| {
                    let a_time = photo_times.get(&a.path);
                    let b_time = photo_times.get(&b.path);
                    match (a_time, b_time) {
                        (Some(a), Some(b)) => b.cmp(&a),
                        _ => Ordering::Equal,
                    }
                });
            }
            (SortCriteria::Size, SortOrder::Ascending) => {
                sorted_filtered.sort_by(|a, b| a.size.cmp(&b.size));
            }
            (SortCriteria::Size, SortOrder::Descending) => {
                sorted_filtered.sort_by(|a, b| b.size.cmp(&a.size));
            }
        }
        
        self.filtered_photos = sorted_filtered;
        self.row_count = (self.filtered_photos.len() + 5) / 6;
    }
}
