// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Printer_ExtendedDetectedErrorState
//////////////////////////////////////////////

/// Printer_ExtendedDetectedErrorState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Printer_ExtendedDetectedErrorState {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// No_Error
    #[serde(rename = "No_Error")]
    NoError = 2,
    /// Low_Paper
    #[serde(rename = "Low_Paper")]
    LowPaper = 3,
    /// No_Paper
    #[serde(rename = "No_Paper")]
    NoPaper = 4,
    /// Low_Toner
    #[serde(rename = "Low_Toner")]
    LowToner = 5,
    /// No_Toner
    #[serde(rename = "No_Toner")]
    NoToner = 6,
    /// Door_Open
    #[serde(rename = "Door_Open")]
    DoorOpen = 7,
    /// Jammed
    #[serde(rename = "Jammed")]
    Jammed = 8,
    /// Service_Requested
    #[serde(rename = "Service_Requested")]
    ServiceRequested = 9,
    /// Output_Bin_Full
    #[serde(rename = "Output_Bin_Full")]
    OutputBinFull = 10,
    /// Paper_Problem
    #[serde(rename = "Paper_Problem")]
    PaperProblem = 11,
    /// Cannot_Print_Page
    #[serde(rename = "Cannot_Print_Page")]
    CannotPrintPage = 12,
    /// User_Intervention_Required
    #[serde(rename = "User_Intervention_Required")]
    UserInterventionRequired = 13,
    /// Out_of_Memory
    #[serde(rename = "Out_of_Memory")]
    OutOfMemory = 14,
    /// Server_Unknown
    #[serde(rename = "Server_Unknown")]
    ServerUnknown = 15,
}

impl Default for Printer_ExtendedDetectedErrorState {
    fn default() -> Self {
        Self::Unknown
    }
}

