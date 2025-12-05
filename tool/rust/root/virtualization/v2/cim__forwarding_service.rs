// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ForwardingService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ForwardingService {
    #[serde(flatten)]
    pub base: CIM_NetworkService,

/// This defines the type of protocol that is being forwarded when the value of the ProtocolType attribute is 1 (i.e., "Other"). This provides for future extensibility.
    #[serde(rename = "OtherProtocolType")]
    pub other_protocol_type: Option<String>,

/// This defines the type of protocol that is being forwarded.
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<ForwardingService_ProtocolType>,
}

impl CIM_ForwardingService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkService::new(),
            other_protocol_type: None,
            protocol_type: None,
        }
    }


    /// Sets the value of OtherProtocolType
    pub fn set_other_protocol_type(&mut self, value: String) {
        self.other_protocol_type = Some(value);
    }

    /// Gets the value of OtherProtocolType
    pub fn get_other_protocol_type(&self) -> Option<&String> {
        self.other_protocol_type.as_ref()
    }

    /// Sets the value of ProtocolType
    pub fn set_protocol_type(&mut self, value: ForwardingService_ProtocolType) {
        self.protocol_type = Some(value);
    }

    /// Gets the value of ProtocolType
    pub fn get_protocol_type(&self) -> Option<&ForwardingService_ProtocolType> {
        self.protocol_type.as_ref()
    }
}

