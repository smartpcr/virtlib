// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TapeDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TapeDrive {
    #[serde(flatten)]
    pub base: CIM_TapeDrive,

/// 
    #[serde(rename = "Compression")]
    pub compression: Option<u32>,

/// 
    #[serde(rename = "ECC")]
    pub ecc: Option<u32>,

/// 
    #[serde(rename = "FeaturesHigh")]
    pub features_high: Option<u32>,

/// 
    #[serde(rename = "FeaturesLow")]
    pub features_low: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,

/// 
    #[serde(rename = "ReportSetMarks")]
    pub report_set_marks: Option<u32>,
}

impl Win32_TapeDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_TapeDrive::new(),
            compression: None,
            ecc: None,
            features_high: None,
            features_low: None,
            id: None,
            manufacturer: None,
            media_type: None,
            report_set_marks: None,
        }
    }


    /// Sets the value of Compression
    pub fn set_compression(&mut self, value: u32) {
        self.compression = Some(value);
    }

    /// Gets the value of Compression
    pub fn get_compression(&self) -> Option<&u32> {
        self.compression.as_ref()
    }

    /// Sets the value of ECC
    pub fn set_ecc(&mut self, value: u32) {
        self.ecc = Some(value);
    }

    /// Gets the value of ECC
    pub fn get_ecc(&self) -> Option<&u32> {
        self.ecc.as_ref()
    }

    /// Sets the value of FeaturesHigh
    pub fn set_features_high(&mut self, value: u32) {
        self.features_high = Some(value);
    }

    /// Gets the value of FeaturesHigh
    pub fn get_features_high(&self) -> Option<&u32> {
        self.features_high.as_ref()
    }

    /// Sets the value of FeaturesLow
    pub fn set_features_low(&mut self, value: u32) {
        self.features_low = Some(value);
    }

    /// Gets the value of FeaturesLow
    pub fn get_features_low(&self) -> Option<&u32> {
        self.features_low.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: String) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&String> {
        self.media_type.as_ref()
    }

    /// Sets the value of ReportSetMarks
    pub fn set_report_set_marks(&mut self, value: u32) {
        self.report_set_marks = Some(value);
    }

    /// Gets the value of ReportSetMarks
    pub fn get_report_set_marks(&self) -> Option<&u32> {
        self.report_set_marks.as_ref()
    }
}

