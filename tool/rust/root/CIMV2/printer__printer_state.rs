// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Printer_PrinterState
//////////////////////////////////////////////

/// Printer_PrinterState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Printer_PrinterState {
    /// Paused
    #[serde(rename = "Paused")]
    Paused = 0,
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Pending_Deletion
    #[serde(rename = "Pending_Deletion")]
    PendingDeletion = 2,
    /// Paper_Jam
    #[serde(rename = "Paper_Jam")]
    PaperJam = 3,
    /// Paper_Out
    #[serde(rename = "Paper_Out")]
    PaperOut = 4,
    /// Manual_Feed
    #[serde(rename = "Manual_Feed")]
    ManualFeed = 5,
    /// Paper_Problem
    #[serde(rename = "Paper_Problem")]
    PaperProblem = 6,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 7,
    /// IO_Active
    #[serde(rename = "IO_Active")]
    IOActive = 8,
    /// Busy
    #[serde(rename = "Busy")]
    Busy = 9,
    /// Printing
    #[serde(rename = "Printing")]
    Printing = 10,
    /// Output_Bin_Full
    #[serde(rename = "Output_Bin_Full")]
    OutputBinFull = 11,
    /// Not_Available
    #[serde(rename = "Not_Available")]
    NotAvailable = 12,
    /// Waiting
    #[serde(rename = "Waiting")]
    Waiting = 13,
    /// Processing
    #[serde(rename = "Processing")]
    Processing = 14,
    /// Initialization
    #[serde(rename = "Initialization")]
    Initialization = 15,
    /// Warming_Up
    #[serde(rename = "Warming_Up")]
    WarmingUp = 16,
    /// Toner_Low
    #[serde(rename = "Toner_Low")]
    TonerLow = 17,
    /// No_Toner
    #[serde(rename = "No_Toner")]
    NoToner = 18,
    /// Page_Punt
    #[serde(rename = "Page_Punt")]
    PagePunt = 19,
    /// User_Intervention_Required
    #[serde(rename = "User_Intervention_Required")]
    UserInterventionRequired = 20,
    /// Out_of_Memory
    #[serde(rename = "Out_of_Memory")]
    OutOfMemory = 21,
    /// Door_Open
    #[serde(rename = "Door_Open")]
    DoorOpen = 22,
    /// Server_Unknown
    #[serde(rename = "Server_Unknown")]
    ServerUnknown = 23,
    /// Power_Save
    #[serde(rename = "Power_Save")]
    PowerSave = 24,
}

impl Default for Printer_PrinterState {
    fn default() -> Self {
        Self::Paused
    }
}

