//! Set the same branded icon on the native window and Windows taskbar.
use bevy::{ecs::system::NonSendMarker, prelude::*, winit::WINIT_WINDOWS};
use winit::window::Icon;

pub fn set_icon(_main_thread: NonSendMarker, mut applied: Local<bool>) {
    if *applied {
        return;
    }
    // Bevy 0.20 stores its native windows in main-thread-local storage rather
    // than an ECS NonSend resource. The marker keeps this access on that thread.
    *applied = WINIT_WINDOWS.with_borrow(|windows| {
        if windows.windows.is_empty() {
            return false;
        }
        let decoded = image::load_from_memory_with_format(
            include_bytes!("../../../assets/icons/app-128.png"),
            image::ImageFormat::Png,
        );
        let image = match decoded {
            Ok(image) => image.into_rgba8(),
            Err(error) => {
                warn!("Cannot decode application icon: {error}");
                return true;
            }
        };
        let (width, height) = image.dimensions();
        let icon = match Icon::from_rgba(image.into_raw(), width, height) {
            Ok(icon) => icon,
            Err(error) => {
                warn!("Cannot create application icon: {error}");
                return true;
            }
        };
        for window in windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
            #[cfg(target_os = "windows")]
            {
                use winit::platform::windows::WindowExtWindows;
                window.set_taskbar_icon(Some(icon.clone()));
            }
        }
        true
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn icon_system_tolerates_window_not_created_yet() {
        let mut app = App::new();
        app.add_systems(Update, set_icon);
        app.update();
        app.update();
    }
}
