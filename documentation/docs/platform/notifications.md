# Notifications

Native desktop notifications for iced-plus applications.

## Basic Usage

```rust
use iced_plus_platform::notifications::{Notification, NotificationManager};

// Create a simple notification
Notification::new("Title", "Message body")
    .show();
```

## NotificationManager

For managing multiple notifications:

```rust
let manager = NotificationManager::new("com.myapp.example");

// Show notification
let id = manager.notify(
    Notification::new("Download Complete", "File saved successfully")
);

// Close notification
manager.close(id);
```

## Notification Options

```rust
Notification::new("Title", "Body")
    .subtitle("Optional subtitle")      // macOS only
    .icon(icon_data)                    // Custom icon
    .sound(true)                        // Play sound
    .timeout(Duration::from_secs(5))    // Auto-dismiss
    .urgency(Urgency::Normal)           // Priority level
```

### Urgency Levels

| Level | Behavior |
|-------|----------|
| `Low` | May be silent, shown briefly |
| `Normal` | Standard notification |
| `Critical` | May persist, require interaction |

## Actions

Add interactive buttons:

```rust
Notification::new("Incoming Call", "John Doe")
    .action("answer", "Answer")
    .action("decline", "Decline")
    .on_action(|action| {
        match action {
            "answer" => Message::AnswerCall,
            "decline" => Message::DeclineCall,
            _ => Message::None,
        }
    })
    .show();
```

## Event Handling

Handle notification interactions:

```rust
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::NotificationClicked(id) => {
            // User clicked the notification body
        }
        Message::NotificationAction(id, action) => {
            // User clicked an action button
        }
        Message::NotificationClosed(id) => {
            // Notification was dismissed
        }
        // ...
    }
    Task::none()
}
```

## Platform Notes

### macOS

- Requires app bundle identifier for persistent notifications
- Supports subtitle field
- Actions appear as buttons

### Windows

- Uses Windows Toast notifications
- Supports action buttons
- App must be registered for persistent notifications

### Linux

- Uses freedesktop.org notification spec
- Requires a notification daemon (e.g., dunst, mako)
- Action support varies by daemon

## Best Practices

1. **Keep it brief**: Notifications should be scannable
2. **Be timely**: Only notify for important, time-sensitive events
3. **Provide actions**: Let users respond without opening the app
4. **Respect preferences**: Allow users to disable notifications
5. **Don't spam**: Rate-limit notifications
