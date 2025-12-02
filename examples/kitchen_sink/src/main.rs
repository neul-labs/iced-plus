//! Kitchen sink example demonstrating iced-plus features.
//!
//! This example showcases the type-safe components and layouts from iced-plus.

mod audio_backend;
mod webcam_backend;
mod webview_backend;

use audio_backend::AudioPlayer;
use webcam_backend::WebcamCapture;
use webview_backend::WebViewWindow;

use iced::widget::{column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{alignment, Element, Length, Subscription, Task, Theme};
use std::time::{Duration, Instant};

use iced_plus_components::button::Button;
use iced_plus_components::checkbox::Checkbox;
use iced_plus_components::color_picker::{color_palette, color_picker_view, color_to_hex, presets, ColorSwatch};
use iced_plus_components::{Icon, IconName};
use iced_plus_components::media::{AudioControls, AudioRecorder, MediaPlayerState, RecorderState, VideoRecorder};
use iced_plus_components::spinner::{CircularSpinner, DotsSpinner, LinearSpinner, PulseSpinner};
use iced_plus_components::toast::{toast_container, ToastPosition, ToastVariant};
use iced_plus_components::webview::{BrowserBar, WebViewState};
use iced_plus_components::navbar::{AppBar, NavItem, SideNav};
use iced_plus_components::select::Select;
use iced_plus_components::slider::Slider;
use iced_plus_components::tabs::{Tab, TabWidth, Tabs};
use iced_plus_components::tooltip::{Tooltip, TooltipPosition};
use iced_plus_components::textarea::{TextArea, TextAreaContent};
use iced_plus_components::rich_text::{RichTextEditor, RichTextAction, RichTextContent};
use iced_plus_components::{
    Alert, Avatar, AvatarShape, AvatarSize, Badge, Card, Elevation, Heading,
    ImagePlaceholder, Progress, Skeleton, Switch, Text, TextInput,
};
use iced::widget::text_editor;
use iced_plus_layouts::{drawer, BreakpointTier, HStack, Modal, ResponsiveRow, VStack};

/// Application entry point.
pub fn main() -> iced::Result {
    iced::application("Kitchen Sink - iced-plus", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Page {
    #[default]
    Components,
    Inputs,
    Feedback,
    Media,
    WebView,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Components => write!(f, "Components"),
            Self::Inputs => write!(f, "Inputs"),
            Self::Feedback => write!(f, "Feedback"),
            Self::Media => write!(f, "Media"),
            Self::WebView => write!(f, "WebView"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum DropdownOption {
    #[default]
    Option1,
    Option2,
    Option3,
}

impl std::fmt::Display for DropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Option1 => write!(f, "Option 1"),
            Self::Option2 => write!(f, "Option 2"),
            Self::Option3 => write!(f, "Option 3"),
        }
    }
}

struct App {
    dark_mode: bool,
    current_page: Page,
    show_drawer: bool,
    show_modal: bool,
    // Form state
    input_value: String,
    checkbox_checked: bool,
    checkbox2_checked: bool,
    switch_enabled: bool,
    slider_value: f32,
    selected_option: Option<DropdownOption>,
    // Demo state
    counter: i32,
    progress_value: f32,
    active_tab: usize,
    // Spinner state
    spinner_progress: f32,
    // Toast state
    toasts: Vec<(usize, String, ToastVariant)>,
    toast_id_counter: usize,
    // Color picker state
    selected_color: iced::Color,
    hex_input: String,
    palette_index: Option<usize>,
    // Media state
    media_player: MediaPlayerState,
    audio_recorder: RecorderState,
    video_recorder: RecorderState,
    video_recording_start: Option<Instant>,
    video_recorded_duration: Duration,
    // WebView state
    webview_state: WebViewState,
    url_input: String,
    // TextArea and RichTextEditor state
    textarea_content: TextAreaContent,
    rich_text_content: RichTextContent,
    // Real audio player
    audio_player: Option<AudioPlayer>,
    audio_playing: bool,
    // Webcam capture
    webcam: WebcamCapture,
    webcam_frame: Option<iced::widget::image::Handle>,
    webcam_error: Option<String>,
    webcam_owned_by_video: bool,
    // Real webview
    webview_window: WebViewWindow,
}

#[derive(Debug, Clone)]
enum Message {
    // Navigation
    ToggleTheme,
    NavigateTo(Page),
    ToggleDrawer,
    OpenModal,
    CloseModal,
    // Form inputs
    InputChanged(String),
    CheckboxToggled(bool),
    Checkbox2Toggled(bool),
    ToggleSwitch(bool),
    SliderChanged(f32),
    DropdownSelected(DropdownOption),
    // Demo
    Increment,
    Decrement,
    IncrementProgress,
    TabSelected(usize),
    // Media player
    MediaPlay,
    MediaPause,
    MediaSeek(f32),
    MediaVolume(f32),
    // Audio recorder
    AudioRecordStart,
    AudioRecordStop,
    AudioRecordPause,
    AudioRecordResume,
    // Video recorder
    VideoRecordStart,
    VideoRecordStop,
    VideoRecordPause,
    VideoRecordResume,
    VideoToggleAudio,
    VideoToggleVideo,
    // WebView
    UrlInputChanged(String),
    WebViewNavigate,
    WebViewBack,
    WebViewForward,
    WebViewReload,
    // Spinners
    SpinnerTick,
    // Toasts
    ShowToast(ToastVariant),
    CloseToast(usize),
    // Color picker
    HexInputChanged(String),
    PaletteColorSelected(usize),
    // TextArea and RichTextEditor
    TextAreaAction(text_editor::Action),
    RichTextAction(RichTextAction),
    // Real audio player
    PlayTestTone,
    StopAudio,
    // Webcam
    StartWebcam,
    StopWebcam,
    WebcamFrame,
    // Real webview
    OpenWebView,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let mut media_player = MediaPlayerState::new();
        media_player.set_duration(std::time::Duration::from_secs(180));
        media_player.set_position(std::time::Duration::from_secs(45));

        let webview_state = WebViewState::with_url("https://example.com");

        (
            Self {
                dark_mode: false,
                current_page: Page::Components,
                show_drawer: false,
                show_modal: false,
                input_value: String::new(),
                checkbox_checked: false,
                checkbox2_checked: true,
                switch_enabled: false,
                slider_value: 50.0,
                selected_option: Some(DropdownOption::Option1),
                counter: 0,
                progress_value: 0.35,
                active_tab: 0,
                spinner_progress: 0.0,
                toasts: Vec::new(),
                toast_id_counter: 0,
                selected_color: iced::Color::from_rgb(0.2, 0.5, 0.8),
                hex_input: "#3380CC".to_string(),
                palette_index: None,
                media_player,
                audio_recorder: RecorderState::audio_only(),
                video_recorder: RecorderState::new(),
                video_recording_start: None,
                video_recorded_duration: Duration::ZERO,
                webview_state,
                url_input: "https://example.com".to_string(),
                textarea_content: TextAreaContent::with_text("This is a multi-line text area.\n\nYou can type here and it will grow as needed.\n\nTry editing this text!"),
                rich_text_content: RichTextContent::with_text("# Rich Text Editor\n\nThis editor supports **bold**, *italic*, and other formatting.\n\n- Bullet lists\n- Are supported\n\nTry the toolbar buttons above!"),
                audio_player: AudioPlayer::new().ok(),
                audio_playing: false,
                webcam: WebcamCapture::new(),
                webcam_frame: None,
                webcam_error: None,
                webcam_owned_by_video: false,
                webview_window: WebViewWindow::new(),
            },
            Task::none(),
        )
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = vec![];

        // Webcam frame capture subscription (30 fps when running)
        if self.webcam.is_running() {
            subscriptions.push(
                iced::time::every(Duration::from_millis(33))
                    .map(|_| Message::WebcamFrame)
            );
        }

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => self.dark_mode = !self.dark_mode,
            Message::NavigateTo(page) => {
                self.current_page = page;
                self.show_drawer = false;
            }
            Message::ToggleDrawer => self.show_drawer = !self.show_drawer,
            Message::OpenModal => self.show_modal = true,
            Message::CloseModal => self.show_modal = false,
            Message::InputChanged(value) => self.input_value = value,
            Message::CheckboxToggled(checked) => self.checkbox_checked = checked,
            Message::Checkbox2Toggled(checked) => self.checkbox2_checked = checked,
            Message::ToggleSwitch(value) => self.switch_enabled = value,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::DropdownSelected(option) => self.selected_option = Some(option),
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
            Message::IncrementProgress => {
                self.progress_value = (self.progress_value + 0.1).min(1.0);
                if self.progress_value >= 1.0 {
                    self.progress_value = 0.0;
                }
            }
            Message::TabSelected(index) => self.active_tab = index,
            Message::MediaPlay => self.media_player.play(),
            Message::MediaPause => self.media_player.pause(),
            Message::MediaSeek(pos) => {
                if let Some(duration) = self.media_player.duration {
                    let secs = (pos * duration.as_secs_f32()) as u64;
                    self.media_player
                        .seek(std::time::Duration::from_secs(secs));
                }
            }
            Message::MediaVolume(vol) => self.media_player.set_volume(vol),
            // Audio recorder
            Message::AudioRecordStart => self.audio_recorder.start(),
            Message::AudioRecordStop => self.audio_recorder.stop(),
            Message::AudioRecordPause => self.audio_recorder.pause(),
            Message::AudioRecordResume => self.audio_recorder.resume(),
            // Video recorder
            Message::VideoRecordStart => {
                match self.ensure_video_camera_running() {
                    Ok(()) => {
                        self.webcam_error = None;
                        self.video_recorded_duration = Duration::ZERO;
                        self.video_recording_start = Some(Instant::now());
                        self.video_recorder.start();
                    }
                    Err(e) => {
                        self.webcam_error = Some(e);
                        self.video_recorder.error();
                    }
                }
            }
            Message::VideoRecordStop => {
                self.finalize_video_duration();
                self.video_recorder.stop();
                self.video_recorder.finish();
                self.reset_video_timers();
                self.stop_camera_if_video_owned();
            }
            Message::VideoRecordPause => {
                if let Some(start) = self.video_recording_start.take() {
                    self.video_recorded_duration += start.elapsed();
                }
                self.video_recorder.pause();
            }
            Message::VideoRecordResume => {
                if !self.webcam.is_running() {
                    if let Err(e) = self.ensure_video_camera_running() {
                        self.webcam_error = Some(e);
                        self.video_recorder.error();
                        return Task::none();
                    }
                }
                self.video_recording_start = Some(Instant::now());
                self.video_recorder.resume();
            }
            Message::VideoToggleAudio => self.video_recorder.toggle_audio(),
            Message::VideoToggleVideo => self.video_recorder.toggle_video(),
            // WebView
            Message::UrlInputChanged(url) => self.url_input = url,
            Message::WebViewNavigate => {
                self.webview_state.set_url(&self.url_input);
                self.webview_state.set_loading(true);
            }
            Message::WebViewBack => {
                self.webview_state.can_go_back = false;
            }
            Message::WebViewForward => {
                self.webview_state.can_go_forward = false;
            }
            Message::WebViewReload => {
                self.webview_state.set_loading(true);
            }
            // Spinners
            Message::SpinnerTick => {
                self.spinner_progress = (self.spinner_progress + 0.02) % 1.0;
            }
            // Toasts
            Message::ShowToast(variant) => {
                let msg = match variant {
                    ToastVariant::Info => "This is an info toast",
                    ToastVariant::Success => "Operation successful!",
                    ToastVariant::Warning => "Please be careful",
                    ToastVariant::Error => "Something went wrong",
                };
                self.toasts.push((self.toast_id_counter, msg.to_string(), variant));
                self.toast_id_counter += 1;
            }
            Message::CloseToast(id) => {
                self.toasts.retain(|(i, _, _)| *i != id);
            }
            // Color picker
            Message::HexInputChanged(hex) => {
                self.hex_input = hex.clone();
                if let Some(color) = iced_plus_components::color_picker::hex_to_color(&hex) {
                    self.selected_color = color;
                }
            }
            Message::PaletteColorSelected(index) => {
                self.palette_index = Some(index);
                if let Some(&color) = presets::MATERIAL_PRIMARY.get(index) {
                    self.selected_color = color;
                    self.hex_input = color_to_hex(color);
                }
            }
            // TextArea and RichTextEditor
            Message::TextAreaAction(action) => {
                self.textarea_content.perform(action);
            }
            Message::RichTextAction(action) => {
                match action {
                    RichTextAction::Edit(editor_action) => {
                        self.rich_text_content.perform(editor_action);
                    }
                    _ => {
                        // Handle other rich text actions (formatting, etc.)
                        // For now, these are placeholder actions
                    }
                }
            }
            // Real audio player
            Message::PlayTestTone => {
                if let Some(ref player) = self.audio_player {
                    player.play_test_tone(3);
                    self.audio_playing = true;
                }
            }
            Message::StopAudio => {
                if let Some(ref player) = self.audio_player {
                    player.stop();
                    self.audio_playing = false;
                }
            }
            // Webcam
            Message::StartWebcam => match self.webcam.start() {
                Ok(()) => {
                    self.webcam_error = None;
                    self.webcam_owned_by_video = false;
                }
                Err(e) => {
                    self.webcam_error = Some(e);
                }
            },
            Message::StopWebcam => {
                self.webcam.stop();
                self.webcam_frame = None;
                self.webcam_owned_by_video = false;
                if !self.video_recorder.can_start() {
                    self.finalize_video_duration();
                    self.video_recorder.stop();
                    self.video_recorder.finish();
                    self.reset_video_timers();
                }
            }
            Message::WebcamFrame => {
                if let Some((data, width, height)) = self.webcam.capture_frame() {
                    self.webcam_frame =
                        Some(iced::widget::image::Handle::from_rgba(width, height, data));
                }
                if self.video_recorder.is_recording() {
                    let duration = self.current_video_duration();
                    self.video_recorder.update_duration(duration);
                }
            }
            // Real webview
            Message::OpenWebView => {
                if let Err(e) = self.webview_window.open(&self.url_input) {
                    // Show error as toast
                    self.toasts.push((self.toast_id_counter, e, ToastVariant::Error));
                    self.toast_id_counter += 1;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // App bar with navigation
        let app_bar = self.app_bar();

        // Main content based on current page
        let page_content: Element<'_, Message> = match self.current_page {
            Page::Components => self.components_page(),
            Page::Inputs => self.inputs_page(),
            Page::Feedback => self.feedback_page(),
            Page::Media => self.media_page(),
            Page::WebView => self.webview_page(),
        };

        let scrollable_content = scrollable(
            container(page_content)
                .padding(24)
                .width(Length::Fill)
                .max_width(1200),
        )
        .height(Length::Fill);

        let main_content: Element<'_, Message> = column![app_bar, scrollable_content,].into();

        // Wrap with drawer if open
        let with_drawer: Element<'_, Message> = if self.show_drawer {
            let side_nav = self.side_nav();
            drawer(main_content, side_nav, Message::ToggleDrawer)
        } else {
            main_content
        };

        // Wrap with modal if open
        let with_modal: Element<'_, Message> = if self.show_modal {
            Modal::new(with_drawer, self.modal_content())
                .on_backdrop_press(Message::CloseModal)
                .into()
        } else {
            with_drawer
        };

        // Wrap with toast container
        toast_container(
            with_modal,
            &self.toasts,
            Message::CloseToast,
            ToastPosition::TopRight,
        )
    }

    fn app_bar(&self) -> Element<'_, Message> {
        use iced::widget::button;

        let theme_icon: Element<'_, Message> = if self.dark_mode {
            Icon::moon().size(18.0).into()
        } else {
            Icon::sun().size(18.0).into()
        };
        let theme_btn: Element<'_, Message> = button(theme_icon)
            .padding([6, 10])
            .on_press(Message::ToggleTheme)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                match status {
                    button::Status::Active | button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                        text_color: palette.background.base.text,
                        ..Default::default()
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(palette.background.weak.color)),
                        text_color: palette.background.base.text,
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                        text_color: palette.background.weak.text,
                        ..Default::default()
                    },
                }
            })
            .into();

        let modal_btn = Button::primary("Modal")
            .small()
            .on_press(Message::OpenModal);

        let end_content: Element<'_, Message> = row![theme_btn, modal_btn,]
            .spacing(8)
            .align_y(alignment::Vertical::Center)
            .into();

        AppBar::new()
            .title("iced-plus Kitchen Sink")
            .on_menu_toggle(Message::ToggleDrawer)
            .push(
                NavItem::new("Components")
                    .icon_element(Icon::grid().size(16.0))
                    .on_click(Message::NavigateTo(Page::Components))
                    .active(self.current_page == Page::Components),
            )
            .push(
                NavItem::new("Inputs")
                    .icon_element(Icon::edit().size(16.0))
                    .on_click(Message::NavigateTo(Page::Inputs))
                    .active(self.current_page == Page::Inputs),
            )
            .push(
                NavItem::new("Feedback")
                    .icon_element(Icon::chat().size(16.0))
                    .on_click(Message::NavigateTo(Page::Feedback))
                    .active(self.current_page == Page::Feedback),
            )
            .push(
                NavItem::new("Media")
                    .icon_element(Icon::camera().size(16.0))
                    .on_click(Message::NavigateTo(Page::Media))
                    .active(self.current_page == Page::Media),
            )
            .push(
                NavItem::new("WebView")
                    .icon_element(Icon::globe().size(16.0))
                    .on_click(Message::NavigateTo(Page::WebView))
                    .active(self.current_page == Page::WebView),
            )
            .end_content(end_content)
            .into()
    }

    fn side_nav(&self) -> Element<'_, Message> {
        SideNav::new()
            .header("NAVIGATION")
            .push(
                NavItem::new("Components")
                    .icon(IconName::Grid)
                    .on_click(Message::NavigateTo(Page::Components))
                    .active(self.current_page == Page::Components),
            )
            .push(
                NavItem::new("Inputs")
                    .icon(IconName::Edit)
                    .on_click(Message::NavigateTo(Page::Inputs))
                    .active(self.current_page == Page::Inputs),
            )
            .push(
                NavItem::new("Feedback")
                    .icon(IconName::Chat)
                    .on_click(Message::NavigateTo(Page::Feedback))
                    .active(self.current_page == Page::Feedback),
            )
            .push(
                NavItem::new("Media")
                    .icon(IconName::Camera)
                    .on_click(Message::NavigateTo(Page::Media))
                    .active(self.current_page == Page::Media),
            )
            .push(
                NavItem::new("WebView")
                    .icon(IconName::Globe)
                    .on_click(Message::NavigateTo(Page::WebView))
                    .active(self.current_page == Page::WebView),
            )
            .into()
    }

    fn modal_content(&self) -> Element<'_, Message> {
        let close_btn = Button::secondary("Close")
            .small()
            .on_press(Message::CloseModal);

        let card_content = column![
            text("Modal Dialog").size(24),
            Space::with_height(12),
            text("This is a modal overlay demonstrating the Modal component.").size(14),
            Space::with_height(8),
            text("Click outside or press the button to close.").size(12),
            Space::with_height(20),
            close_btn,
        ]
        .align_x(alignment::Horizontal::Center);

        Card::new(card_content)
            .elevation(Elevation::High)
            .padding(32.0)
            .width(Length::Fixed(400.0))
            .into()
    }

    // ============ COMPONENTS PAGE ============

    fn components_page(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(32.0)
            .push(self.page_header("Components", "UI building blocks"))
            .push(self.buttons_section())
            .push(horizontal_rule(1))
            .push(self.cards_section())
            .push(horizontal_rule(1))
            .push(self.tabs_section())
            .push(horizontal_rule(1))
            .push(self.avatars_skeletons_row())
            .push(horizontal_rule(1))
            .push(self.tooltips_section())
            .push(horizontal_rule(1))
            .push(self.color_picker_section())
            .push(Space::with_height(32))
            .into()
    }

    fn page_header<'a>(&self, title: &'a str, subtitle: &'a str) -> Element<'a, Message> {
        VStack::new()
            .spacing(4.0)
            .push(Heading::h1(title))
            .push(Text::new(subtitle).muted())
            .into()
    }

    fn buttons_section(&self) -> Element<'_, Message> {
        let variants: Element<'_, Message> = HStack::new()
            .spacing(8.0)
            .push(Button::primary("Primary").on_press(Message::Increment))
            .push(Button::secondary("Secondary").on_press(Message::Increment))
            .push(Button::outline("Outline").on_press(Message::Increment))
            .push(Button::ghost("Ghost").on_press(Message::Increment))
            .push(Button::destructive("Delete").on_press(Message::Decrement))
            .into();

        let sizes: Element<'_, Message> = HStack::new()
            .spacing(8.0)
            .align(alignment::Vertical::Center)
            .push(Button::primary("XS").extra_small().on_press(Message::Increment))
            .push(Button::primary("Small").small().on_press(Message::Increment))
            .push(Button::primary("Medium").on_press(Message::Increment))
            .push(Button::primary("Large").large().on_press(Message::Increment))
            .into();

        VStack::new()
            .spacing(16.0)
            .push(Heading::h2("Buttons"))
            .push(Text::new("Variants:").muted())
            .push(variants)
            .push(Text::new("Sizes:").muted())
            .push(sizes)
            .into()
    }

    fn cards_section(&self) -> Element<'_, Message> {
        let cards: Element<'_, Message> = ResponsiveRow::new()
            .spacing(12.0)
            .stack_below(BreakpointTier::SM)
            .push(
                Card::new(column![text("Flat"), text("No shadow").size(12)].spacing(4))
                    .elevation(Elevation::Flat)
            )
            .push(
                Card::new(column![text("Low"), text("Subtle").size(12)].spacing(4))
                    .elevation(Elevation::Low)
            )
            .push(
                Card::new(column![text("Medium"), text("Moderate").size(12)].spacing(4))
                    .elevation(Elevation::Medium)
            )
            .push(
                Card::new(column![text("High"), text("Strong").size(12)].spacing(4))
                    .elevation(Elevation::High)
            )
            .into();

        VStack::new()
            .spacing(16.0)
            .push(Heading::h2("Cards"))
            .push(Text::new("Elevation levels for depth hierarchy").muted())
            .push(cards)
            .into()
    }

    fn tabs_section(&self) -> Element<'_, Message> {
        let tabs: Element<'_, Message> = Tabs::new(self.active_tab, Message::TabSelected)
            .push(Tab::new("Overview"))
            .push(Tab::new("Settings"))
            .push(Tab::new("Profile"))
            .push(Tab::new("Help"))
            .tab_width(TabWidth::Equal)
            .height(44.0)
            .into();

        let content = match self.active_tab {
            0 => "Overview: General information and dashboard content.",
            1 => "Settings: Configure your application preferences.",
            2 => "Profile: View and edit your user profile.",
            3 => "Help: Documentation and support resources.",
            _ => "Unknown tab",
        };

        VStack::new()
            .spacing(16.0)
            .push(Heading::h2("Tabs"))
            .push(Text::new("Navigation between views").muted())
            .push(container(tabs).width(Length::Fill).max_width(600))
            .push(Card::new(text(content)).fill_width())
            .into()
    }

    fn avatars_skeletons_row(&self) -> Element<'_, Message> {
        let avatars = self.avatars_section();
        let skeletons = self.skeletons_section();

        ResponsiveRow::new()
            .spacing(24.0)
            .stack_below(BreakpointTier::MD)
            .push(avatars)
            .push(skeletons)
            .into()
    }

    fn avatars_section(&self) -> Element<'_, Message> {
        let avatars: Element<'_, Message> = HStack::new()
            .spacing(12.0)
            .align(alignment::Vertical::Center)
            .push(Avatar::new("John Doe").size(AvatarSize::Small))
            .push(Avatar::new("Jane Smith").size(AvatarSize::Medium))
            .push(Avatar::new("Bob Wilson").size(AvatarSize::Large))
            .push(Avatar::new("Alice").size(AvatarSize::XL).shape(AvatarShape::Rounded))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h3("Avatars"))
            .push(Text::new("Auto-generated from names").muted())
            .push(avatars)
            .into()
    }

    fn skeletons_section(&self) -> Element<'_, Message> {
        let skeletons: Element<'_, Message> = HStack::new()
            .spacing(16.0)
            .align(alignment::Vertical::Center)
            .push(Skeleton::circle().size(48.0))
            .push(
                VStack::new()
                    .spacing(8.0)
                    .push(Skeleton::text().width(Length::Fixed(150.0)))
                    .push(Skeleton::text().width(Length::Fixed(100.0))),
            )
            .push(Skeleton::rounded().width(Length::Fixed(60.0)).height(Length::Fixed(40.0)))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h3("Skeletons"))
            .push(Text::new("Loading placeholders").muted())
            .push(skeletons)
            .into()
    }

    fn tooltips_section(&self) -> Element<'_, Message> {
        let tooltips: Element<'_, Message> = HStack::new()
            .spacing(12.0)
            .push(Tooltip::new(
                Button::primary("Top").small().on_press(Message::Increment),
                "Tooltip on top",
            ).position(TooltipPosition::Top))
            .push(Tooltip::new(
                Button::secondary("Bottom").small().on_press(Message::Increment),
                "Tooltip on bottom",
            ).position(TooltipPosition::Bottom))
            .push(Tooltip::new(
                Button::outline("Left").small().on_press(Message::Increment),
                "Tooltip on left",
            ).position(TooltipPosition::Left))
            .push(Tooltip::new(
                Button::ghost("Right").small().on_press(Message::Increment),
                "Tooltip on right",
            ).position(TooltipPosition::Right))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Tooltips"))
            .push(Text::new("Hover over buttons to see tooltips").muted())
            .push(tooltips)
            .into()
    }

    fn color_picker_section(&self) -> Element<'_, Message> {
        let picker = color_picker_view(
            self.selected_color,
            &self.hex_input,
            Message::HexInputChanged,
        );

        let palette = color_palette(
            presets::MATERIAL_PRIMARY,
            self.palette_index,
            Message::PaletteColorSelected,
        );

        let preview = ColorSwatch::new(self.selected_color).size(48.0);

        let color_section: Element<'_, Message> = VStack::new()
            .spacing(16.0)
            .push(Heading::h3("Color Picker"))
            .push(picker)
            .into();

        let palette_section: Element<'_, Message> = VStack::new()
            .spacing(16.0)
            .push(Heading::h3("Color Palette"))
            .push(Text::new("Material Design colors").muted())
            .push(palette)
            .push(
                HStack::new()
                    .spacing(8.0)
                    .align(alignment::Vertical::Center)
                    .push(Text::new("Selected:"))
                    .push(preview)
                    .push(Text::new(&self.hex_input).muted()),
            )
            .into();

        VStack::new()
            .spacing(16.0)
            .push(Heading::h2("Color Picker"))
            .push(Text::new("Color selection and palettes").muted())
            .push(
                ResponsiveRow::new()
                    .spacing(24.0)
                    .stack_below(BreakpointTier::MD)
                    .push(color_section)
                    .push(palette_section),
            )
            .into()
    }

    // ============ INPUTS PAGE ============

    fn inputs_page(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(32.0)
            .push(self.page_header("Inputs", "Form controls and user input"))
            .push(self.inputs_row())
            .push(horizontal_rule(1))
            .push(self.textarea_section())
            .push(horizontal_rule(1))
            .push(self.rich_text_section())
            .push(horizontal_rule(1))
            .push(self.checkboxes_switches_section())
            .push(horizontal_rule(1))
            .push(self.slider_section())
            .push(horizontal_rule(1))
            .push(self.counter_demo())
            .push(Space::with_height(32))
            .into()
    }

    fn inputs_row(&self) -> Element<'_, Message> {
        let text_inputs = self.text_inputs_section();
        let select_section = self.select_section();

        ResponsiveRow::new()
            .spacing(24.0)
            .stack_below(BreakpointTier::MD)
            .push(text_inputs)
            .push(select_section)
            .into()
    }

    fn text_inputs_section(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Text Inputs"))
            .push(
                TextInput::new("Default input...", &self.input_value)
                    .on_input(Message::InputChanged)
                    .width(Length::Fill),
            )
            .push(
                TextInput::new("Filled variant...", &self.input_value)
                    .filled()
                    .on_input(Message::InputChanged)
                    .width(Length::Fill),
            )
            .push(
                TextInput::new("Password...", &self.input_value)
                    .password()
                    .on_input(Message::InputChanged)
                    .width(Length::Fill),
            )
            .push(Text::new(if self.input_value.is_empty() {
                "Type something above...".to_string()
            } else {
                format!("You typed: {}", self.input_value)
            }).muted())
            .into()
    }

    fn select_section(&self) -> Element<'_, Message> {
        let select: Element<'_, Message> = Select::new(
            &[DropdownOption::Option1, DropdownOption::Option2, DropdownOption::Option3][..],
            self.selected_option,
            Message::DropdownSelected,
        )
        .placeholder("Choose an option...")
        .width(Length::Fill)
        .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Select / Dropdown"))
            .push(Text::new("Pick an option from the list").muted())
            .push(select)
            .push(Text::new(format!(
                "Selected: {}",
                self.selected_option.map_or("None".to_string(), |o| o.to_string())
            )).muted())
            .into()
    }

    fn textarea_section(&self) -> Element<'_, Message> {
        let textarea: Element<'_, Message> = TextArea::new(&self.textarea_content)
            .on_action(Message::TextAreaAction)
            .placeholder("Type your multi-line text here...")
            .height(Length::Fixed(150.0))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Text Area"))
            .push(Text::new("Multi-line text input for longer content").muted())
            .push(textarea)
            .push(Text::new(format!(
                "Characters: {}",
                self.textarea_content.text().len()
            )).muted())
            .into()
    }

    fn rich_text_section(&self) -> Element<'_, Message> {
        let editor: Element<'_, Message> = RichTextEditor::new(&self.rich_text_content)
            .on_action(Message::RichTextAction)
            .placeholder("Start writing with formatting...")
            .height(Length::Fixed(250.0))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Rich Text Editor"))
            .push(Text::new("Text editor with formatting toolbar").muted())
            .push(editor)
            .push(Text::new("Use the toolbar to format text (Bold, Italic, Lists, etc.)").muted())
            .into()
    }

    fn checkboxes_switches_section(&self) -> Element<'_, Message> {
        let checkboxes: Element<'_, Message> = VStack::new()
            .spacing(8.0)
            .push(Checkbox::new("Accept terms and conditions", self.checkbox_checked, Message::CheckboxToggled))
            .push(Checkbox::new("Enable notifications", self.checkbox2_checked, Message::Checkbox2Toggled))
            .into();

        let switches: Element<'_, Message> = HStack::new()
            .spacing(16.0)
            .align(alignment::Vertical::Center)
            .push(Text::new("Feature toggle:"))
            .push(Switch::new(self.switch_enabled, Message::ToggleSwitch))
            .push(if self.switch_enabled { Text::new("On") } else { Text::new("Off").muted() })
            .into();

        let checkboxes_section: Element<'_, Message> = VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Checkboxes"))
            .push(checkboxes)
            .into();

        let switches_section: Element<'_, Message> = VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Switches"))
            .push(switches)
            .into();

        ResponsiveRow::new()
            .spacing(24.0)
            .stack_below(BreakpointTier::MD)
            .push(checkboxes_section)
            .push(switches_section)
            .into()
    }

    fn slider_section(&self) -> Element<'_, Message> {
        let slider: Element<'_, Message> = Slider::new(
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged,
        )
        .step(1.0)
        .width(Length::Fill)
        .into();

        let progress = Progress::new(self.slider_value / 100.0).height(8.0);

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Slider"))
            .push(Text::new(format!("Value: {:.0}", self.slider_value)))
            .push(slider)
            .push(Text::new("Progress preview:").muted())
            .push(progress)
            .into()
    }

    fn counter_demo(&self) -> Element<'_, Message> {
        let counter_display = Heading::h1(self.counter.to_string());

        let counter_card = Card::new(
            HStack::new()
                .spacing(24.0)
                .align(alignment::Vertical::Center)
                .push(Button::secondary("−").large().on_press(Message::Decrement))
                .push(counter_display)
                .push(Button::primary("+").large().on_press(Message::Increment)),
        )
        .elevation(Elevation::Medium)
        .padding(24.0);

        let status: Element<'_, Message> = if self.counter > 10 {
            Alert::warning("Counter is getting high!").into()
        } else if self.counter < -5 {
            Alert::error("Counter is very negative!").into()
        } else {
            Alert::info("Try the counter buttons").into()
        };

        let counter_section: Element<'_, Message> = VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Counter Demo"))
            .push(counter_card)
            .into();

        let status_section: Element<'_, Message> = VStack::new()
            .spacing(12.0)
            .push(Heading::h3("Status"))
            .push(status)
            .into();

        ResponsiveRow::new()
            .spacing(24.0)
            .stack_below(BreakpointTier::MD)
            .push(counter_section)
            .push(status_section)
            .into()
    }

    // ============ FEEDBACK PAGE ============

    fn feedback_page(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(32.0)
            .push(self.page_header("Feedback", "User notifications and status"))
            .push(self.badges_section())
            .push(horizontal_rule(1))
            .push(self.alerts_section())
            .push(horizontal_rule(1))
            .push(self.progress_section())
            .push(horizontal_rule(1))
            .push(self.spinners_section())
            .push(horizontal_rule(1))
            .push(self.toasts_section())
            .push(Space::with_height(32))
            .into()
    }

    fn badges_section(&self) -> Element<'_, Message> {
        let badges: Element<'_, Message> = HStack::new()
            .spacing(8.0)
            .push(Badge::new("Default"))
            .push(Badge::new("Primary").primary())
            .push(Badge::new("Success").success())
            .push(Badge::new("Warning").warning())
            .push(Badge::new("Error").error())
            .push(Space::with_width(16))
            .push(Text::new("Count:"))
            .push(Badge::count(42).error())
            .push(Badge::count(7).primary())
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Badges"))
            .push(Text::new("Status indicators and counts").muted())
            .push(badges)
            .into()
    }

    fn alerts_section(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Alerts"))
            .push(Text::new("Contextual feedback messages").muted())
            .push(Alert::info("This is an informational message."))
            .push(Alert::success("Operation completed successfully!").title("Success"))
            .push(Alert::warning("Please review before proceeding.").title("Warning"))
            .push(Alert::error("An error occurred. Please try again.").title("Error"))
            .into()
    }

    fn progress_section(&self) -> Element<'_, Message> {
        let btn = Button::secondary("Increment")
            .small()
            .on_press(Message::IncrementProgress);

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Progress"))
            .push(
                HStack::new()
                    .spacing(12.0)
                    .align(alignment::Vertical::Center)
                    .push(Text::new(format!("{:.0}%", self.progress_value * 100.0)))
                    .push(btn),
            )
            .push(Progress::new(self.progress_value).height(8.0))
            .push(Progress::new(0.75).success().height(8.0))
            .push(Progress::new(0.50).warning().height(8.0))
            .push(Progress::new(0.25).error().height(8.0))
            .into()
    }

    fn spinners_section(&self) -> Element<'_, Message> {
        let spinners: Element<'_, Message> = HStack::new()
            .spacing(24.0)
            .align(alignment::Vertical::Center)
            .push(
                VStack::new()
                    .spacing(8.0)
                    .align(alignment::Horizontal::Center)
                    .push(CircularSpinner::new().size(40.0).progress(self.spinner_progress))
                    .push(Text::new("Circular").size(12.0)),
            )
            .push(
                VStack::new()
                    .spacing(8.0)
                    .align(alignment::Horizontal::Center)
                    .push(container(LinearSpinner::new().width(100.0).height(4.0).progress(self.spinner_progress)).width(Length::Fixed(100.0)))
                    .push(Text::new("Linear").size(12.0)),
            )
            .push(
                VStack::new()
                    .spacing(8.0)
                    .align(alignment::Horizontal::Center)
                    .push(DotsSpinner::new().dot_count(3).dot_size(8.0).progress(self.spinner_progress))
                    .push(Text::new("Dots").size(12.0)),
            )
            .push(
                VStack::new()
                    .spacing(8.0)
                    .align(alignment::Horizontal::Center)
                    .push(PulseSpinner::new().size(24.0).progress(self.spinner_progress))
                    .push(Text::new("Pulse").size(12.0)),
            )
            .into();

        let tick_btn = Button::secondary("Animate")
            .small()
            .on_press(Message::SpinnerTick);

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Spinners"))
            .push(Text::new("Loading indicators (click Animate to progress)").muted())
            .push(spinners)
            .push(tick_btn)
            .into()
    }

    fn toasts_section(&self) -> Element<'_, Message> {
        let buttons: Element<'_, Message> = HStack::new()
            .spacing(8.0)
            .push(Button::primary("Info Toast").small().on_press(Message::ShowToast(ToastVariant::Info)))
            .push(Button::secondary("Success").small().on_press(Message::ShowToast(ToastVariant::Success)))
            .push(Button::outline("Warning").small().on_press(Message::ShowToast(ToastVariant::Warning)))
            .push(Button::destructive("Error").small().on_press(Message::ShowToast(ToastVariant::Error)))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Toasts"))
            .push(Text::new("Click buttons to show toast notifications (top-right corner)").muted())
            .push(buttons)
            .push(Text::new(format!("Active toasts: {}", self.toasts.len())).muted())
            .into()
    }

    // ============ MEDIA PAGE ============

    fn media_page(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(32.0)
            .push(self.page_header("Media", "Images, audio, video, and recording"))
            .push(self.images_section())
            .push(horizontal_rule(1))
            .push(self.real_audio_section())
            .push(horizontal_rule(1))
            .push(self.webcam_section())
            .push(horizontal_rule(1))
            .push(self.audio_section())
            .push(horizontal_rule(1))
            .push(self.audio_recorder_section())
            .push(horizontal_rule(1))
            .push(self.video_recorder_section())
            .push(Space::with_height(32))
            .into()
    }

    fn real_audio_section(&self) -> Element<'_, Message> {
        let status = if self.audio_player.is_some() {
            if self.audio_playing {
                "Playing test tone..."
            } else {
                "Ready to play"
            }
        } else {
            "Audio not available (build with --features audio)"
        };

        let play_btn = Button::primary("Play Test Tone")
            .on_press(Message::PlayTestTone);
        let stop_btn = Button::secondary("Stop")
            .on_press(Message::StopAudio);

        let controls: Element<'_, Message> = row![play_btn, stop_btn]
            .spacing(12)
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Real Audio Playback"))
            .push(Text::new("Uses rodio for actual audio output").muted())
            .push(Card::new(
                column![
                    Text::new(status),
                    Space::with_height(12),
                    controls,
                ]
            ).fill_width().padding(16.0))
            .into()
    }

    fn webcam_section(&self) -> Element<'_, Message> {
        let (width, height) = self.webcam.resolution();

        let preview: Element<'_, Message> = if let Some(ref handle) = self.webcam_frame {
            iced::widget::image(handle.clone())
                .width(Length::Fixed(width as f32))
                .height(Length::Fixed(height as f32))
                .into()
        } else if let Some(ref error) = self.webcam_error {
            container(
                Text::new(format!("Error: {}", error)).muted()
            )
            .width(Length::Fixed(width as f32))
            .height(Length::Fixed(height as f32))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
                    ..Default::default()
                }
            })
            .into()
        } else {
            container(
                Text::new(if self.webcam.is_running() { "Starting camera..." } else { "Camera not started" }).muted()
            )
            .width(Length::Fixed(width as f32))
            .height(Length::Fixed(height as f32))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
                    ..Default::default()
                }
            })
            .into()
        };

        let start_btn = Button::primary("Start Webcam")
            .on_press(Message::StartWebcam);
        let stop_btn = Button::secondary("Stop Webcam")
            .on_press(Message::StopWebcam);
        let capture_btn = Button::outline("Capture Frame")
            .on_press(Message::WebcamFrame);

        let controls: Element<'_, Message> = row![start_btn, stop_btn, capture_btn]
            .spacing(12)
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Webcam"))
            .push(Text::new("Uses nokhwa for camera capture (build with --features webcam)").muted())
            .push(Card::new(
                column![
                    preview,
                    Space::with_height(12),
                    controls,
                ]
            ).fill_width().padding(16.0))
            .into()
    }

    fn images_section(&self) -> Element<'_, Message> {
        let placeholders: Element<'_, Message> = ResponsiveRow::new()
            .spacing(16.0)
            .stack_below(BreakpointTier::SM)
            .push(ImagePlaceholder::new().width(Length::Fixed(120.0)).height(Length::Fixed(90.0)).message("120x90"))
            .push(ImagePlaceholder::new().width(Length::Fixed(160.0)).height(Length::Fixed(120.0)).message("160x120"))
            .push(ImagePlaceholder::new().width(Length::Fixed(200.0)).height(Length::Fixed(150.0)).message("200x150"))
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Images"))
            .push(Text::new("Image placeholders (use Image component with actual image files)").muted())
            .push(placeholders)
            .into()
    }

    fn audio_section(&self) -> Element<'_, Message> {
        let controls: Element<'_, Message> = AudioControls::new(&self.media_player)
            .on_play(Message::MediaPlay)
            .on_pause(Message::MediaPause)
            .on_seek(Message::MediaSeek)
            .on_volume(Message::MediaVolume)
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Audio Player"))
            .push(Text::new("Audio player UI (connect to actual audio backend)").muted())
            .push(Card::new(controls).fill_width().padding(16.0))
            .into()
    }

    fn audio_recorder_section(&self) -> Element<'_, Message> {
        let recorder: Element<'_, Message> = AudioRecorder::new(&self.audio_recorder)
            .on_start(Message::AudioRecordStart)
            .on_stop(Message::AudioRecordStop)
            .on_pause(Message::AudioRecordPause)
            .on_resume(Message::AudioRecordResume)
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Audio Recorder"))
            .push(Text::new("Record audio (connect to platform microphone)").muted())
            .push(Card::new(recorder).fill_width().padding(16.0))
            .into()
    }

    fn video_recorder_section(&self) -> Element<'_, Message> {
        let recorder: Element<'_, Message> = VideoRecorder::new(&self.video_recorder)
            .on_start(Message::VideoRecordStart)
            .on_stop(Message::VideoRecordStop)
            .on_pause(Message::VideoRecordPause)
            .on_resume(Message::VideoRecordResume)
            .on_toggle_audio(Message::VideoToggleAudio)
            .on_toggle_video(Message::VideoToggleVideo)
            .into();

        VStack::new()
            .spacing(12.0)
            .push(Heading::h2("Video Recorder"))
            .push(Text::new("Record video (connect to platform camera)").muted())
            .push(Card::new(recorder).fill_width().padding(16.0))
            .into()
    }

    // ============ WEBVIEW PAGE ============

    fn webview_page(&self) -> Element<'_, Message> {
        VStack::new()
            .spacing(32.0)
            .push(self.page_header("WebView", "Browser integration"))
            .push(self.browser_section())
            .push(Space::with_height(32))
            .into()
    }

    fn browser_section(&self) -> Element<'_, Message> {
        use iced::widget::text_input;

        // URL input bar
        let url_bar: Element<'_, Message> = row![
            Button::secondary("Back")
                .small()
                .on_press(Message::WebViewBack),
            Button::secondary("Fwd")
                .small()
                .on_press(Message::WebViewForward),
            Button::secondary("Reload")
                .small()
                .on_press(Message::WebViewReload),
            text_input("Enter URL...", &self.url_input)
                .on_input(Message::UrlInputChanged)
                .on_submit(Message::WebViewNavigate)
                .width(Length::Fill),
            Button::primary("Go")
                .small()
                .on_press(Message::WebViewNavigate),
        ]
        .spacing(8)
        .align_y(alignment::Vertical::Center)
        .into();

        // Browser bar component
        let browser_bar: Element<'_, Message> = BrowserBar::new(&self.webview_state)
            .on_back(Message::WebViewBack)
            .on_forward(Message::WebViewForward)
            .on_reload(Message::WebViewReload)
            .into();

        // System browser launcher section
        let webview_status = if self.webview_window.is_open() {
            "URL opened in browser"
        } else {
            "Click 'Open in Browser' to launch system browser"
        };

        let open_webview_btn = Button::primary("Open in Browser")
            .on_press(Message::OpenWebView);

        let real_webview_section: Element<'_, Message> = Card::new(
            column![
                row![
                    Icon::globe().size(24.0),
                    Space::with_width(12),
                    text("System Browser").size(18),
                ]
                .align_y(alignment::Vertical::Center),
                Space::with_height(12),
                text(format!("URL: {}", self.url_input)).size(14),
                Space::with_height(8),
                Text::new(webview_status).muted(),
                Space::with_height(16),
                open_webview_btn,
                Space::with_height(8),
                Text::new("Opens URL in your default browser").muted(),
            ]
            .spacing(4),
        )
        .elevation(Elevation::Medium)
        .fill_width()
        .padding(20.0)
        .into();

        // Placeholder for embedded webview content (future)
        let webview_placeholder: Element<'_, Message> = Card::new(
            column![
                text("Embedded WebView Content Area").size(18),
                Space::with_height(8),
                text(format!("URL: {}", self.webview_state.url)).size(14),
                Space::with_height(4),
                text(if self.webview_state.loading {
                    "Status: Loading..."
                } else {
                    "Status: Ready"
                })
                .size(12),
                Space::with_height(16),
                text("Note: Embedded webview requires platform integration").size(11),
            ]
            .spacing(4)
            .align_x(alignment::Horizontal::Center),
        )
        .elevation(Elevation::Low)
        .width(Length::Fill)
        .height(Length::Fixed(300.0))
        .into();

        VStack::new()
            .spacing(16.0)
            .push(Heading::h2("Browser"))
            .push(Text::new("Browser integration and URL handling").muted())
            .push(url_bar)
            .push(Heading::h3("Open in System Browser"))
            .push(real_webview_section)
            .push(Heading::h3("BrowserBar Component"))
            .push(Card::new(browser_bar).fill_width().padding(8.0))
            .push(Heading::h3("Embedded WebView (Future)"))
            .push(webview_placeholder)
            .into()
    }

    fn ensure_video_camera_running(&mut self) -> Result<(), String> {
        if self.webcam.is_running() {
            return Ok(());
        }

        match self.webcam.start() {
            Ok(()) => {
                self.webcam_owned_by_video = true;
                Ok(())
            }
            Err(e) => {
                self.webcam_owned_by_video = false;
                Err(e)
            }
        }
    }

    fn stop_camera_if_video_owned(&mut self) {
        if self.webcam_owned_by_video {
            self.webcam.stop();
            self.webcam_frame = None;
            self.webcam_owned_by_video = false;
        }
    }

    fn finalize_video_duration(&mut self) {
        if let Some(start) = self.video_recording_start.take() {
            self.video_recorded_duration += start.elapsed();
        }
        self.video_recorder
            .update_duration(self.video_recorded_duration);
    }

    fn reset_video_timers(&mut self) {
        self.video_recorded_duration = Duration::ZERO;
        self.video_recording_start = None;
    }

    fn current_video_duration(&self) -> Duration {
        if let Some(start) = self.video_recording_start {
            self.video_recorded_duration + start.elapsed()
        } else {
            self.video_recorded_duration
        }
    }
}
