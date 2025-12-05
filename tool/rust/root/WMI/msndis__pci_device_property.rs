// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PciDeviceProperty struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PciDeviceProperty {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "CurrentLinkSpeed")]
    pub current_link_speed: Option<u32>,

/// 
    #[serde(rename = "CurrentLinkWidth")]
    pub current_link_width: Option<u32>,

/// 
    #[serde(rename = "CurrentPayloadSize")]
    pub current_payload_size: Option<u32>,

/// 
    #[serde(rename = "CurrentSpeedAndMode")]
    pub current_speed_and_mode: Option<u32>,

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MaxLinkSpeed")]
    pub max_link_speed: Option<u32>,

/// 
    #[serde(rename = "MaxLinkWidth")]
    pub max_link_width: Option<u32>,

/// 
    #[serde(rename = "MaxPayloadSize")]
    pub max_payload_size: Option<u32>,

/// 
    #[serde(rename = "MaxReadRequestSize")]
    pub max_read_request_size: Option<u32>,
}

impl MSNdis_PciDeviceProperty {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            current_link_speed: None,
            current_link_width: None,
            current_payload_size: None,
            current_speed_and_mode: None,
            device_type: None,
            header: None,
            max_link_speed: None,
            max_link_width: None,
            max_payload_size: None,
            max_read_request_size: None,
        }
    }


    /// Sets the value of CurrentLinkSpeed
    pub fn set_current_link_speed(&mut self, value: u32) {
        self.current_link_speed = Some(value);
    }

    /// Gets the value of CurrentLinkSpeed
    pub fn get_current_link_speed(&self) -> Option<&u32> {
        self.current_link_speed.as_ref()
    }

    /// Sets the value of CurrentLinkWidth
    pub fn set_current_link_width(&mut self, value: u32) {
        self.current_link_width = Some(value);
    }

    /// Gets the value of CurrentLinkWidth
    pub fn get_current_link_width(&self) -> Option<&u32> {
        self.current_link_width.as_ref()
    }

    /// Sets the value of CurrentPayloadSize
    pub fn set_current_payload_size(&mut self, value: u32) {
        self.current_payload_size = Some(value);
    }

    /// Gets the value of CurrentPayloadSize
    pub fn get_current_payload_size(&self) -> Option<&u32> {
        self.current_payload_size.as_ref()
    }

    /// Sets the value of CurrentSpeedAndMode
    pub fn set_current_speed_and_mode(&mut self, value: u32) {
        self.current_speed_and_mode = Some(value);
    }

    /// Gets the value of CurrentSpeedAndMode
    pub fn get_current_speed_and_mode(&self) -> Option<&u32> {
        self.current_speed_and_mode.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MaxLinkSpeed
    pub fn set_max_link_speed(&mut self, value: u32) {
        self.max_link_speed = Some(value);
    }

    /// Gets the value of MaxLinkSpeed
    pub fn get_max_link_speed(&self) -> Option<&u32> {
        self.max_link_speed.as_ref()
    }

    /// Sets the value of MaxLinkWidth
    pub fn set_max_link_width(&mut self, value: u32) {
        self.max_link_width = Some(value);
    }

    /// Gets the value of MaxLinkWidth
    pub fn get_max_link_width(&self) -> Option<&u32> {
        self.max_link_width.as_ref()
    }

    /// Sets the value of MaxPayloadSize
    pub fn set_max_payload_size(&mut self, value: u32) {
        self.max_payload_size = Some(value);
    }

    /// Gets the value of MaxPayloadSize
    pub fn get_max_payload_size(&self) -> Option<&u32> {
        self.max_payload_size.as_ref()
    }

    /// Sets the value of MaxReadRequestSize
    pub fn set_max_read_request_size(&mut self, value: u32) {
        self.max_read_request_size = Some(value);
    }

    /// Gets the value of MaxReadRequestSize
    pub fn get_max_read_request_size(&self) -> Option<&u32> {
        self.max_read_request_size.as_ref()
    }
}

