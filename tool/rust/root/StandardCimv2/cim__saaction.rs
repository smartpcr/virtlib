// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SAAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SAAction {
    #[serde(flatten)]
    pub base: CIM_PolicyAction,

/// 
    #[serde(rename = "DoPacketLogging")]
    pub do_packet_logging: Option<bool>,
}

impl CIM_SAAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyAction::new(),
            do_packet_logging: None,
        }
    }


    /// Sets the value of DoPacketLogging
    pub fn set_do_packet_logging(&mut self, value: bool) {
        self.do_packet_logging = Some(value);
    }

    /// Gets the value of DoPacketLogging
    pub fn get_do_packet_logging(&self) -> Option<&bool> {
        self.do_packet_logging.as_ref()
    }
}

