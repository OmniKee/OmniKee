use base64::Engine;
use uuid::Uuid;

pub const KEEPASS_ICONS_TO_MDI: [&'static str; 69] = [
    "mdi-key",
    "mdi-earth",
    "mdi-alert-rhombus",
    "mdi-server-security",
    "mdi-clipboard",
    "mdi-account",
    "mdi-cogs",
    "mdi-notebook-edit",
    "mdi-export-variant",
    "mdi-card-account-details",
    "mdi-at",
    "mdi-camera",
    "mdi-cloud",
    "mdi-key-chain",
    "mdi-power-plug",
    "mdi-projector",
    "mdi-bookmark",
    "mdi-disc",
    "mdi-monitor",
    "mdi-email-newsletter",
    "mdi-cog",
    "mdi-clipboard-check",
    "mdi-file",
    "mdi-newspaper",
    "mdi-lightning-bolt",
    "mdi-safe",
    "mdi-floppy",
    "mdi-network",
    "mdi-movie-roll",
    "mdi-script-text-key",
    "mdi-console",
    "mdi-printer",
    "mdi-vector-rectangle",
    "mdi-checkerboard",
    "mdi-wrench",
    "mdi-play-network",
    "mdi-folder-download",
    "mdi-percent",
    "mdi-microsoft-windows",
    "mdi-clock",
    "mdi-magnify",
    "mdi-image",
    "mdi-memory",
    "mdi-trash-can",
    "mdi-note-text",
    "mdi-close-circle",
    "mdi-help-circle",
    "mdi-archive",
    "mdi-folder",
    "mdi-folder-open",
    "mdi-server",
    "mdi-lock-open",
    "mdi-lock",
    "mdi-check-circle",
    "mdi-pencil",
    "mdi-file-image",
    "mdi-book-account",
    "mdi-table-large",
    "mdi-lectern",
    "mdi-tools",
    "mdi-home",
    "mdi-star",
    "mdi-linux",
    "mdi-android",
    "mdi-apple",
    "mdi-file-word-box",
    "mdi-cash",
    "mdi-certificate",
    "mdi-monitor-cellphone",
];

pub fn get_icon(
    database: &keepass::Database,
    uuid: Option<&Uuid>,
    index: Option<usize>,
) -> Option<String> {
    if let Some(ci) = uuid {
        database.meta.custom_icons.icons.iter().find_map(|icon| {
            if &icon.uuid == ci {
                let engine = base64::engine::general_purpose::STANDARD;
                let encoded = engine.encode(&icon.data);

                Some(format!("data:image/png;base64,{encoded}"))
            } else {
                None
            }
        })
    } else if let Some(i) = index {
        KEEPASS_ICONS_TO_MDI.get(i).map(|v| v.to_string())
    } else {
        None
    }
}
