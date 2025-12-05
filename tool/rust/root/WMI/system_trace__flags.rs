// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SystemTrace_Flags
//////////////////////////////////////////////

/// SystemTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SystemTrace_Flags {
    /// process
    #[serde(rename = "process")]
    Process = 1,
    /// thread
    #[serde(rename = "thread")]
    Thread = 2,
    /// img
    #[serde(rename = "img")]
    Img = 3,
    /// proccntr
    #[serde(rename = "proccntr")]
    Proccntr = 4,
    /// cswitch
    #[serde(rename = "cswitch")]
    Cswitch = 5,
    /// dpc
    #[serde(rename = "dpc")]
    Dpc = 6,
    /// isr
    #[serde(rename = "isr")]
    Isr = 7,
    /// syscall
    #[serde(rename = "syscall")]
    Syscall = 8,
    /// disk
    #[serde(rename = "disk")]
    Disk = 9,
    /// file
    #[serde(rename = "file")]
    File = 10,
    /// diskinit
    #[serde(rename = "diskinit")]
    Diskinit = 11,
    /// dispatcher
    #[serde(rename = "dispatcher")]
    Dispatcher = 12,
    /// pf
    #[serde(rename = "pf")]
    Pf = 13,
    /// hf
    #[serde(rename = "hf")]
    Hf = 14,
    /// virtalloc
    #[serde(rename = "virtalloc")]
    Virtalloc = 15,
    /// net
    #[serde(rename = "net")]
    Net = 16,
    /// registry
    #[serde(rename = "registry")]
    Registry = 17,
    /// alpc
    #[serde(rename = "alpc")]
    Alpc = 18,
    /// splitio
    #[serde(rename = "splitio")]
    Splitio = 19,
    /// driver
    #[serde(rename = "driver")]
    Driver = 20,
    /// profile
    #[serde(rename = "profile")]
    Profile = 21,
    /// fileiocompletion
    #[serde(rename = "fileiocompletion")]
    Fileiocompletion = 22,
    /// fileio
    #[serde(rename = "fileio")]
    Fileio = 23,
}

impl Default for SystemTrace_Flags {
    fn default() -> Self {
        Self::Process
    }
}

