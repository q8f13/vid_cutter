use core::ops::RangeInclusive;
use iced::{
    button, slider, text_input, Button, Column, Container, Slider, Space, Text, TextInput, Row, Sandbox,
    Align, ProgressBar, Image, Element, Settings, image, 
};

fn main() {
    VidCutter::run(Settings::default())
}

// #[derive(Default)]
struct VidCutter{
    btn: button::State,
    filename: String,
    time_start: f32,
    time_end: f32,
    play_progress: f32,
    vid_time_range: RangeInclusive<f32>,
    img: image::Handle,
    progress_bar_slider: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message{
    FileLoaded,
    VidPlaying(bool),
    FlagBeginMarked,
    FlagEndMarked,
    FlagMarked,
    SliderChanged(f32),
}

impl VidCutter{
    pub fn new() -> VidCutter{
        VidCutter{
            btn: button::State::default(),
            filename : String::from("Empty"),
            time_start : 0.0,
            time_end : 0.0,
            play_progress : 0.0,
            vid_time_range : RangeInclusive::new(0.0, 100.0),
            img: image::Handle::from_path("res/ferris.png"),
            progress_bar_slider: slider::State::default(),
        }
    }
}

impl Sandbox for VidCutter{
    type Message = Message;

    fn new() -> Self{
        VidCutter::new()
    }

    fn title(&self) -> String{
        String::from("Video Cutter")
    }

    fn update(&mut self, message: Message){
        match message{
            // FileLoaded=>(),// drag vid file and init streaming done
            Message::FlagMarked=>(),
            Message::FileLoaded=>(),
            Message::VidPlaying(b)=>(),
            Message::FlagBeginMarked=>(),
            Message::FlagEndMarked=>(),
            Message::SliderChanged(v)=>self.play_progress = v,
        }
    }

    fn view(&mut self) -> Element<Message>{
        let progress_bar = ProgressBar::new(0.0..=100.0, self.play_progress);
        // let mut slider_st = iced_native::widget::slider::State::new();
        let slider = Slider::new(&mut self.progress_bar_slider, 0.0..=100.0, self.play_progress, Message::SliderChanged);
        let button = Button::new(&mut self.btn, Text::new("AddMarker"))
            .padding(10)
            .on_press(Message::FlagMarked);
        let content = Column::new()
            .spacing(20).padding(20)
            .max_width(600)
            .push(Image::new(self.img.clone()))
            .push(progress_bar)
            .push(slider)
            .push(button);

        Container::new(content).into()
    }
}
