// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IKEAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IKEAction {
    #[serde(flatten)]
    pub base: CIM_SANegotiationAction,

/// 
    #[serde(rename = "AggressiveModeGroupID")]
    pub aggressive_mode_group_id: Option<u16>,

/// 
    #[serde(rename = "ExchangeMode")]
    pub exchange_mode: Option<u16>,

/// 
    #[serde(rename = "UseIKEIdentityType")]
    pub use_ikeidentity_type: Option<u16>,

/// 
    #[serde(rename = "VendorID")]
    pub vendor_id: Option<String>,
}

impl CIM_IKEAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SANegotiationAction::new(),
            aggressive_mode_group_id: None,
            exchange_mode: None,
            use_ikeidentity_type: None,
            vendor_id: None,
        }
    }


    /// Sets the value of AggressiveModeGroupID
    pub fn set_aggressive_mode_group_id(&mut self, value: u16) {
        self.aggressive_mode_group_id = Some(value);
    }

    /// Gets the value of AggressiveModeGroupID
    pub fn get_aggressive_mode_group_id(&self) -> Option<&u16> {
        self.aggressive_mode_group_id.as_ref()
    }

    /// Sets the value of ExchangeMode
    pub fn set_exchange_mode(&mut self, value: u16) {
        self.exchange_mode = Some(value);
    }

    /// Gets the value of ExchangeMode
    pub fn get_exchange_mode(&self) -> Option<&u16> {
        self.exchange_mode.as_ref()
    }

    /// Sets the value of UseIKEIdentityType
    pub fn set_use_ikeidentity_type(&mut self, value: u16) {
        self.use_ikeidentity_type = Some(value);
    }

    /// Gets the value of UseIKEIdentityType
    pub fn get_use_ikeidentity_type(&self) -> Option<&u16> {
        self.use_ikeidentity_type.as_ref()
    }

    /// Sets the value of VendorID
    pub fn set_vendor_id(&mut self, value: String) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorID
    pub fn get_vendor_id(&self) -> Option<&String> {
        self.vendor_id.as_ref()
    }
}

