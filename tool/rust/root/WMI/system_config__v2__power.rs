// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Power struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Power {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "Pad1")]
    pub pad1: Option<u8>,

/// 
    #[serde(rename = "Pad2")]
    pub pad2: Option<u8>,

/// 
    #[serde(rename = "Pad3")]
    pub pad3: Option<u8>,

/// 
    #[serde(rename = "S1")]
    pub s1: Option<u8>,

/// 
    #[serde(rename = "S2")]
    pub s2: Option<u8>,

/// 
    #[serde(rename = "S3")]
    pub s3: Option<u8>,

/// 
    #[serde(rename = "S4")]
    pub s4: Option<u8>,

/// 
    #[serde(rename = "S5")]
    pub s5: Option<u8>,
}

impl SystemConfig_V2_Power {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            pad1: None,
            pad2: None,
            pad3: None,
            s1: None,
            s2: None,
            s3: None,
            s4: None,
            s5: None,
        }
    }


    /// Sets the value of Pad1
    pub fn set_pad1(&mut self, value: u8) {
        self.pad1 = Some(value);
    }

    /// Gets the value of Pad1
    pub fn get_pad1(&self) -> Option<&u8> {
        self.pad1.as_ref()
    }

    /// Sets the value of Pad2
    pub fn set_pad2(&mut self, value: u8) {
        self.pad2 = Some(value);
    }

    /// Gets the value of Pad2
    pub fn get_pad2(&self) -> Option<&u8> {
        self.pad2.as_ref()
    }

    /// Sets the value of Pad3
    pub fn set_pad3(&mut self, value: u8) {
        self.pad3 = Some(value);
    }

    /// Gets the value of Pad3
    pub fn get_pad3(&self) -> Option<&u8> {
        self.pad3.as_ref()
    }

    /// Sets the value of S1
    pub fn set_s1(&mut self, value: u8) {
        self.s1 = Some(value);
    }

    /// Gets the value of S1
    pub fn get_s1(&self) -> Option<&u8> {
        self.s1.as_ref()
    }

    /// Sets the value of S2
    pub fn set_s2(&mut self, value: u8) {
        self.s2 = Some(value);
    }

    /// Gets the value of S2
    pub fn get_s2(&self) -> Option<&u8> {
        self.s2.as_ref()
    }

    /// Sets the value of S3
    pub fn set_s3(&mut self, value: u8) {
        self.s3 = Some(value);
    }

    /// Gets the value of S3
    pub fn get_s3(&self) -> Option<&u8> {
        self.s3.as_ref()
    }

    /// Sets the value of S4
    pub fn set_s4(&mut self, value: u8) {
        self.s4 = Some(value);
    }

    /// Gets the value of S4
    pub fn get_s4(&self) -> Option<&u8> {
        self.s4.as_ref()
    }

    /// Sets the value of S5
    pub fn set_s5(&mut self, value: u8) {
        self.s5 = Some(value);
    }

    /// Gets the value of S5
    pub fn get_s5(&self) -> Option<&u8> {
        self.s5.as_ref()
    }
}

