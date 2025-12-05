// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SoundDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SoundDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "DMABufferSize")]
    pub dmabuffer_size: Option<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MPU401Address")]
    pub mpu401_address: Option<u32>,

/// 
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
}

impl Win32_SoundDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            dmabuffer_size: None,
            manufacturer: None,
            mpu401_address: None,
            product_name: None,
        }
    }


    /// Sets the value of DMABufferSize
    pub fn set_dmabuffer_size(&mut self, value: u16) {
        self.dmabuffer_size = Some(value);
    }

    /// Gets the value of DMABufferSize
    pub fn get_dmabuffer_size(&self) -> Option<&u16> {
        self.dmabuffer_size.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MPU401Address
    pub fn set_mpu401_address(&mut self, value: u32) {
        self.mpu401_address = Some(value);
    }

    /// Gets the value of MPU401Address
    pub fn get_mpu401_address(&self) -> Option<&u32> {
        self.mpu401_address.as_ref()
    }

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
    }
}

