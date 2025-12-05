// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbClientCertificateMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbClientCertificateMapping {

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<SmbClientCertificateMapping_Flags>,

/// 
    #[serde(rename = "IssuerName")]
    pub issuer_name: Option<String>,

/// 
    #[serde(rename = "MappingStatus")]
    pub mapping_status: Option<u32>,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "RenewalChain")]
    pub renewal_chain: Option<String>,

/// 
    #[serde(rename = "StoreName")]
    pub store_name: Option<String>,

/// 
    #[serde(rename = "Subject")]
    pub subject: Option<String>,

/// 
    #[serde(rename = "Thumbprint")]
    pub thumbprint: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<SmbClientCertificateMapping_Type>,
}

impl MSFT_SmbClientCertificateMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            flags: None,
            issuer_name: None,
            mapping_status: None,
            namespace: None,
            renewal_chain: None,
            store_name: None,
            subject: None,
            thumbprint: None,
            type: None,
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: SmbClientCertificateMapping_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&SmbClientCertificateMapping_Flags> {
        self.flags.as_ref()
    }

    /// Sets the value of IssuerName
    pub fn set_issuer_name(&mut self, value: String) {
        self.issuer_name = Some(value);
    }

    /// Gets the value of IssuerName
    pub fn get_issuer_name(&self) -> Option<&String> {
        self.issuer_name.as_ref()
    }

    /// Sets the value of MappingStatus
    pub fn set_mapping_status(&mut self, value: u32) {
        self.mapping_status = Some(value);
    }

    /// Gets the value of MappingStatus
    pub fn get_mapping_status(&self) -> Option<&u32> {
        self.mapping_status.as_ref()
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of RenewalChain
    pub fn set_renewal_chain(&mut self, value: String) {
        self.renewal_chain = Some(value);
    }

    /// Gets the value of RenewalChain
    pub fn get_renewal_chain(&self) -> Option<&String> {
        self.renewal_chain.as_ref()
    }

    /// Sets the value of StoreName
    pub fn set_store_name(&mut self, value: String) {
        self.store_name = Some(value);
    }

    /// Gets the value of StoreName
    pub fn get_store_name(&self) -> Option<&String> {
        self.store_name.as_ref()
    }

    /// Sets the value of Subject
    pub fn set_subject(&mut self, value: String) {
        self.subject = Some(value);
    }

    /// Gets the value of Subject
    pub fn get_subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    /// Sets the value of Thumbprint
    pub fn set_thumbprint(&mut self, value: String) {
        self.thumbprint = Some(value);
    }

    /// Gets the value of Thumbprint
    pub fn get_thumbprint(&self) -> Option<&String> {
        self.thumbprint.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: SmbClientCertificateMapping_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&SmbClientCertificateMapping_Type> {
        self.type.as_ref()
    }

/// 

    /// * `display_name` -  (String)
    /// * `flags` -  (u32)
    /// * `issuer_name` -  (String)
    /// * `namespace` -  (String)
    /// * `store_name` -  (String)
    /// * `subject` -  (String)
    /// * `thumbprint` -  (String)
    /// * `type` -  (u32)

    /// * `created_mapping` -  (MSFT_SmbClientCertificateMapping)
    /// * `return_value` -  (u32)
    pub fn add_client_certificate_mapping(&self, namespace: &String, issuer_name: &String, subject: &String, thumbprint: &String, display_name: &String, store_name: &String, type: u32, flags: u32, created_mapping: &mut MSFT_SmbClientCertificateMapping) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });
        args.push(MethodParameter { name: "IssuerName".to_string(), value: issuer_name.into() });
        args.push(MethodParameter { name: "Subject".to_string(), value: subject.into() });
        args.push(MethodParameter { name: "Thumbprint".to_string(), value: thumbprint.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "StoreName".to_string(), value: store_name.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("AddClientCertificateMapping", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `issuer_name` -  (String)
    /// * `namespace` -  (String)
    /// * `store_name` -  (String)
    /// * `thumbprint` -  (String)

    /// * `created_mapping` -  (MSFT_SmbClientCertificateMapping)
    /// * `return_value` -  (u32)
    pub fn set_client_certificate_mapping(&self, namespace: &String, issuer_name: &String, thumbprint: &String, store_name: &String, flags: u32, created_mapping: &mut MSFT_SmbClientCertificateMapping) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });
        args.push(MethodParameter { name: "IssuerName".to_string(), value: issuer_name.into() });
        args.push(MethodParameter { name: "Thumbprint".to_string(), value: thumbprint.into() });
        args.push(MethodParameter { name: "StoreName".to_string(), value: store_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("SetClientCertificateMapping", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }

}

