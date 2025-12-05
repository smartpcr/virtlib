// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallDynamicKeywordAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallDynamicKeywordAddress {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Addresses")]
    pub addresses: Option<String>,

/// 
    #[serde(rename = "AutoResolve")]
    pub auto_resolve: Option<bool>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Keyword")]
    pub keyword: Option<String>,

/// 
    #[serde(rename = "PolicyStoreSource")]
    pub policy_store_source: Option<String>,

/// 
    #[serde(rename = "PolicyStoreSourceType")]
    pub policy_store_source_type: Option<u16>,
}

impl MSFT_NetFirewallDynamicKeywordAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            addresses: None,
            auto_resolve: None,
            id: None,
            keyword: None,
            policy_store_source: None,
            policy_store_source_type: None,
        }
    }


    /// Sets the value of Addresses
    pub fn set_addresses(&mut self, value: String) {
        self.addresses = Some(value);
    }

    /// Gets the value of Addresses
    pub fn get_addresses(&self) -> Option<&String> {
        self.addresses.as_ref()
    }

    /// Sets the value of AutoResolve
    pub fn set_auto_resolve(&mut self, value: bool) {
        self.auto_resolve = Some(value);
    }

    /// Gets the value of AutoResolve
    pub fn get_auto_resolve(&self) -> Option<&bool> {
        self.auto_resolve.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Keyword
    pub fn set_keyword(&mut self, value: String) {
        self.keyword = Some(value);
    }

    /// Gets the value of Keyword
    pub fn get_keyword(&self) -> Option<&String> {
        self.keyword.as_ref()
    }

    /// Sets the value of PolicyStoreSource
    pub fn set_policy_store_source(&mut self, value: String) {
        self.policy_store_source = Some(value);
    }

    /// Gets the value of PolicyStoreSource
    pub fn get_policy_store_source(&self) -> Option<&String> {
        self.policy_store_source.as_ref()
    }

    /// Sets the value of PolicyStoreSourceType
    pub fn set_policy_store_source_type(&mut self, value: u16) {
        self.policy_store_source_type = Some(value);
    }

    /// Gets the value of PolicyStoreSourceType
    pub fn get_policy_store_source_type(&self) -> Option<&u16> {
        self.policy_store_source_type.as_ref()
    }

/// 

    /// * `addresses` -  (String)
    /// * `append` -  (bool)
    /// * `id` -  (String)

    /// * `return_value` -  (u32)
    pub fn update_dynamic_keyword_address(&self, id: &String, addresses: &String, append: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "Addresses".to_string(), value: addresses.into() });
        args.push(MethodParameter { name: "Append".to_string(), value: append.into() });
        self.invoke_method("UpdateDynamicKeywordAddress", &args)

    }

}

