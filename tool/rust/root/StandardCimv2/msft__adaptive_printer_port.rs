// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_AdaptivePrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_AdaptivePrinterPort {
    #[serde(flatten)]
    pub base: MSFT_PrinterPort,

/// 
    #[serde(rename = "DeviceURL")]
    pub device_url: Option<String>,

/// 
    #[serde(rename = "DeviceUUID")]
    pub device_uuid: Option<String>,

/// 
    #[serde(rename = "DiscoveryMethod")]
    pub discovery_method: Option<u32>,

/// 
    #[serde(rename = "IppUrl")]
    pub ipp_url: Option<String>,
}

impl MSFT_AdaptivePrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PrinterPort::new(),
            device_url: None,
            device_uuid: None,
            discovery_method: None,
            ipp_url: None,
        }
    }


    /// Sets the value of DeviceURL
    pub fn set_device_url(&mut self, value: String) {
        self.device_url = Some(value);
    }

    /// Gets the value of DeviceURL
    pub fn get_device_url(&self) -> Option<&String> {
        self.device_url.as_ref()
    }

    /// Sets the value of DeviceUUID
    pub fn set_device_uuid(&mut self, value: String) {
        self.device_uuid = Some(value);
    }

    /// Gets the value of DeviceUUID
    pub fn get_device_uuid(&self) -> Option<&String> {
        self.device_uuid.as_ref()
    }

    /// Sets the value of DiscoveryMethod
    pub fn set_discovery_method(&mut self, value: u32) {
        self.discovery_method = Some(value);
    }

    /// Gets the value of DiscoveryMethod
    pub fn get_discovery_method(&self) -> Option<&u32> {
        self.discovery_method.as_ref()
    }

    /// Sets the value of IppUrl
    pub fn set_ipp_url(&mut self, value: String) {
        self.ipp_url = Some(value);
    }

    /// Gets the value of IppUrl
    pub fn get_ipp_url(&self) -> Option<&String> {
        self.ipp_url.as_ref()
    }
}

