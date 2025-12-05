// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbShare struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbShare {

/// 
    #[serde(rename = "AvailabilityType")]
    pub availability_type: Option<SmbShare_AvailabilityType>,

/// 
    #[serde(rename = "CachingMode")]
    pub caching_mode: Option<SmbShare_CachingMode>,

/// 
    #[serde(rename = "CATimeout")]
    pub catimeout: Option<u32>,

/// 
    #[serde(rename = "CompressData")]
    pub compress_data: Option<bool>,

/// 
    #[serde(rename = "ConcurrentUserLimit")]
    pub concurrent_user_limit: Option<u32>,

/// 
    #[serde(rename = "ContinuouslyAvailable")]
    pub continuously_available: Option<bool>,

/// 
    #[serde(rename = "CurrentUsers")]
    pub current_users: Option<u32>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "DirectoryHandleLeasing")]
    pub directory_handle_leasing: Option<bool>,

/// 
    #[serde(rename = "EncryptData")]
    pub encrypt_data: Option<bool>,

/// 
    #[serde(rename = "FolderEnumerationMode")]
    pub folder_enumeration_mode: Option<SmbShare_FolderEnumerationMode>,

/// 
    #[serde(rename = "IdentityRemoting")]
    pub identity_remoting: Option<bool>,

/// 
    #[serde(rename = "Infrastructure")]
    pub infrastructure: Option<bool>,

/// 
    #[serde(rename = "IsolatedTransport")]
    pub isolated_transport: Option<bool>,

/// 
    #[serde(rename = "LeasingMode")]
    pub leasing_mode: Option<SmbShare_LeasingMode>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "QoSFlowScope")]
    pub qo_sflow_scope: Option<SmbShare_QoSFlowScope>,

/// 
    #[serde(rename = "QoSPolicyId")]
    pub qo_spolicy_id: Option<String>,

/// 
    #[serde(rename = "Scoped")]
    pub scoped: Option<bool>,

/// 
    #[serde(rename = "ScopeName")]
    pub scope_name: Option<String>,

/// 
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Option<String>,

/// 
    #[serde(rename = "ShadowCopy")]
    pub shadow_copy: Option<bool>,

/// 
    #[serde(rename = "ShareState")]
    pub share_state: Option<SmbShare_ShareState>,

/// 
    #[serde(rename = "ShareType")]
    pub share_type: Option<SmbShare_ShareType>,

/// 
    #[serde(rename = "SmbInstance")]
    pub smb_instance: Option<SmbShare_SmbInstance>,

/// 
    #[serde(rename = "Special")]
    pub special: Option<bool>,

/// 
    #[serde(rename = "Temporary")]
    pub temporary: Option<bool>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<String>,
}

impl MSFT_SmbShare {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            availability_type: None,
            caching_mode: None,
            catimeout: None,
            compress_data: None,
            concurrent_user_limit: None,
            continuously_available: None,
            current_users: None,
            description: None,
            directory_handle_leasing: None,
            encrypt_data: None,
            folder_enumeration_mode: None,
            identity_remoting: None,
            infrastructure: None,
            isolated_transport: None,
            leasing_mode: None,
            name: None,
            path: None,
            qo_sflow_scope: None,
            qo_spolicy_id: None,
            scoped: None,
            scope_name: None,
            security_descriptor: None,
            shadow_copy: None,
            share_state: None,
            share_type: None,
            smb_instance: None,
            special: None,
            temporary: None,
            volume: None,
        }
    }


    /// Sets the value of AvailabilityType
    pub fn set_availability_type(&mut self, value: SmbShare_AvailabilityType) {
        self.availability_type = Some(value);
    }

    /// Gets the value of AvailabilityType
    pub fn get_availability_type(&self) -> Option<&SmbShare_AvailabilityType> {
        self.availability_type.as_ref()
    }

    /// Sets the value of CachingMode
    pub fn set_caching_mode(&mut self, value: SmbShare_CachingMode) {
        self.caching_mode = Some(value);
    }

    /// Gets the value of CachingMode
    pub fn get_caching_mode(&self) -> Option<&SmbShare_CachingMode> {
        self.caching_mode.as_ref()
    }

    /// Sets the value of CATimeout
    pub fn set_catimeout(&mut self, value: u32) {
        self.catimeout = Some(value);
    }

    /// Gets the value of CATimeout
    pub fn get_catimeout(&self) -> Option<&u32> {
        self.catimeout.as_ref()
    }

    /// Sets the value of CompressData
    pub fn set_compress_data(&mut self, value: bool) {
        self.compress_data = Some(value);
    }

    /// Gets the value of CompressData
    pub fn get_compress_data(&self) -> Option<&bool> {
        self.compress_data.as_ref()
    }

    /// Sets the value of ConcurrentUserLimit
    pub fn set_concurrent_user_limit(&mut self, value: u32) {
        self.concurrent_user_limit = Some(value);
    }

    /// Gets the value of ConcurrentUserLimit
    pub fn get_concurrent_user_limit(&self) -> Option<&u32> {
        self.concurrent_user_limit.as_ref()
    }

    /// Sets the value of ContinuouslyAvailable
    pub fn set_continuously_available(&mut self, value: bool) {
        self.continuously_available = Some(value);
    }

    /// Gets the value of ContinuouslyAvailable
    pub fn get_continuously_available(&self) -> Option<&bool> {
        self.continuously_available.as_ref()
    }

    /// Sets the value of CurrentUsers
    pub fn set_current_users(&mut self, value: u32) {
        self.current_users = Some(value);
    }

    /// Gets the value of CurrentUsers
    pub fn get_current_users(&self) -> Option<&u32> {
        self.current_users.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of DirectoryHandleLeasing
    pub fn set_directory_handle_leasing(&mut self, value: bool) {
        self.directory_handle_leasing = Some(value);
    }

    /// Gets the value of DirectoryHandleLeasing
    pub fn get_directory_handle_leasing(&self) -> Option<&bool> {
        self.directory_handle_leasing.as_ref()
    }

    /// Sets the value of EncryptData
    pub fn set_encrypt_data(&mut self, value: bool) {
        self.encrypt_data = Some(value);
    }

    /// Gets the value of EncryptData
    pub fn get_encrypt_data(&self) -> Option<&bool> {
        self.encrypt_data.as_ref()
    }

    /// Sets the value of FolderEnumerationMode
    pub fn set_folder_enumeration_mode(&mut self, value: SmbShare_FolderEnumerationMode) {
        self.folder_enumeration_mode = Some(value);
    }

    /// Gets the value of FolderEnumerationMode
    pub fn get_folder_enumeration_mode(&self) -> Option<&SmbShare_FolderEnumerationMode> {
        self.folder_enumeration_mode.as_ref()
    }

    /// Sets the value of IdentityRemoting
    pub fn set_identity_remoting(&mut self, value: bool) {
        self.identity_remoting = Some(value);
    }

    /// Gets the value of IdentityRemoting
    pub fn get_identity_remoting(&self) -> Option<&bool> {
        self.identity_remoting.as_ref()
    }

    /// Sets the value of Infrastructure
    pub fn set_infrastructure(&mut self, value: bool) {
        self.infrastructure = Some(value);
    }

    /// Gets the value of Infrastructure
    pub fn get_infrastructure(&self) -> Option<&bool> {
        self.infrastructure.as_ref()
    }

    /// Sets the value of IsolatedTransport
    pub fn set_isolated_transport(&mut self, value: bool) {
        self.isolated_transport = Some(value);
    }

    /// Gets the value of IsolatedTransport
    pub fn get_isolated_transport(&self) -> Option<&bool> {
        self.isolated_transport.as_ref()
    }

    /// Sets the value of LeasingMode
    pub fn set_leasing_mode(&mut self, value: SmbShare_LeasingMode) {
        self.leasing_mode = Some(value);
    }

    /// Gets the value of LeasingMode
    pub fn get_leasing_mode(&self) -> Option<&SmbShare_LeasingMode> {
        self.leasing_mode.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of QoSFlowScope
    pub fn set_qo_sflow_scope(&mut self, value: SmbShare_QoSFlowScope) {
        self.qo_sflow_scope = Some(value);
    }

    /// Gets the value of QoSFlowScope
    pub fn get_qo_sflow_scope(&self) -> Option<&SmbShare_QoSFlowScope> {
        self.qo_sflow_scope.as_ref()
    }

    /// Sets the value of QoSPolicyId
    pub fn set_qo_spolicy_id(&mut self, value: String) {
        self.qo_spolicy_id = Some(value);
    }

    /// Gets the value of QoSPolicyId
    pub fn get_qo_spolicy_id(&self) -> Option<&String> {
        self.qo_spolicy_id.as_ref()
    }

    /// Sets the value of Scoped
    pub fn set_scoped(&mut self, value: bool) {
        self.scoped = Some(value);
    }

    /// Gets the value of Scoped
    pub fn get_scoped(&self) -> Option<&bool> {
        self.scoped.as_ref()
    }

    /// Sets the value of ScopeName
    pub fn set_scope_name(&mut self, value: String) {
        self.scope_name = Some(value);
    }

    /// Gets the value of ScopeName
    pub fn get_scope_name(&self) -> Option<&String> {
        self.scope_name.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: String) {
        self.security_descriptor = Some(value);
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> Option<&String> {
        self.security_descriptor.as_ref()
    }

    /// Sets the value of ShadowCopy
    pub fn set_shadow_copy(&mut self, value: bool) {
        self.shadow_copy = Some(value);
    }

    /// Gets the value of ShadowCopy
    pub fn get_shadow_copy(&self) -> Option<&bool> {
        self.shadow_copy.as_ref()
    }

    /// Sets the value of ShareState
    pub fn set_share_state(&mut self, value: SmbShare_ShareState) {
        self.share_state = Some(value);
    }

    /// Gets the value of ShareState
    pub fn get_share_state(&self) -> Option<&SmbShare_ShareState> {
        self.share_state.as_ref()
    }

    /// Sets the value of ShareType
    pub fn set_share_type(&mut self, value: SmbShare_ShareType) {
        self.share_type = Some(value);
    }

    /// Gets the value of ShareType
    pub fn get_share_type(&self) -> Option<&SmbShare_ShareType> {
        self.share_type.as_ref()
    }

    /// Sets the value of SmbInstance
    pub fn set_smb_instance(&mut self, value: SmbShare_SmbInstance) {
        self.smb_instance = Some(value);
    }

    /// Gets the value of SmbInstance
    pub fn get_smb_instance(&self) -> Option<&SmbShare_SmbInstance> {
        self.smb_instance.as_ref()
    }

    /// Sets the value of Special
    pub fn set_special(&mut self, value: bool) {
        self.special = Some(value);
    }

    /// Gets the value of Special
    pub fn get_special(&self) -> Option<&bool> {
        self.special.as_ref()
    }

    /// Sets the value of Temporary
    pub fn set_temporary(&mut self, value: bool) {
        self.temporary = Some(value);
    }

    /// Gets the value of Temporary
    pub fn get_temporary(&self) -> Option<&bool> {
        self.temporary.as_ref()
    }

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: String) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&String> {
        self.volume.as_ref()
    }

/// 

    /// * `caching_mode` -  (u32)
    /// * `catimeout` -  (u32)
    /// * `change_access` -  (String[])
    /// * `compress_data` -  (bool)
    /// * `concurrent_user_limit` -  (u32)
    /// * `continuously_available` -  (bool)
    /// * `description` -  (String)
    /// * `directory_handle_leasing` -  (bool)
    /// * `encrypt_data` -  (bool)
    /// * `folder_enumeration_mode` -  (u32)
    /// * `full_access` -  (String[])
    /// * `isolated_transport` -  (bool)
    /// * `leasing_mode` -  (u32)
    /// * `name` -  (String)
    /// * `no_access` -  (String[])
    /// * `path` -  (String)
    /// * `qo_sflow_scope` -  (u32)
    /// * `qo_spolicy_id` -  (String)
    /// * `read_access` -  (String[])
    /// * `scope_name` -  (String)
    /// * `security_descriptor` -  (String)
    /// * `temporary` -  (bool)

    /// * `created_share` -  (MSFT_SmbShare)
    /// * `return_value` -  (u32)
    pub fn create_share(&self, name: &String, scope_name: &String, path: &String, description: &String, concurrent_user_limit: u32, folder_enumeration_mode: u32, caching_mode: u32, temporary: bool, continuously_available: bool, catimeout: u32, encrypt_data: bool, compress_data: bool, isolated_transport: bool, full_access: &Vec<String>, change_access: &Vec<String>, read_access: &Vec<String>, no_access: &Vec<String>, security_descriptor: &String, leasing_mode: u32, directory_handle_leasing: bool, qo_sflow_scope: u32, qo_spolicy_id: &String, created_share: &mut MSFT_SmbShare) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ScopeName".to_string(), value: scope_name.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "ConcurrentUserLimit".to_string(), value: concurrent_user_limit.into() });
        args.push(MethodParameter { name: "FolderEnumerationMode".to_string(), value: folder_enumeration_mode.into() });
        args.push(MethodParameter { name: "CachingMode".to_string(), value: caching_mode.into() });
        args.push(MethodParameter { name: "Temporary".to_string(), value: temporary.into() });
        args.push(MethodParameter { name: "ContinuouslyAvailable".to_string(), value: continuously_available.into() });
        args.push(MethodParameter { name: "CATimeout".to_string(), value: catimeout.into() });
        args.push(MethodParameter { name: "EncryptData".to_string(), value: encrypt_data.into() });
        args.push(MethodParameter { name: "CompressData".to_string(), value: compress_data.into() });
        args.push(MethodParameter { name: "IsolatedTransport".to_string(), value: isolated_transport.into() });
        args.push(MethodParameter { name: "FullAccess".to_string(), value: full_access.into() });
        args.push(MethodParameter { name: "ChangeAccess".to_string(), value: change_access.into() });
        args.push(MethodParameter { name: "ReadAccess".to_string(), value: read_access.into() });
        args.push(MethodParameter { name: "NoAccess".to_string(), value: no_access.into() });
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });
        args.push(MethodParameter { name: "LeasingMode".to_string(), value: leasing_mode.into() });
        args.push(MethodParameter { name: "DirectoryHandleLeasing".to_string(), value: directory_handle_leasing.into() });
        args.push(MethodParameter { name: "QoSFlowScope".to_string(), value: qo_sflow_scope.into() });
        args.push(MethodParameter { name: "QoSPolicyId".to_string(), value: qo_spolicy_id.into() });

        let result = self.invoke_method("CreateShare", &args)?;
        let created_share = result.get_value("CreatedShare")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_right` -  (u32)
    /// * `account_name` -  (String[])

    /// * `output` -  (MSFT_SmbShareAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn grant_access(&self, account_name: &Vec<String>, access_right: u32, output: &mut Vec<MSFT_SmbShareAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });
        args.push(MethodParameter { name: "AccessRight".to_string(), value: access_right.into() });

        let result = self.invoke_method("GrantAccess", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_name` -  (String[])

    /// * `output` -  (MSFT_SmbShareAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn revoke_access(&self, account_name: &Vec<String>, output: &mut Vec<MSFT_SmbShareAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });

        let result = self.invoke_method("RevokeAccess", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_name` -  (String[])

    /// * `output` -  (MSFT_SmbShareAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn block_access(&self, account_name: &Vec<String>, output: &mut Vec<MSFT_SmbShareAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });

        let result = self.invoke_method("BlockAccess", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `account_name` -  (String[])

    /// * `output` -  (MSFT_SmbShareAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn unblock_access(&self, account_name: &Vec<String>, output: &mut Vec<MSFT_SmbShareAccessControlEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccountName".to_string(), value: account_name.into() });

        let result = self.invoke_method("UnblockAccess", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `output` -  (MSFT_SmbShareAccessControlEntry[])
    /// * `return_value` -  (u32)
    pub fn get_access_control_entries(&self, output: &mut Vec<MSFT_SmbShareAccessControlEntry>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAccessControlEntries", &[])?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `populate_volume_property` -  (bool)
    /// * `scope_name` -  (String)

    /// * `output` -  (MSFT_SmbShare[])
    /// * `return_value` -  (u32)
    pub fn enumerate_shares(&self, scope_name: &String, populate_volume_property: bool, output: &mut Vec<MSFT_SmbShare>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScopeName".to_string(), value: scope_name.into() });
        args.push(MethodParameter { name: "PopulateVolumeProperty".to_string(), value: populate_volume_property.into() });

        let result = self.invoke_method("EnumerateShares", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `get_acl_non_admin` -  (bool)
    /// * `scope_name` -  (String)
    /// * `share_name` -  (String)

    /// * `output` -  (MSFT_SmbShare)
    /// * `return_value` -  (u32)
    pub fn get_share(&self, scope_name: &String, share_name: &String, get_acl_non_admin: bool, output: &mut MSFT_SmbShare) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScopeName".to_string(), value: scope_name.into() });
        args.push(MethodParameter { name: "ShareName".to_string(), value: share_name.into() });
        args.push(MethodParameter { name: "GetAclNonAdmin".to_string(), value: get_acl_non_admin.into() });

        let result = self.invoke_method("GetShare", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `catimeout` -  (u32)
    /// * `concurrent_user_limit` -  (u32)
    /// * `event_type` -  (u32)
    /// * `flags` -  (u32)
    /// * `path` -  (String)
    /// * `qo_sflow_scope` -  (u32)
    /// * `qo_spolicy_id` -  (String)
    /// * `remark` -  (String)
    /// * `scope_name` -  (String)
    /// * `security_descriptor` -  (String)
    /// * `share_name` -  (String)
    /// * `share_state` -  (u32)
    /// * `type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn fire_share_change_event(&self, event_type: u32, scope_name: &String, share_name: &String, path: &String, remark: &String, security_descriptor: &String, share_state: u32, catimeout: u32, flags: u32, type: u32, concurrent_user_limit: u32, qo_sflow_scope: u32, qo_spolicy_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EventType".to_string(), value: event_type.into() });
        args.push(MethodParameter { name: "ScopeName".to_string(), value: scope_name.into() });
        args.push(MethodParameter { name: "ShareName".to_string(), value: share_name.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Remark".to_string(), value: remark.into() });
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });
        args.push(MethodParameter { name: "ShareState".to_string(), value: share_state.into() });
        args.push(MethodParameter { name: "CATimeout".to_string(), value: catimeout.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "ConcurrentUserLimit".to_string(), value: concurrent_user_limit.into() });
        args.push(MethodParameter { name: "QoSFlowScope".to_string(), value: qo_sflow_scope.into() });
        args.push(MethodParameter { name: "QoSPolicyId".to_string(), value: qo_spolicy_id.into() });
        self.invoke_method("FireShareChangeEvent", &args)

    }

}

