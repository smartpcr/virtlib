// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AcpiGenAddr struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcpiGenAddr {

/// 
    #[serde(rename = "Address")]
    pub address: Option<u64>,

/// 
    #[serde(rename = "AddressSpaceID")]
    pub address_space_id: Option<u32>,

/// 
    #[serde(rename = "BitOffset")]
    pub bit_offset: Option<u32>,

/// 
    #[serde(rename = "BitWidth")]
    pub bit_width: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,
}

impl AcpiGenAddr {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            address_space_id: None,
            bit_offset: None,
            bit_width: None,
            reserved: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: u64) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&u64> {
        self.address.as_ref()
    }

    /// Sets the value of AddressSpaceID
    pub fn set_address_space_id(&mut self, value: u32) {
        self.address_space_id = Some(value);
    }

    /// Gets the value of AddressSpaceID
    pub fn get_address_space_id(&self) -> Option<&u32> {
        self.address_space_id.as_ref()
    }

    /// Sets the value of BitOffset
    pub fn set_bit_offset(&mut self, value: u32) {
        self.bit_offset = Some(value);
    }

    /// Gets the value of BitOffset
    pub fn get_bit_offset(&self) -> Option<&u32> {
        self.bit_offset.as_ref()
    }

    /// Sets the value of BitWidth
    pub fn set_bit_width(&mut self, value: u32) {
        self.bit_width = Some(value);
    }

    /// Gets the value of BitWidth
    pub fn get_bit_width(&self) -> Option<&u32> {
        self.bit_width.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }
}

