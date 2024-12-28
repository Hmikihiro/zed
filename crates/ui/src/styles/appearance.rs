use crate::prelude::*;
use gpui::{AppContext, Window, WindowBackgroundAppearance};

/// Returns the [WindowBackgroundAppearance].
fn window_appearance(_window: &Window, cx: &AppContext) -> WindowBackgroundAppearance {
    cx.theme().styles.window_background_appearance
}

/// Returns if the window and it's surfaces are expected
/// to be transparent.
///
/// Helps determine if you need to take extra steps to prevent
/// transparent backgrounds.
pub fn window_is_transparent(_window: &Window, cx: &AppContext) -> bool {
    matches!(
        window_appearance(cx),
        WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred
    )
}
