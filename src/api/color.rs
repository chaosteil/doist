/// Maps the color number used by the Todoist API to a specific color name.
#[derive(
    Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Eq, Ord, PartialOrd,
)]
#[repr(u16)]
pub enum Color {
    Unknown,
    BerryRed = 30,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    Charcoal,
    Grey,
    Taupe,
}

impl Default for Color {
    fn default() -> Self {
        Color::Unknown
    }
}
