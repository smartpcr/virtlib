// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PushPrinterConnectionsPolicySetting_ConnectionType
//////////////////////////////////////////////

/// PushPrinterConnectionsPolicySetting_ConnectionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PushPrinterConnectionsPolicySetting_ConnectionType {
    /// User_connection
    #[serde(rename = "User_connection")]
    UserConnection = 1,
    /// Machine_connection
    #[serde(rename = "Machine_connection")]
    MachineConnection = 2,
}

impl Default for PushPrinterConnectionsPolicySetting_ConnectionType {
    fn default() -> Self {
        Self::UserConnection
    }
}

