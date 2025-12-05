// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RoamingProfileSlowLinkParams struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RoamingProfileSlowLinkParams {

/// The connection speed, in kilobytes per second (kbps). This threshold is used to determine if the connection is a slow link. If the server's transfer rate in kbps is less than this threshold, the connection is considered to be slow. This property applies to IP networks.
    #[serde(rename = "ConnectionTransferRate")]
    pub connection_transfer_rate: Option<u32>,

/// The slow-network connection timeout, in milliseconds. This threshold is used to determine if the connection is a slow link. If the delay in milliseconds is greater than this threshold, the connection is considered to be slow. This property applies to non-IP networks.
    #[serde(rename = "TimeOut")]
    pub time_out: Option<u16>,
}

impl Win32_RoamingProfileSlowLinkParams {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_transfer_rate: None,
            time_out: None,
        }
    }


    /// Sets the value of ConnectionTransferRate
    pub fn set_connection_transfer_rate(&mut self, value: u32) {
        self.connection_transfer_rate = Some(value);
    }

    /// Gets the value of ConnectionTransferRate
    pub fn get_connection_transfer_rate(&self) -> Option<&u32> {
        self.connection_transfer_rate.as_ref()
    }

    /// Sets the value of TimeOut
    pub fn set_time_out(&mut self, value: u16) {
        self.time_out = Some(value);
    }

    /// Gets the value of TimeOut
    pub fn get_time_out(&self) -> Option<&u16> {
        self.time_out.as_ref()
    }
}

