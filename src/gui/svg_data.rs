use iced_core::svg::Handle;

/// App icon
pub const ICON: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/icon.svg"));

/// Play song icon
pub const PLAY: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/play.svg"));
/// Pause song icon
pub const PAUSE: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/pause.svg"));

/// Previous song icon
pub const PREV: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/prev.svg"));
/// Next song icon
pub const NEXT: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/next.svg"));

/// Volume icons - icons for each volume level
pub const VOL_100: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/volume_100.svg"));
pub const VOL_66: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/volume_66.svg"));
pub const VOL_33: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/volume_33.svg"));
pub const MUTE: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/volume_muted.svg"));

/// Scrollbar icons
pub const SCROLL_UP: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/scroll_up.svg"));
pub const SCROLL_DOWN: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/scroll_down.svg"));

pub const BIN: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/bin.svg"));
pub const PLUS: SvgData =
    SvgData::new(include_bytes!("../../assets/icons/add.svg"));

/// Gets play or pause icon based on play bool
pub fn pp_icon(play: bool) -> Handle {
    if play {
        PAUSE.into()
    } else {
        PLAY.into()
    }
}

pub fn vol_icon(vol: f32, muted: bool) -> Handle {
    if muted {
        MUTE.into()
    } else if vol < 1. / 3. {
        VOL_33.into()
    } else if vol < 2. / 3. {
        VOL_66.into()
    } else {
        VOL_100.into()
    }
}

#[derive(Clone, Copy)]
pub struct SvgData {
    data: &'static [u8],
}

impl SvgData {
    pub const fn new(data: &'static [u8]) -> Self {
        Self { data }
    }
}

impl Into<Handle> for SvgData {
    fn into(self) -> Handle {
        Handle::from_memory(self.data)
    }
}
