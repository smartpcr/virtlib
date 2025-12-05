// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_RemotePerfProvider_HyperVVMRemoting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_RemotePerfProvider_HyperVVMRemoting {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ConnectedClients")]
    pub connected_clients: Option<u32>,

/// 
    #[serde(rename = "UpdatedPixelsPersec")]
    pub updated_pixels_persec: Option<u32>,
}

impl Win32_PerfFormattedData_RemotePerfProvider_HyperVVMRemoting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            connected_clients: None,
            updated_pixels_persec: None,
        }
    }


    /// Sets the value of ConnectedClients
    pub fn set_connected_clients(&mut self, value: u32) {
        self.connected_clients = Some(value);
    }

    /// Gets the value of ConnectedClients
    pub fn get_connected_clients(&self) -> Option<&u32> {
        self.connected_clients.as_ref()
    }

    /// Sets the value of UpdatedPixelsPersec
    pub fn set_updated_pixels_persec(&mut self, value: u32) {
        self.updated_pixels_persec = Some(value);
    }

    /// Gets the value of UpdatedPixelsPersec
    pub fn get_updated_pixels_persec(&self) -> Option<&u32> {
        self.updated_pixels_persec.as_ref()
    }
}

