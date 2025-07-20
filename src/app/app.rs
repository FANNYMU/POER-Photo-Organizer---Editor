use iced::{Application, Command, Element, Settings, executor, Subscription, theme, Alignment};
use iced::widget::{Column, Scrollable, Container, Button};
use iced::alignment::Horizontal;
use std::path::PathBuf;
use walkdir::WalkDir;
use iced::widget::Image;

pub fn main() -> iced::Result {
    PhotoOrganizer::run(Settings::default())
}

struct PhotoOrganizer {
    photos: Vec<Photo>,
    scroll: iced::widget::scrollable::State,
}

#[derive(Debug, Clone)]
enum Message {
    PhotosLoaded(Vec<Photo>),
    Scrolled,
    PhotoSelected(usize),
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
                scroll: iced::widget::scrollable::State::new(),
            },
            Command::perform(load_photos(), Message::PhotosLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("POER - Photo Organizer & Editor Ringan")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PhotosLoaded(list) => {
                self.photos = list;
                Command::none()
            }
            Message::Scrolled => Command::none(),
            Message::PhotoSelected(index) => {
                println!("Selected photo: {}", index);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = Column::new().spacing(10).padding(10);

        for (index, photo) in self.photos.iter().enumerate() {
            // Wrap the Image in a Button to handle click events
            let image_button = Button::new(
                Image::new(photo.path.clone())
                    .width(100)
                    .height(100),
            )
                .on_press(Message::PhotoSelected(index));

            content = content.push(image_button);
        }

        let scrollable = Scrollable::new(content);

        Container::new(scrollable)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(Horizontal::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

#[derive(Clone, Debug)]
struct Photo {
    path: PathBuf,
}

async fn load_photos() -> Vec<Photo> {
    let mut photos = Vec::new();
    let pictures_dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("~/Pictures"));

    for entry in WalkDir::new(pictures_dir) {
        let entry = entry.unwrap();
        let path = entry.path().to_path_buf();

        if path.is_file() && is_image(&path) {
            photos.push(Photo { path });
        }
    }

    photos
}

fn is_image(path: &PathBuf) -> bool {
    let extension = path.extension().and_then(|e| e.to_str());
    matches!(extension, Some("jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff"))
}

