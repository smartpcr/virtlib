// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TcpIp_V1_Fail struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpIp_V1_Fail {
    #[serde(flatten)]
    pub base: TcpIp_V1,

/// 
    #[serde(rename = "Proto")]
    pub proto: Option<u32>,
}

impl TcpIp_V1_Fail {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TcpIp_V1::new(),
            proto: None,
        }
    }


    /// Sets the value of Proto
    pub fn set_proto(&mut self, value: u32) {
        self.proto = Some(value);
    }

    /// Gets the value of Proto
    pub fn get_proto(&self) -> Option<&u32> {
        self.proto.as_ref()
    }
}

