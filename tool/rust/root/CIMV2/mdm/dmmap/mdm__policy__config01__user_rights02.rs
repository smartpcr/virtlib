// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_UserRights02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_UserRights02 {

/// 
    #[serde(rename = "AccessCredentialManagerAsTrustedCaller")]
    pub access_credential_manager_as_trusted_caller: Option<String>,

/// 
    #[serde(rename = "AccessFromNetwork")]
    pub access_from_network: Option<String>,

/// 
    #[serde(rename = "ActAsPartOfTheOperatingSystem")]
    pub act_as_part_of_the_operating_system: Option<String>,

/// 
    #[serde(rename = "AllowLocalLogOn")]
    pub allow_local_log_on: Option<String>,

/// 
    #[serde(rename = "BackupFilesAndDirectories")]
    pub backup_files_and_directories: Option<String>,

/// 
    #[serde(rename = "ChangeSystemTime")]
    pub change_system_time: Option<String>,

/// 
    #[serde(rename = "CreateGlobalObjects")]
    pub create_global_objects: Option<String>,

/// 
    #[serde(rename = "CreatePageFile")]
    pub create_page_file: Option<String>,

/// 
    #[serde(rename = "CreatePermanentSharedObjects")]
    pub create_permanent_shared_objects: Option<String>,

/// 
    #[serde(rename = "CreateSymbolicLinks")]
    pub create_symbolic_links: Option<String>,

/// 
    #[serde(rename = "CreateToken")]
    pub create_token: Option<String>,

/// 
    #[serde(rename = "DebugPrograms")]
    pub debug_programs: Option<String>,

/// 
    #[serde(rename = "DenyAccessFromNetwork")]
    pub deny_access_from_network: Option<String>,

/// 
    #[serde(rename = "DenyLocalLogOn")]
    pub deny_local_log_on: Option<String>,

/// 
    #[serde(rename = "DenyRemoteDesktopServicesLogOn")]
    pub deny_remote_desktop_services_log_on: Option<String>,

/// 
    #[serde(rename = "EnableDelegation")]
    pub enable_delegation: Option<String>,

/// 
    #[serde(rename = "GenerateSecurityAudits")]
    pub generate_security_audits: Option<String>,

/// 
    #[serde(rename = "ImpersonateClient")]
    pub impersonate_client: Option<String>,

/// 
    #[serde(rename = "IncreaseSchedulingPriority")]
    pub increase_scheduling_priority: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LoadUnloadDeviceDrivers")]
    pub load_unload_device_drivers: Option<String>,

/// 
    #[serde(rename = "LockMemory")]
    pub lock_memory: Option<String>,

/// 
    #[serde(rename = "ManageAuditingAndSecurityLog")]
    pub manage_auditing_and_security_log: Option<String>,

/// 
    #[serde(rename = "ManageVolume")]
    pub manage_volume: Option<String>,

/// 
    #[serde(rename = "ModifyFirmwareEnvironment")]
    pub modify_firmware_environment: Option<String>,

/// 
    #[serde(rename = "ModifyObjectLabel")]
    pub modify_object_label: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ProfileSingleProcess")]
    pub profile_single_process: Option<String>,

/// 
    #[serde(rename = "RemoteShutdown")]
    pub remote_shutdown: Option<String>,

/// 
    #[serde(rename = "RestoreFilesAndDirectories")]
    pub restore_files_and_directories: Option<String>,

/// 
    #[serde(rename = "TakeOwnership")]
    pub take_ownership: Option<String>,
}

impl MDM_Policy_Config01_UserRights02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_credential_manager_as_trusted_caller: None,
            access_from_network: None,
            act_as_part_of_the_operating_system: None,
            allow_local_log_on: None,
            backup_files_and_directories: None,
            change_system_time: None,
            create_global_objects: None,
            create_page_file: None,
            create_permanent_shared_objects: None,
            create_symbolic_links: None,
            create_token: None,
            debug_programs: None,
            deny_access_from_network: None,
            deny_local_log_on: None,
            deny_remote_desktop_services_log_on: None,
            enable_delegation: None,
            generate_security_audits: None,
            impersonate_client: None,
            increase_scheduling_priority: None,
            instance_id: None,
            load_unload_device_drivers: None,
            lock_memory: None,
            manage_auditing_and_security_log: None,
            manage_volume: None,
            modify_firmware_environment: None,
            modify_object_label: None,
            parent_id: None,
            profile_single_process: None,
            remote_shutdown: None,
            restore_files_and_directories: None,
            take_ownership: None,
        }
    }


    /// Sets the value of AccessCredentialManagerAsTrustedCaller
    pub fn set_access_credential_manager_as_trusted_caller(&mut self, value: String) {
        self.access_credential_manager_as_trusted_caller = Some(value);
    }

    /// Gets the value of AccessCredentialManagerAsTrustedCaller
    pub fn get_access_credential_manager_as_trusted_caller(&self) -> Option<&String> {
        self.access_credential_manager_as_trusted_caller.as_ref()
    }

    /// Sets the value of AccessFromNetwork
    pub fn set_access_from_network(&mut self, value: String) {
        self.access_from_network = Some(value);
    }

    /// Gets the value of AccessFromNetwork
    pub fn get_access_from_network(&self) -> Option<&String> {
        self.access_from_network.as_ref()
    }

    /// Sets the value of ActAsPartOfTheOperatingSystem
    pub fn set_act_as_part_of_the_operating_system(&mut self, value: String) {
        self.act_as_part_of_the_operating_system = Some(value);
    }

    /// Gets the value of ActAsPartOfTheOperatingSystem
    pub fn get_act_as_part_of_the_operating_system(&self) -> Option<&String> {
        self.act_as_part_of_the_operating_system.as_ref()
    }

    /// Sets the value of AllowLocalLogOn
    pub fn set_allow_local_log_on(&mut self, value: String) {
        self.allow_local_log_on = Some(value);
    }

    /// Gets the value of AllowLocalLogOn
    pub fn get_allow_local_log_on(&self) -> Option<&String> {
        self.allow_local_log_on.as_ref()
    }

    /// Sets the value of BackupFilesAndDirectories
    pub fn set_backup_files_and_directories(&mut self, value: String) {
        self.backup_files_and_directories = Some(value);
    }

    /// Gets the value of BackupFilesAndDirectories
    pub fn get_backup_files_and_directories(&self) -> Option<&String> {
        self.backup_files_and_directories.as_ref()
    }

    /// Sets the value of ChangeSystemTime
    pub fn set_change_system_time(&mut self, value: String) {
        self.change_system_time = Some(value);
    }

    /// Gets the value of ChangeSystemTime
    pub fn get_change_system_time(&self) -> Option<&String> {
        self.change_system_time.as_ref()
    }

    /// Sets the value of CreateGlobalObjects
    pub fn set_create_global_objects(&mut self, value: String) {
        self.create_global_objects = Some(value);
    }

    /// Gets the value of CreateGlobalObjects
    pub fn get_create_global_objects(&self) -> Option<&String> {
        self.create_global_objects.as_ref()
    }

    /// Sets the value of CreatePageFile
    pub fn set_create_page_file(&mut self, value: String) {
        self.create_page_file = Some(value);
    }

    /// Gets the value of CreatePageFile
    pub fn get_create_page_file(&self) -> Option<&String> {
        self.create_page_file.as_ref()
    }

    /// Sets the value of CreatePermanentSharedObjects
    pub fn set_create_permanent_shared_objects(&mut self, value: String) {
        self.create_permanent_shared_objects = Some(value);
    }

    /// Gets the value of CreatePermanentSharedObjects
    pub fn get_create_permanent_shared_objects(&self) -> Option<&String> {
        self.create_permanent_shared_objects.as_ref()
    }

    /// Sets the value of CreateSymbolicLinks
    pub fn set_create_symbolic_links(&mut self, value: String) {
        self.create_symbolic_links = Some(value);
    }

    /// Gets the value of CreateSymbolicLinks
    pub fn get_create_symbolic_links(&self) -> Option<&String> {
        self.create_symbolic_links.as_ref()
    }

    /// Sets the value of CreateToken
    pub fn set_create_token(&mut self, value: String) {
        self.create_token = Some(value);
    }

    /// Gets the value of CreateToken
    pub fn get_create_token(&self) -> Option<&String> {
        self.create_token.as_ref()
    }

    /// Sets the value of DebugPrograms
    pub fn set_debug_programs(&mut self, value: String) {
        self.debug_programs = Some(value);
    }

    /// Gets the value of DebugPrograms
    pub fn get_debug_programs(&self) -> Option<&String> {
        self.debug_programs.as_ref()
    }

    /// Sets the value of DenyAccessFromNetwork
    pub fn set_deny_access_from_network(&mut self, value: String) {
        self.deny_access_from_network = Some(value);
    }

    /// Gets the value of DenyAccessFromNetwork
    pub fn get_deny_access_from_network(&self) -> Option<&String> {
        self.deny_access_from_network.as_ref()
    }

    /// Sets the value of DenyLocalLogOn
    pub fn set_deny_local_log_on(&mut self, value: String) {
        self.deny_local_log_on = Some(value);
    }

    /// Gets the value of DenyLocalLogOn
    pub fn get_deny_local_log_on(&self) -> Option<&String> {
        self.deny_local_log_on.as_ref()
    }

    /// Sets the value of DenyRemoteDesktopServicesLogOn
    pub fn set_deny_remote_desktop_services_log_on(&mut self, value: String) {
        self.deny_remote_desktop_services_log_on = Some(value);
    }

    /// Gets the value of DenyRemoteDesktopServicesLogOn
    pub fn get_deny_remote_desktop_services_log_on(&self) -> Option<&String> {
        self.deny_remote_desktop_services_log_on.as_ref()
    }

    /// Sets the value of EnableDelegation
    pub fn set_enable_delegation(&mut self, value: String) {
        self.enable_delegation = Some(value);
    }

    /// Gets the value of EnableDelegation
    pub fn get_enable_delegation(&self) -> Option<&String> {
        self.enable_delegation.as_ref()
    }

    /// Sets the value of GenerateSecurityAudits
    pub fn set_generate_security_audits(&mut self, value: String) {
        self.generate_security_audits = Some(value);
    }

    /// Gets the value of GenerateSecurityAudits
    pub fn get_generate_security_audits(&self) -> Option<&String> {
        self.generate_security_audits.as_ref()
    }

    /// Sets the value of ImpersonateClient
    pub fn set_impersonate_client(&mut self, value: String) {
        self.impersonate_client = Some(value);
    }

    /// Gets the value of ImpersonateClient
    pub fn get_impersonate_client(&self) -> Option<&String> {
        self.impersonate_client.as_ref()
    }

    /// Sets the value of IncreaseSchedulingPriority
    pub fn set_increase_scheduling_priority(&mut self, value: String) {
        self.increase_scheduling_priority = Some(value);
    }

    /// Gets the value of IncreaseSchedulingPriority
    pub fn get_increase_scheduling_priority(&self) -> Option<&String> {
        self.increase_scheduling_priority.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LoadUnloadDeviceDrivers
    pub fn set_load_unload_device_drivers(&mut self, value: String) {
        self.load_unload_device_drivers = Some(value);
    }

    /// Gets the value of LoadUnloadDeviceDrivers
    pub fn get_load_unload_device_drivers(&self) -> Option<&String> {
        self.load_unload_device_drivers.as_ref()
    }

    /// Sets the value of LockMemory
    pub fn set_lock_memory(&mut self, value: String) {
        self.lock_memory = Some(value);
    }

    /// Gets the value of LockMemory
    pub fn get_lock_memory(&self) -> Option<&String> {
        self.lock_memory.as_ref()
    }

    /// Sets the value of ManageAuditingAndSecurityLog
    pub fn set_manage_auditing_and_security_log(&mut self, value: String) {
        self.manage_auditing_and_security_log = Some(value);
    }

    /// Gets the value of ManageAuditingAndSecurityLog
    pub fn get_manage_auditing_and_security_log(&self) -> Option<&String> {
        self.manage_auditing_and_security_log.as_ref()
    }

    /// Sets the value of ManageVolume
    pub fn set_manage_volume(&mut self, value: String) {
        self.manage_volume = Some(value);
    }

    /// Gets the value of ManageVolume
    pub fn get_manage_volume(&self) -> Option<&String> {
        self.manage_volume.as_ref()
    }

    /// Sets the value of ModifyFirmwareEnvironment
    pub fn set_modify_firmware_environment(&mut self, value: String) {
        self.modify_firmware_environment = Some(value);
    }

    /// Gets the value of ModifyFirmwareEnvironment
    pub fn get_modify_firmware_environment(&self) -> Option<&String> {
        self.modify_firmware_environment.as_ref()
    }

    /// Sets the value of ModifyObjectLabel
    pub fn set_modify_object_label(&mut self, value: String) {
        self.modify_object_label = Some(value);
    }

    /// Gets the value of ModifyObjectLabel
    pub fn get_modify_object_label(&self) -> Option<&String> {
        self.modify_object_label.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ProfileSingleProcess
    pub fn set_profile_single_process(&mut self, value: String) {
        self.profile_single_process = Some(value);
    }

    /// Gets the value of ProfileSingleProcess
    pub fn get_profile_single_process(&self) -> Option<&String> {
        self.profile_single_process.as_ref()
    }

    /// Sets the value of RemoteShutdown
    pub fn set_remote_shutdown(&mut self, value: String) {
        self.remote_shutdown = Some(value);
    }

    /// Gets the value of RemoteShutdown
    pub fn get_remote_shutdown(&self) -> Option<&String> {
        self.remote_shutdown.as_ref()
    }

    /// Sets the value of RestoreFilesAndDirectories
    pub fn set_restore_files_and_directories(&mut self, value: String) {
        self.restore_files_and_directories = Some(value);
    }

    /// Gets the value of RestoreFilesAndDirectories
    pub fn get_restore_files_and_directories(&self) -> Option<&String> {
        self.restore_files_and_directories.as_ref()
    }

    /// Sets the value of TakeOwnership
    pub fn set_take_ownership(&mut self, value: String) {
        self.take_ownership = Some(value);
    }

    /// Gets the value of TakeOwnership
    pub fn get_take_ownership(&self) -> Option<&String> {
        self.take_ownership.as_ref()
    }
}

