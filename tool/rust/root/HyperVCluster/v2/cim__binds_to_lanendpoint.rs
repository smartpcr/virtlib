// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BindsToLANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BindsToLANEndpoint {
    #[serde(flatten)]
    pub base: CIM_BindsTo,

/// This describes the framing method for the upper layer SAP or Endpoint that is bound to the LANEndpoint. Note: "Raw802.3" is only known to be used with the IPX protocol.
    #[serde(rename = "FrameType")]
    pub frame_type: Option<BindsToLANEndpoint_FrameType>,
}

impl CIM_BindsToLANEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BindsTo::new(),
            frame_type: None,
        }
    }


    /// Sets the value of FrameType
    pub fn set_frame_type(&mut self, value: BindsToLANEndpoint_FrameType) {
        self.frame_type = Some(value);
    }

    /// Gets the value of FrameType
    pub fn get_frame_type(&self) -> Option<&BindsToLANEndpoint_FrameType> {
        self.frame_type.as_ref()
    }
}

