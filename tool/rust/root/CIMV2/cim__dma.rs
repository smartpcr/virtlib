// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DMA struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DMA {
    #[serde(flatten)]
    pub base: CIM_SystemResource,

/// 
    #[serde(rename = "AddressSize")]
    pub address_size: Option<u16>,

/// 
    #[serde(rename = "Availability")]
    pub availability: Option<u16>,

/// 
    #[serde(rename = "BurstMode")]
    pub burst_mode: Option<bool>,

/// 
    #[serde(rename = "ByteMode")]
    pub byte_mode: Option<u16>,

/// 
    #[serde(rename = "ChannelTiming")]
    pub channel_timing: Option<u16>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "DMAChannel")]
    pub dmachannel: Option<u32>,

/// 
    #[serde(rename = "MaxTransferSize")]
    pub max_transfer_size: Option<u32>,

/// 
    #[serde(rename = "TransferWidths")]
    pub transfer_widths: Vec<u16>,

/// 
    #[serde(rename = "TypeCTiming")]
    pub type_ctiming: Option<u16>,

/// 
    #[serde(rename = "WordMode")]
    pub word_mode: Option<u16>,
}

impl CIM_DMA {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemResource::new(),
            address_size: None,
            availability: None,
            burst_mode: None,
            byte_mode: None,
            channel_timing: None,
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            dmachannel: None,
            max_transfer_size: None,
            transfer_widths: Vec::new(),
            type_ctiming: None,
            word_mode: None,
        }
    }


    /// Sets the value of AddressSize
    pub fn set_address_size(&mut self, value: u16) {
        self.address_size = Some(value);
    }

    /// Gets the value of AddressSize
    pub fn get_address_size(&self) -> Option<&u16> {
        self.address_size.as_ref()
    }

    /// Sets the value of Availability
    pub fn set_availability(&mut self, value: u16) {
        self.availability = Some(value);
    }

    /// Gets the value of Availability
    pub fn get_availability(&self) -> Option<&u16> {
        self.availability.as_ref()
    }

    /// Sets the value of BurstMode
    pub fn set_burst_mode(&mut self, value: bool) {
        self.burst_mode = Some(value);
    }

    /// Gets the value of BurstMode
    pub fn get_burst_mode(&self) -> Option<&bool> {
        self.burst_mode.as_ref()
    }

    /// Sets the value of ByteMode
    pub fn set_byte_mode(&mut self, value: u16) {
        self.byte_mode = Some(value);
    }

    /// Gets the value of ByteMode
    pub fn get_byte_mode(&self) -> Option<&u16> {
        self.byte_mode.as_ref()
    }

    /// Sets the value of ChannelTiming
    pub fn set_channel_timing(&mut self, value: u16) {
        self.channel_timing = Some(value);
    }

    /// Gets the value of ChannelTiming
    pub fn get_channel_timing(&self) -> Option<&u16> {
        self.channel_timing.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of DMAChannel
    pub fn set_dmachannel(&mut self, value: u32) {
        self.dmachannel = Some(value);
    }

    /// Gets the value of DMAChannel
    pub fn get_dmachannel(&self) -> Option<&u32> {
        self.dmachannel.as_ref()
    }

    /// Sets the value of MaxTransferSize
    pub fn set_max_transfer_size(&mut self, value: u32) {
        self.max_transfer_size = Some(value);
    }

    /// Gets the value of MaxTransferSize
    pub fn get_max_transfer_size(&self) -> Option<&u32> {
        self.max_transfer_size.as_ref()
    }

    /// Sets the value of TransferWidths
    pub fn set_transfer_widths(&mut self, value: Vec<u16>) {
        self.transfer_widths = value;
    }

    /// Gets the value of TransferWidths
    pub fn get_transfer_widths(&self) -> &Vec<u16> {
        &self.transfer_widths
    }

    /// Sets the value of TypeCTiming
    pub fn set_type_ctiming(&mut self, value: u16) {
        self.type_ctiming = Some(value);
    }

    /// Gets the value of TypeCTiming
    pub fn get_type_ctiming(&self) -> Option<&u16> {
        self.type_ctiming.as_ref()
    }

    /// Sets the value of WordMode
    pub fn set_word_mode(&mut self, value: u16) {
        self.word_mode = Some(value);
    }

    /// Gets the value of WordMode
    pub fn get_word_mode(&self) -> Option<&u16> {
        self.word_mode.as_ref()
    }
}

