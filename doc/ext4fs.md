# Ext4fs porting

## 简述

本文档记录 Starry 适配 ext4fs 的过程和使用方法。

> Starry 使用的 ext4fs 套件有如下两种：
>
> - lwext4_rust：https://github.com/elliott10/lwext4_rust/tree/main
> - ext4_rs：https://github.com/yuoo655/ext4_rs
>
> 在此感谢 [yuoo655](https://github.com/yuoo655) 和 [elliott10](https://github.com/elliott10/) 两位工程师的支持。



## 适配方法

Starry 将文件系统相关功能集中在 `axfs` 模块。

> 如本地没有`axfs`模块，请参照根目录的 README 的 `Pull crates to local workspace` 章节将模块拉取到本地。

在`axfs/src/fs`中需要令外部的文件系统实现以下几个内容：

1. 文件系统的`new`方法：需要初始化一个文件系统实例，作为全局文件系统的对象
2. `VfsOps`泛型：提供对文件系统的访问操作
3. `VfsNodeOps`泛型：提供对文件和目录的访问和修改操作

对于每一个文件系统套件，按照自己的逻辑实现了以上的内容，集中在`fs`文件夹中。

在完成了文件系统的适配之后，需要根据编译条件选择启动的文件系统类型，这部分内容在`axfs/src/root.rs:init_rootfs`。

```rust
cfg_if::cfg_if! {
    if #[cfg(feature = "myfs")] { // override the default filesystem
        let main_fs = fs::myfs::new_myfs(disk);
    } else if #[cfg(feature = "lwext4_rust")] {
        static EXT4_FS: LazyInit<Arc<fs::lwext4_rust::Ext4FileSystem>> = LazyInit::new();
        EXT4_FS.init_by(Arc::new(fs::lwext4_rust::Ext4FileSystem::new(disk)));
        let main_fs = EXT4_FS.clone();
    } else if #[cfg(feature = "ext4_rs")] {
        static EXT4_FS: LazyInit<Arc<fs::ext4_rs::Ext4FileSystem>> = LazyInit::new();
        EXT4_FS.init_by(Arc::new(fs::ext4_rs::Ext4FileSystem::new(disk)));
        let main_fs = EXT4_FS.clone();
    } else if #[cfg(feature = "fatfs")] {
        // default to be fatfs
        static FAT_FS: LazyInit<Arc<fs::fatfs::FatFileSystem>> = LazyInit::new();
        FAT_FS.init_by(Arc::new(fs::fatfs::FatFileSystem::new(disk)));
        FAT_FS.init();
        let main_fs = FAT_FS.clone();
    }
}
```

相关的 feature 定义在 Cargo.toml 中，利用了 Rust 的条件编译机制和 if-else 匹配机制保证每次只会有一个文件系统被实例化。

按照如上操作进行，即可完成文件系统的适配。



> Tips：Starry 对块设备的读写默认以 512 字节为一个块大小，因此对于 ext4_rs 的读写做了特判。



## 编译运行

```sh
# Run in the lwext4fs with Rust interface, whose url is https://github.com/elliott10/lwext4_rust.
make A=apps/monolithic_userboot FEATURES=lwext4_rust LOG=off ACCEL=n run

# Run in a new ext4fs written in Rust, whose url is https://github.com/yuoo655/ext4_rs.
make A=apps/monolithic_userboot FEATURES=ext4_rs LOG=off ACCEL=n run

# Replace virt-io with ram-disk
make A=apps/monolithic_userboot FEATURES=ext4_rs,img LOG=error ARCH=x86_64 ACCEL=n run

# Run testcases for OSCOMP
make A=apps/monolithic_userboot FEATURES=ext4_rs,img LOG=off ACCEL=n run APP_FEATURES=batch
```



## 注意事项

直接使用 ext4fs 运行比赛测例时，运行速度相比 fat32 会明显变慢，可能包括如下原因（分析来自 [yuoo655](https://github.com/yuoo655) ）：

1. ext4fs 有校验和，读取写入的时候都会检查校验和。会比 fat32 多很多次 io，比如创建一个 inode。涉及到多种元数据改动。 这些元数据设置校验和后需要写回磁盘，读文件。读目录。元数据都会有相应的检查。如果加入了 pagecache 或者 blockcache，可能会有所改善。
2. 对于小文件，ext4 没有优势。大文件的话。由于 B+ 树的 extent 特性。可以一次性读写比较大的数量。

