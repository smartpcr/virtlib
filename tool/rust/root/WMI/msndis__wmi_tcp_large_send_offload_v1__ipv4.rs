// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpLargeSendOffloadV1_IPv4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpLargeSendOffloadV1_IPv4 {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u32>,

/// 
    #[serde(rename = "IpOptions")]
    pub ip_options: Option<u32>,

/// 
    #[serde(rename = "MaxOffLoadSize")]
    pub max_off_load_size: Option<u32>,

/// 
    #[serde(rename = "MinSegmentCount")]
    pub min_segment_count: Option<u32>,

/// 
    #[serde(rename = "TcpOptions")]
    pub tcp_options: Option<u32>,
}

impl MSNdis_WmiTcpLargeSendOffloadV1_IPv4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            encapsulation: None,
            ip_options: None,
            max_off_load_size: None,
            min_segment_count: None,
            tcp_options: None,
        }
    }


    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u32) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u32> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of IpOptions
    pub fn set_ip_options(&mut self, value: u32) {
        self.ip_options = Some(value);
    }

    /// Gets the value of IpOptions
    pub fn get_ip_options(&self) -> Option<&u32> {
        self.ip_options.as_ref()
    }

    /// Sets the value of MaxOffLoadSize
    pub fn set_max_off_load_size(&mut self, value: u32) {
        self.max_off_load_size = Some(value);
    }

    /// Gets the value of MaxOffLoadSize
    pub fn get_max_off_load_size(&self) -> Option<&u32> {
        self.max_off_load_size.as_ref()
    }

    /// Sets the value of MinSegmentCount
    pub fn set_min_segment_count(&mut self, value: u32) {
        self.min_segment_count = Some(value);
    }

    /// Gets the value of MinSegmentCount
    pub fn get_min_segment_count(&self) -> Option<&u32> {
        self.min_segment_count.as_ref()
    }

    /// Sets the value of TcpOptions
    pub fn set_tcp_options(&mut self, value: u32) {
        self.tcp_options = Some(value);
    }

    /// Gets the value of TcpOptions
    pub fn get_tcp_options(&self) -> Option<&u32> {
        self.tcp_options.as_ref()
    }
}

