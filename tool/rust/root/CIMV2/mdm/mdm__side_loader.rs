// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_SideLoader struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_SideLoader {

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "ProductKeyHash")]
    pub product_key_hash: Option<String>,
}

impl MDM_SideLoader {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            key: None,
            product_key_hash: None,
        }
    }


    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of ProductKeyHash
    pub fn set_product_key_hash(&mut self, value: String) {
        self.product_key_hash = Some(value);
    }

    /// Gets the value of ProductKeyHash
    pub fn get_product_key_hash(&self) -> Option<&String> {
        self.product_key_hash.as_ref()
    }

/// 

    /// * `product_key` -  (String)

    /// * `return_value` -  (u32)
    pub fn activate_key(&self, product_key: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProductKey".to_string(), value: product_key.into() });
        self.invoke_method("ActivateKey", &args)

    }


/// 

    /// * `certificate_blob` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_certificate(&self, certificate_blob: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CertificateBlob".to_string(), value: certificate_blob.into() });
        self.invoke_method("AddCertificate", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn un_activate_lob(&self) -> Result<(), WmiError> {
        self.invoke_method("UnActivateLOB", &[])

    }

}

