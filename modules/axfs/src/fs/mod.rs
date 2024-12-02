cfg_if::cfg_if! {
    if #[cfg(feature = "myfs")] {
        pub mod myfs;
        /// The block size of the file system.
        pub const BLOCK_SIZE: usize = 512;
    } else if #[cfg(feature = "lwext4_rust")] {
        pub mod lwext4_rust;
        pub use lwext4_rust::BLOCK_SIZE;
    } else if #[cfg(feature = "ext4_rs")] {
        pub mod ext4_rs;
        pub use ext4_rs::BLOCK_SIZE;
    } else if #[cfg(feature = "another_ext4")] {
        pub mod another_ext4;
        pub use another_ext4::BLOCK_SIZE;
    } else if #[cfg(feature = "fatfs")] {
        // default to be fatfs
        pub mod fatfs;
        pub use fatfs::BLOCK_SIZE;
    }
}

#[cfg(feature = "devfs")]
pub use axfs_devfs as devfs;

#[cfg(feature = "ramfs")]
pub use axfs_ramfs as ramfs;
