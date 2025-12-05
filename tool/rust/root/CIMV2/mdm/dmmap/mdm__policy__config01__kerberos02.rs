// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Kerberos02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Kerberos02 {

/// 
    #[serde(rename = "AllowForestSearchOrder")]
    pub allow_forest_search_order: Option<String>,

/// 
    #[serde(rename = "CloudKerberosTicketRetrievalEnabled")]
    pub cloud_kerberos_ticket_retrieval_enabled: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "KerberosClientSupportsClaimsCompoundArmor")]
    pub kerberos_client_supports_claims_compound_armor: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequireKerberosArmoring")]
    pub require_kerberos_armoring: Option<String>,

/// 
    #[serde(rename = "RequireStrictKDCValidation")]
    pub require_strict_kdcvalidation: Option<String>,

/// 
    #[serde(rename = "SetMaximumContextTokenSize")]
    pub set_maximum_context_token_size: Option<String>,

/// 
    #[serde(rename = "UPNNameHints")]
    pub upnname_hints: Option<String>,
}

impl MDM_Policy_Config01_Kerberos02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_forest_search_order: None,
            cloud_kerberos_ticket_retrieval_enabled: None,
            instance_id: None,
            kerberos_client_supports_claims_compound_armor: None,
            parent_id: None,
            require_kerberos_armoring: None,
            require_strict_kdcvalidation: None,
            set_maximum_context_token_size: None,
            upnname_hints: None,
        }
    }


    /// Sets the value of AllowForestSearchOrder
    pub fn set_allow_forest_search_order(&mut self, value: String) {
        self.allow_forest_search_order = Some(value);
    }

    /// Gets the value of AllowForestSearchOrder
    pub fn get_allow_forest_search_order(&self) -> Option<&String> {
        self.allow_forest_search_order.as_ref()
    }

    /// Sets the value of CloudKerberosTicketRetrievalEnabled
    pub fn set_cloud_kerberos_ticket_retrieval_enabled(&mut self, value: i32) {
        self.cloud_kerberos_ticket_retrieval_enabled = Some(value);
    }

    /// Gets the value of CloudKerberosTicketRetrievalEnabled
    pub fn get_cloud_kerberos_ticket_retrieval_enabled(&self) -> Option<&i32> {
        self.cloud_kerberos_ticket_retrieval_enabled.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of KerberosClientSupportsClaimsCompoundArmor
    pub fn set_kerberos_client_supports_claims_compound_armor(&mut self, value: String) {
        self.kerberos_client_supports_claims_compound_armor = Some(value);
    }

    /// Gets the value of KerberosClientSupportsClaimsCompoundArmor
    pub fn get_kerberos_client_supports_claims_compound_armor(&self) -> Option<&String> {
        self.kerberos_client_supports_claims_compound_armor.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequireKerberosArmoring
    pub fn set_require_kerberos_armoring(&mut self, value: String) {
        self.require_kerberos_armoring = Some(value);
    }

    /// Gets the value of RequireKerberosArmoring
    pub fn get_require_kerberos_armoring(&self) -> Option<&String> {
        self.require_kerberos_armoring.as_ref()
    }

    /// Sets the value of RequireStrictKDCValidation
    pub fn set_require_strict_kdcvalidation(&mut self, value: String) {
        self.require_strict_kdcvalidation = Some(value);
    }

    /// Gets the value of RequireStrictKDCValidation
    pub fn get_require_strict_kdcvalidation(&self) -> Option<&String> {
        self.require_strict_kdcvalidation.as_ref()
    }

    /// Sets the value of SetMaximumContextTokenSize
    pub fn set_set_maximum_context_token_size(&mut self, value: String) {
        self.set_maximum_context_token_size = Some(value);
    }

    /// Gets the value of SetMaximumContextTokenSize
    pub fn get_set_maximum_context_token_size(&self) -> Option<&String> {
        self.set_maximum_context_token_size.as_ref()
    }

    /// Sets the value of UPNNameHints
    pub fn set_upnname_hints(&mut self, value: String) {
        self.upnname_hints = Some(value);
    }

    /// Gets the value of UPNNameHints
    pub fn get_upnname_hints(&self) -> Option<&String> {
        self.upnname_hints.as_ref()
    }
}

