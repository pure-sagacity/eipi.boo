use crate::server::input::InputMode;

#[derive(Clone, Copy)]
pub struct KeybindHint {
    pub key: &'static str,
    pub label: &'static str,
}

pub const HELP_KEYBINDS: &[KeybindHint] = &[
    KeybindHint {
        key: "move",
        label: "↑↓←→ / hjkl",
    },
    KeybindHint {
        key: "mouse",
        label: "🖱 click / wheel",
    },
    KeybindHint {
        key: "tab",
        label: "select next confession",
    },
    KeybindHint {
        key: "enter",
        label: "open replies",
    },
    KeybindHint {
        key: "space",
        label: "open feed",
    },
    KeybindHint {
        key: "v",
        label: "upvote",
    },
    KeybindHint {
        key: "n",
        label: "new confession",
    },
    KeybindHint {
        key: "/",
        label: "search",
    },
    KeybindHint {
        key: "r",
        label: "reply",
    },
    KeybindHint {
        key: "T",
        label: "themes",
    },
    KeybindHint {
        key: "G",
        label: "jump to latest",
    },
];

const BROWSE_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "↑↓/jk",
        label: "scroll",
    },
    KeybindHint {
        key: "G",
        label: "last",
    },
    KeybindHint {
        key: "tab",
        label: "select",
    },
    KeybindHint {
        key: "v",
        label: "vote",
    },
    KeybindHint {
        key: "⏎",
        label: "replies",
    },
    KeybindHint {
        key: "␣",
        label: "feed",
    },
    KeybindHint {
        key: "/",
        label: "search",
    },
    KeybindHint {
        key: "n",
        label: "confess",
    },
    KeybindHint {
        key: "T",
        label: "theme",
    },
    KeybindHint {
        key: "q",
        label: "quit",
    },
];

const CARD_VIEW_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "←→/hl",
        label: "prev/next",
    },
    KeybindHint {
        key: "v",
        label: "vote",
    },
    KeybindHint {
        key: "⏎",
        label: "replies",
    },
    KeybindHint {
        key: "/",
        label: "search",
    },
    KeybindHint {
        key: "n",
        label: "confess",
    },
    KeybindHint {
        key: "T",
        label: "theme",
    },
    KeybindHint {
        key: "␣",
        label: "canvas",
    },
];

const VIEW_REPLIES_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "r",
        label: "reply",
    },
    KeybindHint {
        key: "↑↓/jk",
        label: "scroll",
    },
    KeybindHint {
        key: "v",
        label: "vote",
    },
    KeybindHint {
        key: "esc",
        label: "back",
    },
];

const SEARCH_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "⏎",
        label: "search",
    },
    KeybindHint {
        key: "esc",
        label: "cancel",
    },
];

const SEARCH_RESULTS_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "←→/hl",
        label: "prev/next",
    },
    KeybindHint {
        key: "v",
        label: "vote",
    },
    KeybindHint {
        key: "⏎",
        label: "replies",
    },
    KeybindHint {
        key: "esc",
        label: "back",
    },
];

const COMPOSE_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "⏎",
        label: "submit",
    },
    KeybindHint {
        key: "esc",
        label: "cancel",
    },
];

const COMPOSE_REPLY_NAME_HINTS: &[KeybindHint] = &[
    KeybindHint {
        key: "⏎",
        label: "next",
    },
    KeybindHint {
        key: "esc",
        label: "cancel",
    },
];

pub fn status_hints(mode: InputMode, reply_name_phase: bool) -> &'static [KeybindHint] {
    match mode {
        InputMode::Browse => BROWSE_HINTS,
        InputMode::CardView => CARD_VIEW_HINTS,
        InputMode::ViewReplies => VIEW_REPLIES_HINTS,
        InputMode::Search => SEARCH_HINTS,
        InputMode::SearchResults => SEARCH_RESULTS_HINTS,
        InputMode::Compose => COMPOSE_HINTS,
        InputMode::ComposeReply if reply_name_phase => COMPOSE_REPLY_NAME_HINTS,
        InputMode::ComposeReply => COMPOSE_HINTS,
        InputMode::ConfirmQuit | InputMode::Splash | InputMode::ThemePicker | InputMode::Help => {
            &[]
        }
    }
}
