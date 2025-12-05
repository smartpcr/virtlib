// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiOffload struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiOffload {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Checksum")]
    pub checksum: Option<MSNdis_WmiTcpIpChecksumOffload>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "IPsecV1")]
    pub ipsec_v1: Option<MSNdis_WmiIPSecOffloadV1>,

/// 
    #[serde(rename = "LsoV1")]
    pub lso_v1: Option<MSNdis_WmiTcpLargeSendOffloadV1>,

/// 
    #[serde(rename = "LsoV2")]
    pub lso_v2: Option<MSNdis_WmiTcpLargeSendOffloadV2>,
}

impl MSNdis_WmiOffload {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            checksum: None,
            flags: None,
            header: None,
            ipsec_v1: None,
            lso_v1: None,
            lso_v2: None,
        }
    }


    /// Sets the value of Checksum
    pub fn set_checksum(&mut self, value: MSNdis_WmiTcpIpChecksumOffload) {
        self.checksum = Some(value);
    }

    /// Gets the value of Checksum
    pub fn get_checksum(&self) -> Option<&MSNdis_WmiTcpIpChecksumOffload> {
        self.checksum.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of IPsecV1
    pub fn set_ipsec_v1(&mut self, value: MSNdis_WmiIPSecOffloadV1) {
        self.ipsec_v1 = Some(value);
    }

    /// Gets the value of IPsecV1
    pub fn get_ipsec_v1(&self) -> Option<&MSNdis_WmiIPSecOffloadV1> {
        self.ipsec_v1.as_ref()
    }

    /// Sets the value of LsoV1
    pub fn set_lso_v1(&mut self, value: MSNdis_WmiTcpLargeSendOffloadV1) {
        self.lso_v1 = Some(value);
    }

    /// Gets the value of LsoV1
    pub fn get_lso_v1(&self) -> Option<&MSNdis_WmiTcpLargeSendOffloadV1> {
        self.lso_v1.as_ref()
    }

    /// Sets the value of LsoV2
    pub fn set_lso_v2(&mut self, value: MSNdis_WmiTcpLargeSendOffloadV2) {
        self.lso_v2 = Some(value);
    }

    /// Gets the value of LsoV2
    pub fn get_lso_v2(&self) -> Option<&MSNdis_WmiTcpLargeSendOffloadV2> {
        self.lso_v2.as_ref()
    }
}

