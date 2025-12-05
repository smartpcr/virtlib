// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetPrefixPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetPrefixPolicy {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Label")]
    pub label: Option<u32>,

/// 
    #[serde(rename = "Precedence")]
    pub precedence: Option<u32>,

/// 
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u8>,
}

impl MSFT_NetPrefixPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            label: None,
            precedence: None,
            prefix: None,
            store: None,
        }
    }


    /// Sets the value of Label
    pub fn set_label(&mut self, value: u32) {
        self.label = Some(value);
    }

    /// Gets the value of Label
    pub fn get_label(&self) -> Option<&u32> {
        self.label.as_ref()
    }

    /// Sets the value of Precedence
    pub fn set_precedence(&mut self, value: u32) {
        self.precedence = Some(value);
    }

    /// Gets the value of Precedence
    pub fn get_precedence(&self) -> Option<&u32> {
        self.precedence.as_ref()
    }

    /// Sets the value of Prefix
    pub fn set_prefix(&mut self, value: String) {
        self.prefix = Some(value);
    }

    /// Gets the value of Prefix
    pub fn get_prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u8) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u8> {
        self.store.as_ref()
    }

/// 

    /// * `label` -  (u32)
    /// * `pass_thru` -  (bool)
    /// * `policy_store` -  (String)
    /// * `precedence` -  (u32)
    /// * `prefix` -  (String)

    /// * `cmdlet_output` -  (MSFT_NetPrefixPolicy[])
    /// * `return_value` -  (u32)
    pub fn create(&self, prefix: &String, precedence: u32, label: u32, policy_store: &String, pass_thru: bool, cmdlet_output: &mut Vec<MSFT_NetPrefixPolicy>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Prefix".to_string(), value: prefix.into() });
        args.push(MethodParameter { name: "Precedence".to_string(), value: precedence.into() });
        args.push(MethodParameter { name: "Label".to_string(), value: label.into() });
        args.push(MethodParameter { name: "PolicyStore".to_string(), value: policy_store.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Create", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

