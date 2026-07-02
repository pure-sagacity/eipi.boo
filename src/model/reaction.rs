#[derive(Clone, Copy, Debug)]
pub struct ReactionDef {
    pub token: &'static str,
    pub meaning: &'static str,
}

#[derive(Clone, Debug)]
pub struct ReactionCount {
    pub token: String,
    pub count: i64,
}

pub const ALL: [ReactionDef; 8] = [
    ReactionDef {
        token: "♥",
        meaning: "support / care",
    },
    ReactionDef {
        token: "==",
        meaning: "same / i relate",
    },
    ReactionDef {
        token: "...",
        meaning: "heavy / speechless",
    },
    ReactionDef {
        token: "!!",
        meaning: "shock / intense",
    },
    ReactionDef {
        token: ":)",
        meaning: "warm / funny",
    },
    ReactionDef {
        token: ":/",
        meaning: "awkward / uneasy",
    },
    ReactionDef {
        token: "~~",
        meaning: "soft / tender",
    },
    ReactionDef {
        token: "**",
        meaning: "beautiful / striking",
    },
];

pub fn find_index(token: &str) -> Option<usize> {
    ALL.iter().position(|reaction| reaction.token == token)
}

pub fn meaning_at(index: usize) -> &'static str {
    ALL.get(index)
        .map(|reaction| reaction.meaning)
        .unwrap_or_default()
}

pub fn token_at(index: usize) -> &'static str {
    ALL.get(index)
        .map(|reaction| reaction.token)
        .unwrap_or_default()
}
