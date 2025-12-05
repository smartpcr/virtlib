// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SecurityAssociationEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SecurityAssociationEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 
    #[serde(rename = "IdleDurationSeconds")]
    pub idle_duration_seconds: Option<u64>,

/// 
    #[serde(rename = "LifetimeKilobytes")]
    pub lifetime_kilobytes: Option<u64>,

/// 
    #[serde(rename = "LifetimeSeconds")]
    pub lifetime_seconds: Option<u64>,

/// 
    #[serde(rename = "PacketLoggingActive")]
    pub packet_logging_active: Option<bool>,

/// 
    #[serde(rename = "RefreshThresholdKbytesPercentage")]
    pub refresh_threshold_kbytes_percentage: Option<u8>,

/// 
    #[serde(rename = "RefreshThresholdSecondsPercentage")]
    pub refresh_threshold_seconds_percentage: Option<u8>,
}

impl CIM_SecurityAssociationEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            idle_duration_seconds: None,
            lifetime_kilobytes: None,
            lifetime_seconds: None,
            packet_logging_active: None,
            refresh_threshold_kbytes_percentage: None,
            refresh_threshold_seconds_percentage: None,
        }
    }


    /// Sets the value of IdleDurationSeconds
    pub fn set_idle_duration_seconds(&mut self, value: u64) {
        self.idle_duration_seconds = Some(value);
    }

    /// Gets the value of IdleDurationSeconds
    pub fn get_idle_duration_seconds(&self) -> Option<&u64> {
        self.idle_duration_seconds.as_ref()
    }

    /// Sets the value of LifetimeKilobytes
    pub fn set_lifetime_kilobytes(&mut self, value: u64) {
        self.lifetime_kilobytes = Some(value);
    }

    /// Gets the value of LifetimeKilobytes
    pub fn get_lifetime_kilobytes(&self) -> Option<&u64> {
        self.lifetime_kilobytes.as_ref()
    }

    /// Sets the value of LifetimeSeconds
    pub fn set_lifetime_seconds(&mut self, value: u64) {
        self.lifetime_seconds = Some(value);
    }

    /// Gets the value of LifetimeSeconds
    pub fn get_lifetime_seconds(&self) -> Option<&u64> {
        self.lifetime_seconds.as_ref()
    }

    /// Sets the value of PacketLoggingActive
    pub fn set_packet_logging_active(&mut self, value: bool) {
        self.packet_logging_active = Some(value);
    }

    /// Gets the value of PacketLoggingActive
    pub fn get_packet_logging_active(&self) -> Option<&bool> {
        self.packet_logging_active.as_ref()
    }

    /// Sets the value of RefreshThresholdKbytesPercentage
    pub fn set_refresh_threshold_kbytes_percentage(&mut self, value: u8) {
        self.refresh_threshold_kbytes_percentage = Some(value);
    }

    /// Gets the value of RefreshThresholdKbytesPercentage
    pub fn get_refresh_threshold_kbytes_percentage(&self) -> Option<&u8> {
        self.refresh_threshold_kbytes_percentage.as_ref()
    }

    /// Sets the value of RefreshThresholdSecondsPercentage
    pub fn set_refresh_threshold_seconds_percentage(&mut self, value: u8) {
        self.refresh_threshold_seconds_percentage = Some(value);
    }

    /// Gets the value of RefreshThresholdSecondsPercentage
    pub fn get_refresh_threshold_seconds_percentage(&self) -> Option<&u8> {
        self.refresh_threshold_seconds_percentage.as_ref()
    }
}

