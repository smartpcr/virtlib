// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbServerCertificateMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbServerCertificateMapping {

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<SmbServerCertificateMapping_Flags>,

/// 
    #[serde(rename = "MappingStatus")]
    pub mapping_status: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "RenewalChain")]
    pub renewal_chain: Option<String>,

/// 
    #[serde(rename = "RequireClientAuthentication")]
    pub require_client_authentication: Option<bool>,

/// 
    #[serde(rename = "SkipClientCertificateAccessCheck")]
    pub skip_client_certificate_access_check: Option<bool>,

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
    pub type: Option<SmbServerCertificateMapping_Type>,
}

impl MSFT_SmbServerCertificateMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            flags: None,
            mapping_status: None,
            name: None,
            renewal_chain: None,
            require_client_authentication: None,
            skip_client_certificate_access_check: None,
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
    pub fn set_flags(&mut self, value: SmbServerCertificateMapping_Flags) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&SmbServerCertificateMapping_Flags> {
        self.flags.as_ref()
    }

    /// Sets the value of MappingStatus
    pub fn set_mapping_status(&mut self, value: u32) {
        self.mapping_status = Some(value);
    }

    /// Gets the value of MappingStatus
    pub fn get_mapping_status(&self) -> Option<&u32> {
        self.mapping_status.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of RenewalChain
    pub fn set_renewal_chain(&mut self, value: String) {
        self.renewal_chain = Some(value);
    }

    /// Gets the value of RenewalChain
    pub fn get_renewal_chain(&self) -> Option<&String> {
        self.renewal_chain.as_ref()
    }

    /// Sets the value of RequireClientAuthentication
    pub fn set_require_client_authentication(&mut self, value: bool) {
        self.require_client_authentication = Some(value);
    }

    /// Gets the value of RequireClientAuthentication
    pub fn get_require_client_authentication(&self) -> Option<&bool> {
        self.require_client_authentication.as_ref()
    }

    /// Sets the value of SkipClientCertificateAccessCheck
    pub fn set_skip_client_certificate_access_check(&mut self, value: bool) {
        self.skip_client_certificate_access_check = Some(value);
    }

    /// Gets the value of SkipClientCertificateAccessCheck
    pub fn get_skip_client_certificate_access_check(&self) -> Option<&bool> {
        self.skip_client_certificate_access_check.as_ref()
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
    pub fn set_type(&mut self, value: SmbServerCertificateMapping_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&SmbServerCertificateMapping_Type> {
        self.type.as_ref()
    }

/// 

    /// * `display_name` -  (String)
    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `require_client_authentication` -  (bool)
    /// * `skip_client_certificate_access_check` -  (bool)
    /// * `store_name` -  (String)
    /// * `subject` -  (String)
    /// * `thumbprint` -  (String)
    /// * `type` -  (u32)

    /// * `created_mapping` -  (MSFT_SmbServerCertificateMapping)
    /// * `return_value` -  (u32)
    pub fn add_server_certificate_mapping(&self, name: &String, subject: &String, thumbprint: &String, display_name: &String, store_name: &String, type: u32, flags: u32, require_client_authentication: bool, skip_client_certificate_access_check: bool, created_mapping: &mut MSFT_SmbServerCertificateMapping) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Subject".to_string(), value: subject.into() });
        args.push(MethodParameter { name: "Thumbprint".to_string(), value: thumbprint.into() });
        args.push(MethodParameter { name: "DisplayName".to_string(), value: display_name.into() });
        args.push(MethodParameter { name: "StoreName".to_string(), value: store_name.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "RequireClientAuthentication".to_string(), value: require_client_authentication.into() });
        args.push(MethodParameter { name: "SkipClientCertificateAccessCheck".to_string(), value: skip_client_certificate_access_check.into() });

        let result = self.invoke_method("AddServerCertificateMapping", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `require_client_authentication` -  (bool)
    /// * `skip_client_certificate_access_check` -  (bool)
    /// * `store_name` -  (String)
    /// * `thumbprint` -  (String)

    /// * `created_mapping` -  (MSFT_SmbServerCertificateMapping)
    /// * `return_value` -  (u32)
    pub fn set_server_certificate_mapping(&self, name: &String, thumbprint: &String, store_name: &String, flags: u32, require_client_authentication: bool, skip_client_certificate_access_check: bool, created_mapping: &mut MSFT_SmbServerCertificateMapping) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Thumbprint".to_string(), value: thumbprint.into() });
        args.push(MethodParameter { name: "StoreName".to_string(), value: store_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "RequireClientAuthentication".to_string(), value: require_client_authentication.into() });
        args.push(MethodParameter { name: "SkipClientCertificateAccessCheck".to_string(), value: skip_client_certificate_access_check.into() });

        let result = self.invoke_method("SetServerCertificateMapping", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `identifier` -  (String)
    /// * `identifier_type` -  (u32)

    /// * `output` -  (MSFT_SmbServerCertificateMappingAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn grant_client_access_to_server(&self, identifier_type: u32, identifier: &String, description: &String, output: &mut Vec<MSFT_SmbServerCertificateMappingAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentifierType".to_string(), value: identifier_type.into() });
        args.push(MethodParameter { name: "Identifier".to_string(), value: identifier.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("GrantClientAccessToServer", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `identifier` -  (String)
    /// * `identifier_type` -  (u32)

    /// * `output` -  (MSFT_SmbServerCertificateMappingAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn revoke_client_access_to_server(&self, identifier_type: u32, identifier: &String, output: &mut Vec<MSFT_SmbServerCertificateMappingAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentifierType".to_string(), value: identifier_type.into() });
        args.push(MethodParameter { name: "Identifier".to_string(), value: identifier.into() });

        let result = self.invoke_method("RevokeClientAccessToServer", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `identifier` -  (String)
    /// * `identifier_type` -  (u32)

    /// * `output` -  (MSFT_SmbServerCertificateMappingAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn block_client_access_to_server(&self, identifier_type: u32, identifier: &String, description: &String, output: &mut Vec<MSFT_SmbServerCertificateMappingAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentifierType".to_string(), value: identifier_type.into() });
        args.push(MethodParameter { name: "Identifier".to_string(), value: identifier.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("BlockClientAccessToServer", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `identifier` -  (String)
    /// * `identifier_type` -  (u32)

    /// * `output` -  (MSFT_SmbServerCertificateMappingAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn unblock_client_access_to_server(&self, identifier_type: u32, identifier: &String, output: &mut Vec<MSFT_SmbServerCertificateMappingAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentifierType".to_string(), value: identifier_type.into() });
        args.push(MethodParameter { name: "Identifier".to_string(), value: identifier.into() });

        let result = self.invoke_method("UnblockClientAccessToServer", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_control_type` -  (u32)
    /// * `identifier` -  (String)
    /// * `identifier_type` -  (u32)

    /// * `output` -  (MSFT_SmbServerCertificateMappingAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn get_access_control_entries(&self, identifier_type: u32, identifier: &String, access_control_type: u32, output: &mut Vec<MSFT_SmbServerCertificateMappingAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentifierType".to_string(), value: identifier_type.into() });
        args.push(MethodParameter { name: "Identifier".to_string(), value: identifier.into() });
        args.push(MethodParameter { name: "AccessControlType".to_string(), value: access_control_type.into() });

        let result = self.invoke_method("GetAccessControlEntries", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }

}

