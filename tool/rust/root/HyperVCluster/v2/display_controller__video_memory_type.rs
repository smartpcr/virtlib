// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DisplayController_VideoMemoryType
//////////////////////////////////////////////

/// DisplayController_VideoMemoryType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DisplayController_VideoMemoryType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// VRAM
    #[serde(rename = "VRAM")]
    VRAM = 2,
    /// DRAM
    #[serde(rename = "DRAM")]
    DRAM = 3,
    /// SRAM
    #[serde(rename = "SRAM")]
    SRAM = 4,
    /// WRAM
    #[serde(rename = "WRAM")]
    WRAM = 5,
    /// EDO_RAM
    #[serde(rename = "EDO_RAM")]
    EDORAM = 6,
    /// Burst_Synchronous_DRAM
    #[serde(rename = "Burst_Synchronous_DRAM")]
    BurstSynchronousDRAM = 7,
    /// Pipelined_Burst_SRAM
    #[serde(rename = "Pipelined_Burst_SRAM")]
    PipelinedBurstSRAM = 8,
    /// CDRAM
    #[serde(rename = "CDRAM")]
    CDRAM = 9,
    /// _3DRAM
    #[serde(rename = "_3DRAM")]
    V3DRAM = 10,
    /// SDRAM
    #[serde(rename = "SDRAM")]
    SDRAM = 11,
    /// SGRAM
    #[serde(rename = "SGRAM")]
    SGRAM = 12,
}

impl Default for DisplayController_VideoMemoryType {
    fn default() -> Self {
        Self::Unknown
    }
}

