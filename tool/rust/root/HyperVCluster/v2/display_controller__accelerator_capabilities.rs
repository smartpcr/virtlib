// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DisplayController_AcceleratorCapabilities
//////////////////////////////////////////////

/// DisplayController_AcceleratorCapabilities enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DisplayController_AcceleratorCapabilities {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Graphics_Accelerator
    #[serde(rename = "Graphics_Accelerator")]
    GraphicsAccelerator = 2,
    /// _3D_Accelerator
    #[serde(rename = "_3D_Accelerator")]
    V3DAccelerator = 3,
    /// PCI_Fast_Write
    #[serde(rename = "PCI_Fast_Write")]
    PCIFastWrite = 4,
    /// MultiMonitor_Support
    #[serde(rename = "MultiMonitor_Support")]
    MultiMonitorSupport = 5,
    /// PCI_Mastering
    #[serde(rename = "PCI_Mastering")]
    PCIMastering = 6,
    /// Second_Monochrome_Adapter_Support
    #[serde(rename = "Second_Monochrome_Adapter_Support")]
    SecondMonochromeAdapterSupport = 7,
    /// Large_Memory_Address_Support
    #[serde(rename = "Large_Memory_Address_Support")]
    LargeMemoryAddressSupport = 8,
}

impl Default for DisplayController_AcceleratorCapabilities {
    fn default() -> Self {
        Self::Unknown
    }
}

