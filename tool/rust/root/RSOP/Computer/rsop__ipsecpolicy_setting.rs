// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IPSECPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IPSECPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "ClassName")]
    pub class_name: Option<String>,

/// 
    #[serde(rename = "description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ipsecData")]
    pub ipsec_data: Vec<u8>,

/// 
    #[serde(rename = "ipsecDataType")]
    pub ipsec_data_type: Option<u32>,

/// 
    #[serde(rename = "ipsecFilterReference")]
    pub ipsec_filter_reference: Vec<String>,

/// 
    #[serde(rename = "ipsecID")]
    pub ipsec_id: Option<String>,

/// 
    #[serde(rename = "ipsecISAKMPReference")]
    pub ipsec_isakmpreference: Option<String>,

/// 
    #[serde(rename = "ipsecName")]
    pub ipsec_name: Option<String>,

/// 
    #[serde(rename = "ipsecNegotiationPolicyAction")]
    pub ipsec_negotiation_policy_action: Option<String>,

/// 
    #[serde(rename = "ipsecNegotiationPolicyReference")]
    pub ipsec_negotiation_policy_reference: Option<String>,

/// 
    #[serde(rename = "ipsecNegotiationPolicyType")]
    pub ipsec_negotiation_policy_type: Option<String>,

/// 
    #[serde(rename = "ipsecNFAReference")]
    pub ipsec_nfareference: Vec<String>,

/// 
    #[serde(rename = "ipsecOwnersReference")]
    pub ipsec_owners_reference: Vec<String>,

/// 
    #[serde(rename = "whenChanged")]
    pub when_changed: Option<u32>,
}

impl RSOP_IPSECPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            class_name: None,
            description: None,
            ipsec_data: Vec::new(),
            ipsec_data_type: None,
            ipsec_filter_reference: Vec::new(),
            ipsec_id: None,
            ipsec_isakmpreference: None,
            ipsec_name: None,
            ipsec_negotiation_policy_action: None,
            ipsec_negotiation_policy_reference: None,
            ipsec_negotiation_policy_type: None,
            ipsec_nfareference: Vec::new(),
            ipsec_owners_reference: Vec::new(),
            when_changed: None,
        }
    }


    /// Sets the value of ClassName
    pub fn set_class_name(&mut self, value: String) {
        self.class_name = Some(value);
    }

    /// Gets the value of ClassName
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
    }

    /// Sets the value of description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ipsecData
    pub fn set_ipsec_data(&mut self, value: Vec<u8>) {
        self.ipsec_data = value;
    }

    /// Gets the value of ipsecData
    pub fn get_ipsec_data(&self) -> &Vec<u8> {
        &self.ipsec_data
    }

    /// Sets the value of ipsecDataType
    pub fn set_ipsec_data_type(&mut self, value: u32) {
        self.ipsec_data_type = Some(value);
    }

    /// Gets the value of ipsecDataType
    pub fn get_ipsec_data_type(&self) -> Option<&u32> {
        self.ipsec_data_type.as_ref()
    }

    /// Sets the value of ipsecFilterReference
    pub fn set_ipsec_filter_reference(&mut self, value: Vec<String>) {
        self.ipsec_filter_reference = value;
    }

    /// Gets the value of ipsecFilterReference
    pub fn get_ipsec_filter_reference(&self) -> &Vec<String> {
        &self.ipsec_filter_reference
    }

    /// Sets the value of ipsecID
    pub fn set_ipsec_id(&mut self, value: String) {
        self.ipsec_id = Some(value);
    }

    /// Gets the value of ipsecID
    pub fn get_ipsec_id(&self) -> Option<&String> {
        self.ipsec_id.as_ref()
    }

    /// Sets the value of ipsecISAKMPReference
    pub fn set_ipsec_isakmpreference(&mut self, value: String) {
        self.ipsec_isakmpreference = Some(value);
    }

    /// Gets the value of ipsecISAKMPReference
    pub fn get_ipsec_isakmpreference(&self) -> Option<&String> {
        self.ipsec_isakmpreference.as_ref()
    }

    /// Sets the value of ipsecName
    pub fn set_ipsec_name(&mut self, value: String) {
        self.ipsec_name = Some(value);
    }

    /// Gets the value of ipsecName
    pub fn get_ipsec_name(&self) -> Option<&String> {
        self.ipsec_name.as_ref()
    }

    /// Sets the value of ipsecNegotiationPolicyAction
    pub fn set_ipsec_negotiation_policy_action(&mut self, value: String) {
        self.ipsec_negotiation_policy_action = Some(value);
    }

    /// Gets the value of ipsecNegotiationPolicyAction
    pub fn get_ipsec_negotiation_policy_action(&self) -> Option<&String> {
        self.ipsec_negotiation_policy_action.as_ref()
    }

    /// Sets the value of ipsecNegotiationPolicyReference
    pub fn set_ipsec_negotiation_policy_reference(&mut self, value: String) {
        self.ipsec_negotiation_policy_reference = Some(value);
    }

    /// Gets the value of ipsecNegotiationPolicyReference
    pub fn get_ipsec_negotiation_policy_reference(&self) -> Option<&String> {
        self.ipsec_negotiation_policy_reference.as_ref()
    }

    /// Sets the value of ipsecNegotiationPolicyType
    pub fn set_ipsec_negotiation_policy_type(&mut self, value: String) {
        self.ipsec_negotiation_policy_type = Some(value);
    }

    /// Gets the value of ipsecNegotiationPolicyType
    pub fn get_ipsec_negotiation_policy_type(&self) -> Option<&String> {
        self.ipsec_negotiation_policy_type.as_ref()
    }

    /// Sets the value of ipsecNFAReference
    pub fn set_ipsec_nfareference(&mut self, value: Vec<String>) {
        self.ipsec_nfareference = value;
    }

    /// Gets the value of ipsecNFAReference
    pub fn get_ipsec_nfareference(&self) -> &Vec<String> {
        &self.ipsec_nfareference
    }

    /// Sets the value of ipsecOwnersReference
    pub fn set_ipsec_owners_reference(&mut self, value: Vec<String>) {
        self.ipsec_owners_reference = value;
    }

    /// Gets the value of ipsecOwnersReference
    pub fn get_ipsec_owners_reference(&self) -> &Vec<String> {
        &self.ipsec_owners_reference
    }

    /// Sets the value of whenChanged
    pub fn set_when_changed(&mut self, value: u32) {
        self.when_changed = Some(value);
    }

    /// Gets the value of whenChanged
    pub fn get_when_changed(&self) -> Option<&u32> {
        self.when_changed.as_ref()
    }
}

