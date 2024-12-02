//! Defines types and probe methods of all supported devices.

#![allow(unused_imports, dead_code)]

use core::ptr::NonNull;

use crate::AxDeviceEnum;
use driver_common::DeviceType;

#[cfg(feature = "virtio")]
use crate::virtio::{self, VirtIoDevMeta};

#[cfg(feature = "bus-pci")]
use driver_pci::{DeviceFunction, DeviceFunctionInfo, PciRoot};

pub use super::dummy::*;

pub trait DriverProbe {
    fn probe_global() -> Option<AxDeviceEnum> {
        None
    }

    #[cfg(bus = "mmio")]
    fn probe_mmio(_mmio_base: usize, _mmio_size: usize) -> Option<AxDeviceEnum> {
        None
    }

    #[cfg(bus = "pci")]
    fn probe_pci(
        _root: &mut PciRoot,
        _bdf: DeviceFunction,
        _dev_info: &DeviceFunctionInfo,
    ) -> Option<AxDeviceEnum> {
        None
    }
}

#[cfg(net_dev = "virtio-net")]
register_net_driver!(
    <virtio::VirtIoNet as VirtIoDevMeta>::Driver,
    <virtio::VirtIoNet as VirtIoDevMeta>::Device
);

#[cfg(block_dev = "virtio-blk")]
register_block_driver!(
    <virtio::VirtIoBlk as VirtIoDevMeta>::Driver,
    <virtio::VirtIoBlk as VirtIoDevMeta>::Device
);

#[cfg(display_dev = "virtio-gpu")]
register_display_driver!(
    <virtio::VirtIoGpu as VirtIoDevMeta>::Driver,
    <virtio::VirtIoGpu as VirtIoDevMeta>::Device
);

cfg_if::cfg_if! {
    if #[cfg(block_dev = "ramdisk")] {
        pub struct RamDiskDriver;
        register_block_driver!(RamDiskDriver, driver_block::ramdisk::RamDisk);

        impl DriverProbe for RamDiskDriver {
            fn probe_global() -> Option<AxDeviceEnum> {
                // TODO: format RAM disk
                Some(AxDeviceEnum::from_block(
                    driver_block::ramdisk::RamDisk::new(0x100_0000), // 16 MiB
                ))
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(block_dev = "bcm2835-sdhci")]{
        pub struct BcmSdhciDriver;
        register_block_driver!(MmckDriver, driver_block::bcm2835sdhci::SDHCIDriver);

        impl DriverProbe for BcmSdhciDriver {
            fn probe_global() -> Option<AxDeviceEnum> {
                debug!("mmc probe");
                driver_block::bcm2835sdhci::SDHCIDriver::try_new().ok().map(AxDeviceEnum::from_block)
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(net_dev = "ixgbe")] {
        use crate::ixgbe::IxgbeHalImpl;
        use axhal::mem::phys_to_virt;
        pub struct IxgbeDriver;
        register_net_driver!(IxgbeDriver, driver_net::ixgbe::IxgbeNic<IxgbeHalImpl, 1024, 1>);
        impl DriverProbe for IxgbeDriver {
            #[cfg(bus = "pci")]
            fn probe_pci(
                    root: &mut driver_pci::PciRoot,
                    bdf: driver_pci::DeviceFunction,
                    dev_info: &driver_pci::DeviceFunctionInfo,
                ) -> Option<crate::AxDeviceEnum> {
                    use driver_net::ixgbe::{INTEL_82599, INTEL_VEND, IxgbeNic};
                    if dev_info.vendor_id == INTEL_VEND && dev_info.device_id == INTEL_82599 {
                        // Intel 10Gb Network
                        info!("ixgbe PCI device found at {:?}", bdf);

                        // Initialize the device
                        // These can be changed according to the requirments specified in the ixgbe init function.
                        const QN: u16 = 1;
                        const QS: usize = 1024;
                        let bar_info = root.bar_info(bdf, 0).unwrap();
                        match bar_info {
                            driver_pci::BarInfo::Memory {
                                address,
                                size,
                                ..
                            } => {
                                let ixgbe_nic = IxgbeNic::<IxgbeHalImpl, QS, QN>::init(
                                    phys_to_virt((address as usize).into()).into(),
                                    size as usize
                                )
                                .expect("failed to initialize ixgbe device");
                                return Some(AxDeviceEnum::from_net(ixgbe_nic));
                            }
                            driver_pci::BarInfo::IO { .. } => {
                                error!("ixgbe: BAR0 is of I/O type");
                                return None;
                            }
                        }
                    }
                    None
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(net_dev = "e1000")] {
        use axalloc::global_allocator;
        use driver_net::e1000::{E1000Nic, KernelFunc};
        use core::alloc::Layout;

        pub struct KernelFuncObj;
        impl KernelFunc for KernelFuncObj {
            /// Allocate consequent physical memory for DMA;
            /// Return (cpu virtual address, dma physical address) which is page aligned.
            //fn dma_alloc_coherent(pages: usize) -> usize;
            fn dma_alloc_coherent(&mut self, pages: usize) -> (usize, usize) {
            let vaddr = if let Ok(start_vaddr) = global_allocator().alloc_pages(pages, Self::PAGE_SIZE) {
                        start_vaddr
                } else {
                    error!("failed to alloc pages");
                    return (0, 0);
                };
                let paddr = axhal::mem::virt_to_phys((vaddr).into());
                info!("dma_alloc_coherent pages @ vaddr={:#x}, paddr={:#x}", vaddr, paddr);
                (vaddr, paddr.as_usize())
            }

            /// Deallocate DMA memory by virtual address
            fn dma_free_coherent(&mut self, vaddr: usize, pages: usize) {
                global_allocator().dealloc_pages(vaddr, pages);
            }
        }

        pub struct E1000Driver;
        register_net_driver!(E1000Driver, driver_net::e1000::E1000Nic<'static, KernelFuncObj>);


        impl DriverProbe for E1000Driver {
            #[cfg(bus = "pci")]
            fn probe_pci(
                    root: &mut driver_pci::PciRoot,
                    bdf: driver_pci::DeviceFunction,
                    dev_info: &driver_pci::DeviceFunctionInfo,
                ) -> Option<crate::AxDeviceEnum> {
                    const E1000_VENDOR_ID: u16 = 0x8086;
                    const E1000_DEVICE_ID: u16 = 0x100e;
                    info!("PCI vendor:device = {:#x}:{:#x}", dev_info.vendor_id, dev_info.device_id);
                    if dev_info.vendor_id == E1000_VENDOR_ID && dev_info.device_id == E1000_DEVICE_ID {
                        info!("E1000 PCI device found at {:?}", bdf);

                        // Initialize the device
                        match root.bar_info(bdf, 0).unwrap() {
                            driver_pci::BarInfo::Memory {
                                address,
                                ..
                            } => {
                                let kfn = KernelFuncObj;
                                let nic = E1000Nic::<KernelFuncObj>::init(
                                    kfn,
                                    axhal::mem::phys_to_virt((address as usize).into()).into()
                                )
                                .expect("failed to initialize e1000 device");
                                return Some(AxDeviceEnum::from_net(nic));
                            }
                            driver_pci::BarInfo::IO { .. } => {
                                error!("e1000: BAR0 is of I/O type");
                                return None;
                            }
                        }
                    }
                    None
            }
        }
    }
}
