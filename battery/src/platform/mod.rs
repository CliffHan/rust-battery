cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;

        pub type Manager = linux::SysFsManager;
        pub type Iterator = linux::SysFsIterator;
        pub type Device = linux::SysFsDevice;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub type Manager = macos::IoKitManager;
        pub type Iterator = macos::IoKitIterator;
        pub type Device = macos::IoKitDevice;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub type Manager = windows::PowerManager;
        pub type Iterator = windows::PowerIterator;
        pub type Device = windows::PowerDevice;
    } else if #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))] {
        mod freebsd;

        pub type Manager = freebsd::IoCtlManager;
        pub type Iterator = freebsd::IoCtlIterator;
        pub type Device = freebsd::IoCtlDevice;
    } else {
        compile_error!("Support for this target OS is not implemented yet!\n \
            You may want to create an issue: https://github.com/svartalf/rust-battery/issues/new");
    }
}

pub mod traits;
