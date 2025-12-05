// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Printer_ExtendedPrinterStatus
//////////////////////////////////////////////

/// Printer_ExtendedPrinterStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Printer_ExtendedPrinterStatus {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Idle
    #[serde(rename = "Idle")]
    Idle = 3,
    /// Printing
    #[serde(rename = "Printing")]
    Printing = 4,
    /// Warmup
    #[serde(rename = "Warmup")]
    Warmup = 5,
    /// Stopped_Printing
    #[serde(rename = "Stopped_Printing")]
    StoppedPrinting = 6,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 7,
    /// Paused
    #[serde(rename = "Paused")]
    Paused = 8,
    /// Error
    #[serde(rename = "Error")]
    Error = 9,
    /// Busy
    #[serde(rename = "Busy")]
    Busy = 10,
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 11,
    /// Waiting
    #[serde(rename = "Waiting")]
    Waiting = 12,
    /// Processing
    #[serde(rename = "Processing")]
    Processing = 13,
    /// Initialization
    #[serde(rename = "Initialization")]
    Initialization = 14,
    /// Power_Save
    #[serde(rename = "Power_Save")]
    PowerSave = 15,
    /// Pending_Deletion
    #[serde(rename = "Pending_Deletion")]
    PendingDeletion = 16,
    /// I_O_Active
    #[serde(rename = "I_O_Active")]
    IOActive = 17,
    /// Manual_Feed
    #[serde(rename = "Manual_Feed")]
    ManualFeed = 18,
}

impl Default for Printer_ExtendedPrinterStatus {
    fn default() -> Self {
        Self::Other
    }
}

