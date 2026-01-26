# Input Components

Text input and form components with built-in styling and validation support.

## TextInput

Enhanced text input with label and helper text:

```rust
use iced_plus_components::TextInput;

TextInput::new("Username")
    .value(&self.username)
    .on_input(Message::UsernameChanged)
```

### With Placeholder

```rust
TextInput::new("Email")
    .placeholder("you@example.com")
    .value(&self.email)
    .on_input(Message::EmailChanged)
```

### With Helper Text

```rust
TextInput::new("Password")
    .helper("Must be at least 8 characters")
    .secure(true)
    .value(&self.password)
    .on_input(Message::PasswordChanged)
```

### With Error

```rust
TextInput::new("Email")
    .value(&self.email)
    .error(self.email_error.as_deref())
    .on_input(Message::EmailChanged)
```

### Disabled

```rust
TextInput::new("Read Only")
    .value(&self.value)
    .disabled(true)
```

## TextArea

Multi-line text input:

```rust
use iced_plus_components::TextArea;

TextArea::new("Description")
    .value(&self.description)
    .rows(5)
    .on_input(Message::DescriptionChanged)
```

## Checkbox

Styled checkbox:

```rust
use iced_plus_components::Checkbox;

Checkbox::new("I agree to the terms")
    .checked(self.agreed)
    .on_toggle(Message::AgreedToggled)
```

## Radio Buttons

Single selection from options:

```rust
use iced_plus_components::{Radio, RadioGroup};

// Individual radio
Radio::new("Option A", Choice::A, self.selected)
    .on_click(Message::Selected)

// Radio group
RadioGroup::new("Choose one", self.selected, Message::Selected)
    .push("Option A", Choice::A)
    .push("Option B", Choice::B)
    .push("Option C", Choice::C)
```

## Switch

Toggle switch:

```rust
use iced_plus_components::Switch;

Switch::new(self.enabled)
    .label("Enable notifications")
    .on_toggle(Message::NotificationsToggled)
```

## Slider

Range input:

```rust
use iced_plus_components::Slider;

Slider::new(0.0..=100.0, self.volume)
    .on_change(Message::VolumeChanged)
    .step(1.0)
```

### Vertical Slider

```rust
use iced_plus_components::VerticalSlider;

VerticalSlider::new(0.0..=100.0, self.value)
    .on_change(Message::ValueChanged)
    .height(Length::Fixed(200.0))
```

## Select

Dropdown selection:

```rust
use iced_plus_components::Select;

Select::new("Country", &self.countries, self.selected)
    .on_select(Message::CountrySelected)
    .placeholder("Choose a country...")
```

## Form Example

Complete form using input components:

```rust
fn view(&self) -> Element<Message> {
    VStack::new()
        .spacing(16.0)
        .push(
            TextInput::new("Name")
                .value(&self.name)
                .on_input(Message::NameChanged)
        )
        .push(
            TextInput::new("Email")
                .placeholder("you@example.com")
                .value(&self.email)
                .error(self.email_error.as_deref())
                .on_input(Message::EmailChanged)
        )
        .push(
            Select::new("Country", &self.countries, self.country)
                .on_select(Message::CountrySelected)
        )
        .push(
            Checkbox::new("Subscribe to newsletter")
                .checked(self.subscribe)
                .on_toggle(Message::SubscribeToggled)
        )
        .push(
            HStack::new()
                .spacing(8.0)
                .push(Button::secondary("Cancel").on_press(Message::Cancel))
                .push(Button::primary("Submit").on_press(Message::Submit))
        )
        .into()
}
```
