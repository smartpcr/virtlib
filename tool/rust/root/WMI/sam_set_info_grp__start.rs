// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SamSetInfoGrp_Start struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SamSetInfoGrp_Start {
    #[serde(flatten)]
    pub base: SamSetInfoGrp,

/// Client Network Address
    #[serde(rename = "Client")]
    pub client: Option<String>,

/// Signature
    #[serde(rename = "Sam")]
    pub sam: Option<String>,

/// Client SID
    #[serde(rename = "Sid")]
    pub sid: Option<String>,

/// SamTraceVersion
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

impl SamSetInfoGrp_Start {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SamSetInfoGrp::new(),
            client: None,
            sam: None,
            sid: None,
            version: None,
        }
    }


    /// Sets the value of Client
    pub fn set_client(&mut self, value: String) {
        self.client = Some(value);
    }

    /// Gets the value of Client
    pub fn get_client(&self) -> Option<&String> {
        self.client.as_ref()
    }

    /// Sets the value of Sam
    pub fn set_sam(&mut self, value: String) {
        self.sam = Some(value);
    }

    /// Gets the value of Sam
    pub fn get_sam(&self) -> Option<&String> {
        self.sam.as_ref()
    }

    /// Sets the value of Sid
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of Sid
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

