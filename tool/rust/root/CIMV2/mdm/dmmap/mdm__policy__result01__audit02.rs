// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Audit02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Audit02 {

/// 
    #[serde(rename = "AccountLogon_AuditCredentialValidation")]
    pub account_logon__audit_credential_validation: Option<i32>,

/// 
    #[serde(rename = "AccountLogon_AuditKerberosAuthenticationService")]
    pub account_logon__audit_kerberos_authentication_service: Option<i32>,

/// 
    #[serde(rename = "AccountLogon_AuditKerberosServiceTicketOperations")]
    pub account_logon__audit_kerberos_service_ticket_operations: Option<i32>,

/// 
    #[serde(rename = "AccountLogon_AuditOtherAccountLogonEvents")]
    pub account_logon__audit_other_account_logon_events: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditAccountLockout")]
    pub account_logon_logoff__audit_account_lockout: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditGroupMembership")]
    pub account_logon_logoff__audit_group_membership: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditIPsecExtendedMode")]
    pub account_logon_logoff__audit_ipsec_extended_mode: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditIPsecMainMode")]
    pub account_logon_logoff__audit_ipsec_main_mode: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditIPsecQuickMode")]
    pub account_logon_logoff__audit_ipsec_quick_mode: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditLogoff")]
    pub account_logon_logoff__audit_logoff: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditLogon")]
    pub account_logon_logoff__audit_logon: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditNetworkPolicyServer")]
    pub account_logon_logoff__audit_network_policy_server: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditOtherLogonLogoffEvents")]
    pub account_logon_logoff__audit_other_logon_logoff_events: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditSpecialLogon")]
    pub account_logon_logoff__audit_special_logon: Option<i32>,

/// 
    #[serde(rename = "AccountLogonLogoff_AuditUserDeviceClaims")]
    pub account_logon_logoff__audit_user_device_claims: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditApplicationGroupManagement")]
    pub account_management__audit_application_group_management: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditComputerAccountManagement")]
    pub account_management__audit_computer_account_management: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditDistributionGroupManagement")]
    pub account_management__audit_distribution_group_management: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditOtherAccountManagementEvents")]
    pub account_management__audit_other_account_management_events: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditSecurityGroupManagement")]
    pub account_management__audit_security_group_management: Option<i32>,

/// 
    #[serde(rename = "AccountManagement_AuditUserAccountManagement")]
    pub account_management__audit_user_account_management: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditDPAPIActivity")]
    pub detailed_tracking__audit_dpapiactivity: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditPNPActivity")]
    pub detailed_tracking__audit_pnpactivity: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditProcessCreation")]
    pub detailed_tracking__audit_process_creation: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditProcessTermination")]
    pub detailed_tracking__audit_process_termination: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditRPCEvents")]
    pub detailed_tracking__audit_rpcevents: Option<i32>,

/// 
    #[serde(rename = "DetailedTracking_AuditTokenRightAdjusted")]
    pub detailed_tracking__audit_token_right_adjusted: Option<i32>,

/// 
    #[serde(rename = "DSAccess_AuditDetailedDirectoryServiceReplication")]
    pub dsaccess__audit_detailed_directory_service_replication: Option<i32>,

/// 
    #[serde(rename = "DSAccess_AuditDirectoryServiceAccess")]
    pub dsaccess__audit_directory_service_access: Option<i32>,

/// 
    #[serde(rename = "DSAccess_AuditDirectoryServiceChanges")]
    pub dsaccess__audit_directory_service_changes: Option<i32>,

/// 
    #[serde(rename = "DSAccess_AuditDirectoryServiceReplication")]
    pub dsaccess__audit_directory_service_replication: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ObjectAccess_AuditApplicationGenerated")]
    pub object_access__audit_application_generated: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditCentralAccessPolicyStaging")]
    pub object_access__audit_central_access_policy_staging: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditCertificationServices")]
    pub object_access__audit_certification_services: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditDetailedFileShare")]
    pub object_access__audit_detailed_file_share: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditFileShare")]
    pub object_access__audit_file_share: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditFileSystem")]
    pub object_access__audit_file_system: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditFilteringPlatformConnection")]
    pub object_access__audit_filtering_platform_connection: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditFilteringPlatformPacketDrop")]
    pub object_access__audit_filtering_platform_packet_drop: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditHandleManipulation")]
    pub object_access__audit_handle_manipulation: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditKernelObject")]
    pub object_access__audit_kernel_object: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditOtherObjectAccessEvents")]
    pub object_access__audit_other_object_access_events: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditRegistry")]
    pub object_access__audit_registry: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditRemovableStorage")]
    pub object_access__audit_removable_storage: Option<i32>,

/// 
    #[serde(rename = "ObjectAccess_AuditSAM")]
    pub object_access__audit_sam: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PolicyChange_AuditAuthenticationPolicyChange")]
    pub policy_change__audit_authentication_policy_change: Option<i32>,

/// 
    #[serde(rename = "PolicyChange_AuditAuthorizationPolicyChange")]
    pub policy_change__audit_authorization_policy_change: Option<i32>,

/// 
    #[serde(rename = "PolicyChange_AuditFilteringPlatformPolicyChange")]
    pub policy_change__audit_filtering_platform_policy_change: Option<i32>,

/// 
    #[serde(rename = "PolicyChange_AuditMPSSVCRuleLevelPolicyChange")]
    pub policy_change__audit_mpssvcrule_level_policy_change: Option<i32>,

/// 
    #[serde(rename = "PolicyChange_AuditOtherPolicyChangeEvents")]
    pub policy_change__audit_other_policy_change_events: Option<i32>,

/// 
    #[serde(rename = "PolicyChange_AuditPolicyChange")]
    pub policy_change__audit_policy_change: Option<i32>,

/// 
    #[serde(rename = "PrivilegeUse_AuditNonSensitivePrivilegeUse")]
    pub privilege_use__audit_non_sensitive_privilege_use: Option<i32>,

/// 
    #[serde(rename = "PrivilegeUse_AuditOtherPrivilegeUseEvents")]
    pub privilege_use__audit_other_privilege_use_events: Option<i32>,

/// 
    #[serde(rename = "PrivilegeUse_AuditSensitivePrivilegeUse")]
    pub privilege_use__audit_sensitive_privilege_use: Option<i32>,

/// 
    #[serde(rename = "System_AuditIPsecDriver")]
    pub system__audit_ipsec_driver: Option<i32>,

/// 
    #[serde(rename = "System_AuditOtherSystemEvents")]
    pub system__audit_other_system_events: Option<i32>,

/// 
    #[serde(rename = "System_AuditSecurityStateChange")]
    pub system__audit_security_state_change: Option<i32>,

/// 
    #[serde(rename = "System_AuditSecuritySystemExtension")]
    pub system__audit_security_system_extension: Option<i32>,

/// 
    #[serde(rename = "System_AuditSystemIntegrity")]
    pub system__audit_system_integrity: Option<i32>,
}

impl MDM_Policy_Result01_Audit02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            account_logon__audit_credential_validation: None,
            account_logon__audit_kerberos_authentication_service: None,
            account_logon__audit_kerberos_service_ticket_operations: None,
            account_logon__audit_other_account_logon_events: None,
            account_logon_logoff__audit_account_lockout: None,
            account_logon_logoff__audit_group_membership: None,
            account_logon_logoff__audit_ipsec_extended_mode: None,
            account_logon_logoff__audit_ipsec_main_mode: None,
            account_logon_logoff__audit_ipsec_quick_mode: None,
            account_logon_logoff__audit_logoff: None,
            account_logon_logoff__audit_logon: None,
            account_logon_logoff__audit_network_policy_server: None,
            account_logon_logoff__audit_other_logon_logoff_events: None,
            account_logon_logoff__audit_special_logon: None,
            account_logon_logoff__audit_user_device_claims: None,
            account_management__audit_application_group_management: None,
            account_management__audit_computer_account_management: None,
            account_management__audit_distribution_group_management: None,
            account_management__audit_other_account_management_events: None,
            account_management__audit_security_group_management: None,
            account_management__audit_user_account_management: None,
            detailed_tracking__audit_dpapiactivity: None,
            detailed_tracking__audit_pnpactivity: None,
            detailed_tracking__audit_process_creation: None,
            detailed_tracking__audit_process_termination: None,
            detailed_tracking__audit_rpcevents: None,
            detailed_tracking__audit_token_right_adjusted: None,
            dsaccess__audit_detailed_directory_service_replication: None,
            dsaccess__audit_directory_service_access: None,
            dsaccess__audit_directory_service_changes: None,
            dsaccess__audit_directory_service_replication: None,
            instance_id: None,
            object_access__audit_application_generated: None,
            object_access__audit_central_access_policy_staging: None,
            object_access__audit_certification_services: None,
            object_access__audit_detailed_file_share: None,
            object_access__audit_file_share: None,
            object_access__audit_file_system: None,
            object_access__audit_filtering_platform_connection: None,
            object_access__audit_filtering_platform_packet_drop: None,
            object_access__audit_handle_manipulation: None,
            object_access__audit_kernel_object: None,
            object_access__audit_other_object_access_events: None,
            object_access__audit_registry: None,
            object_access__audit_removable_storage: None,
            object_access__audit_sam: None,
            parent_id: None,
            policy_change__audit_authentication_policy_change: None,
            policy_change__audit_authorization_policy_change: None,
            policy_change__audit_filtering_platform_policy_change: None,
            policy_change__audit_mpssvcrule_level_policy_change: None,
            policy_change__audit_other_policy_change_events: None,
            policy_change__audit_policy_change: None,
            privilege_use__audit_non_sensitive_privilege_use: None,
            privilege_use__audit_other_privilege_use_events: None,
            privilege_use__audit_sensitive_privilege_use: None,
            system__audit_ipsec_driver: None,
            system__audit_other_system_events: None,
            system__audit_security_state_change: None,
            system__audit_security_system_extension: None,
            system__audit_system_integrity: None,
        }
    }


    /// Sets the value of AccountLogon_AuditCredentialValidation
    pub fn set_account_logon__audit_credential_validation(&mut self, value: i32) {
        self.account_logon__audit_credential_validation = Some(value);
    }

    /// Gets the value of AccountLogon_AuditCredentialValidation
    pub fn get_account_logon__audit_credential_validation(&self) -> Option<&i32> {
        self.account_logon__audit_credential_validation.as_ref()
    }

    /// Sets the value of AccountLogon_AuditKerberosAuthenticationService
    pub fn set_account_logon__audit_kerberos_authentication_service(&mut self, value: i32) {
        self.account_logon__audit_kerberos_authentication_service = Some(value);
    }

    /// Gets the value of AccountLogon_AuditKerberosAuthenticationService
    pub fn get_account_logon__audit_kerberos_authentication_service(&self) -> Option<&i32> {
        self.account_logon__audit_kerberos_authentication_service.as_ref()
    }

    /// Sets the value of AccountLogon_AuditKerberosServiceTicketOperations
    pub fn set_account_logon__audit_kerberos_service_ticket_operations(&mut self, value: i32) {
        self.account_logon__audit_kerberos_service_ticket_operations = Some(value);
    }

    /// Gets the value of AccountLogon_AuditKerberosServiceTicketOperations
    pub fn get_account_logon__audit_kerberos_service_ticket_operations(&self) -> Option<&i32> {
        self.account_logon__audit_kerberos_service_ticket_operations.as_ref()
    }

    /// Sets the value of AccountLogon_AuditOtherAccountLogonEvents
    pub fn set_account_logon__audit_other_account_logon_events(&mut self, value: i32) {
        self.account_logon__audit_other_account_logon_events = Some(value);
    }

    /// Gets the value of AccountLogon_AuditOtherAccountLogonEvents
    pub fn get_account_logon__audit_other_account_logon_events(&self) -> Option<&i32> {
        self.account_logon__audit_other_account_logon_events.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditAccountLockout
    pub fn set_account_logon_logoff__audit_account_lockout(&mut self, value: i32) {
        self.account_logon_logoff__audit_account_lockout = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditAccountLockout
    pub fn get_account_logon_logoff__audit_account_lockout(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_account_lockout.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditGroupMembership
    pub fn set_account_logon_logoff__audit_group_membership(&mut self, value: i32) {
        self.account_logon_logoff__audit_group_membership = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditGroupMembership
    pub fn get_account_logon_logoff__audit_group_membership(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_group_membership.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditIPsecExtendedMode
    pub fn set_account_logon_logoff__audit_ipsec_extended_mode(&mut self, value: i32) {
        self.account_logon_logoff__audit_ipsec_extended_mode = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditIPsecExtendedMode
    pub fn get_account_logon_logoff__audit_ipsec_extended_mode(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_ipsec_extended_mode.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditIPsecMainMode
    pub fn set_account_logon_logoff__audit_ipsec_main_mode(&mut self, value: i32) {
        self.account_logon_logoff__audit_ipsec_main_mode = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditIPsecMainMode
    pub fn get_account_logon_logoff__audit_ipsec_main_mode(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_ipsec_main_mode.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditIPsecQuickMode
    pub fn set_account_logon_logoff__audit_ipsec_quick_mode(&mut self, value: i32) {
        self.account_logon_logoff__audit_ipsec_quick_mode = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditIPsecQuickMode
    pub fn get_account_logon_logoff__audit_ipsec_quick_mode(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_ipsec_quick_mode.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditLogoff
    pub fn set_account_logon_logoff__audit_logoff(&mut self, value: i32) {
        self.account_logon_logoff__audit_logoff = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditLogoff
    pub fn get_account_logon_logoff__audit_logoff(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_logoff.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditLogon
    pub fn set_account_logon_logoff__audit_logon(&mut self, value: i32) {
        self.account_logon_logoff__audit_logon = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditLogon
    pub fn get_account_logon_logoff__audit_logon(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_logon.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditNetworkPolicyServer
    pub fn set_account_logon_logoff__audit_network_policy_server(&mut self, value: i32) {
        self.account_logon_logoff__audit_network_policy_server = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditNetworkPolicyServer
    pub fn get_account_logon_logoff__audit_network_policy_server(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_network_policy_server.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditOtherLogonLogoffEvents
    pub fn set_account_logon_logoff__audit_other_logon_logoff_events(&mut self, value: i32) {
        self.account_logon_logoff__audit_other_logon_logoff_events = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditOtherLogonLogoffEvents
    pub fn get_account_logon_logoff__audit_other_logon_logoff_events(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_other_logon_logoff_events.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditSpecialLogon
    pub fn set_account_logon_logoff__audit_special_logon(&mut self, value: i32) {
        self.account_logon_logoff__audit_special_logon = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditSpecialLogon
    pub fn get_account_logon_logoff__audit_special_logon(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_special_logon.as_ref()
    }

    /// Sets the value of AccountLogonLogoff_AuditUserDeviceClaims
    pub fn set_account_logon_logoff__audit_user_device_claims(&mut self, value: i32) {
        self.account_logon_logoff__audit_user_device_claims = Some(value);
    }

    /// Gets the value of AccountLogonLogoff_AuditUserDeviceClaims
    pub fn get_account_logon_logoff__audit_user_device_claims(&self) -> Option<&i32> {
        self.account_logon_logoff__audit_user_device_claims.as_ref()
    }

    /// Sets the value of AccountManagement_AuditApplicationGroupManagement
    pub fn set_account_management__audit_application_group_management(&mut self, value: i32) {
        self.account_management__audit_application_group_management = Some(value);
    }

    /// Gets the value of AccountManagement_AuditApplicationGroupManagement
    pub fn get_account_management__audit_application_group_management(&self) -> Option<&i32> {
        self.account_management__audit_application_group_management.as_ref()
    }

    /// Sets the value of AccountManagement_AuditComputerAccountManagement
    pub fn set_account_management__audit_computer_account_management(&mut self, value: i32) {
        self.account_management__audit_computer_account_management = Some(value);
    }

    /// Gets the value of AccountManagement_AuditComputerAccountManagement
    pub fn get_account_management__audit_computer_account_management(&self) -> Option<&i32> {
        self.account_management__audit_computer_account_management.as_ref()
    }

    /// Sets the value of AccountManagement_AuditDistributionGroupManagement
    pub fn set_account_management__audit_distribution_group_management(&mut self, value: i32) {
        self.account_management__audit_distribution_group_management = Some(value);
    }

    /// Gets the value of AccountManagement_AuditDistributionGroupManagement
    pub fn get_account_management__audit_distribution_group_management(&self) -> Option<&i32> {
        self.account_management__audit_distribution_group_management.as_ref()
    }

    /// Sets the value of AccountManagement_AuditOtherAccountManagementEvents
    pub fn set_account_management__audit_other_account_management_events(&mut self, value: i32) {
        self.account_management__audit_other_account_management_events = Some(value);
    }

    /// Gets the value of AccountManagement_AuditOtherAccountManagementEvents
    pub fn get_account_management__audit_other_account_management_events(&self) -> Option<&i32> {
        self.account_management__audit_other_account_management_events.as_ref()
    }

    /// Sets the value of AccountManagement_AuditSecurityGroupManagement
    pub fn set_account_management__audit_security_group_management(&mut self, value: i32) {
        self.account_management__audit_security_group_management = Some(value);
    }

    /// Gets the value of AccountManagement_AuditSecurityGroupManagement
    pub fn get_account_management__audit_security_group_management(&self) -> Option<&i32> {
        self.account_management__audit_security_group_management.as_ref()
    }

    /// Sets the value of AccountManagement_AuditUserAccountManagement
    pub fn set_account_management__audit_user_account_management(&mut self, value: i32) {
        self.account_management__audit_user_account_management = Some(value);
    }

    /// Gets the value of AccountManagement_AuditUserAccountManagement
    pub fn get_account_management__audit_user_account_management(&self) -> Option<&i32> {
        self.account_management__audit_user_account_management.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditDPAPIActivity
    pub fn set_detailed_tracking__audit_dpapiactivity(&mut self, value: i32) {
        self.detailed_tracking__audit_dpapiactivity = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditDPAPIActivity
    pub fn get_detailed_tracking__audit_dpapiactivity(&self) -> Option<&i32> {
        self.detailed_tracking__audit_dpapiactivity.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditPNPActivity
    pub fn set_detailed_tracking__audit_pnpactivity(&mut self, value: i32) {
        self.detailed_tracking__audit_pnpactivity = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditPNPActivity
    pub fn get_detailed_tracking__audit_pnpactivity(&self) -> Option<&i32> {
        self.detailed_tracking__audit_pnpactivity.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditProcessCreation
    pub fn set_detailed_tracking__audit_process_creation(&mut self, value: i32) {
        self.detailed_tracking__audit_process_creation = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditProcessCreation
    pub fn get_detailed_tracking__audit_process_creation(&self) -> Option<&i32> {
        self.detailed_tracking__audit_process_creation.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditProcessTermination
    pub fn set_detailed_tracking__audit_process_termination(&mut self, value: i32) {
        self.detailed_tracking__audit_process_termination = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditProcessTermination
    pub fn get_detailed_tracking__audit_process_termination(&self) -> Option<&i32> {
        self.detailed_tracking__audit_process_termination.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditRPCEvents
    pub fn set_detailed_tracking__audit_rpcevents(&mut self, value: i32) {
        self.detailed_tracking__audit_rpcevents = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditRPCEvents
    pub fn get_detailed_tracking__audit_rpcevents(&self) -> Option<&i32> {
        self.detailed_tracking__audit_rpcevents.as_ref()
    }

    /// Sets the value of DetailedTracking_AuditTokenRightAdjusted
    pub fn set_detailed_tracking__audit_token_right_adjusted(&mut self, value: i32) {
        self.detailed_tracking__audit_token_right_adjusted = Some(value);
    }

    /// Gets the value of DetailedTracking_AuditTokenRightAdjusted
    pub fn get_detailed_tracking__audit_token_right_adjusted(&self) -> Option<&i32> {
        self.detailed_tracking__audit_token_right_adjusted.as_ref()
    }

    /// Sets the value of DSAccess_AuditDetailedDirectoryServiceReplication
    pub fn set_dsaccess__audit_detailed_directory_service_replication(&mut self, value: i32) {
        self.dsaccess__audit_detailed_directory_service_replication = Some(value);
    }

    /// Gets the value of DSAccess_AuditDetailedDirectoryServiceReplication
    pub fn get_dsaccess__audit_detailed_directory_service_replication(&self) -> Option<&i32> {
        self.dsaccess__audit_detailed_directory_service_replication.as_ref()
    }

    /// Sets the value of DSAccess_AuditDirectoryServiceAccess
    pub fn set_dsaccess__audit_directory_service_access(&mut self, value: i32) {
        self.dsaccess__audit_directory_service_access = Some(value);
    }

    /// Gets the value of DSAccess_AuditDirectoryServiceAccess
    pub fn get_dsaccess__audit_directory_service_access(&self) -> Option<&i32> {
        self.dsaccess__audit_directory_service_access.as_ref()
    }

    /// Sets the value of DSAccess_AuditDirectoryServiceChanges
    pub fn set_dsaccess__audit_directory_service_changes(&mut self, value: i32) {
        self.dsaccess__audit_directory_service_changes = Some(value);
    }

    /// Gets the value of DSAccess_AuditDirectoryServiceChanges
    pub fn get_dsaccess__audit_directory_service_changes(&self) -> Option<&i32> {
        self.dsaccess__audit_directory_service_changes.as_ref()
    }

    /// Sets the value of DSAccess_AuditDirectoryServiceReplication
    pub fn set_dsaccess__audit_directory_service_replication(&mut self, value: i32) {
        self.dsaccess__audit_directory_service_replication = Some(value);
    }

    /// Gets the value of DSAccess_AuditDirectoryServiceReplication
    pub fn get_dsaccess__audit_directory_service_replication(&self) -> Option<&i32> {
        self.dsaccess__audit_directory_service_replication.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditApplicationGenerated
    pub fn set_object_access__audit_application_generated(&mut self, value: i32) {
        self.object_access__audit_application_generated = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditApplicationGenerated
    pub fn get_object_access__audit_application_generated(&self) -> Option<&i32> {
        self.object_access__audit_application_generated.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditCentralAccessPolicyStaging
    pub fn set_object_access__audit_central_access_policy_staging(&mut self, value: i32) {
        self.object_access__audit_central_access_policy_staging = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditCentralAccessPolicyStaging
    pub fn get_object_access__audit_central_access_policy_staging(&self) -> Option<&i32> {
        self.object_access__audit_central_access_policy_staging.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditCertificationServices
    pub fn set_object_access__audit_certification_services(&mut self, value: i32) {
        self.object_access__audit_certification_services = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditCertificationServices
    pub fn get_object_access__audit_certification_services(&self) -> Option<&i32> {
        self.object_access__audit_certification_services.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditDetailedFileShare
    pub fn set_object_access__audit_detailed_file_share(&mut self, value: i32) {
        self.object_access__audit_detailed_file_share = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditDetailedFileShare
    pub fn get_object_access__audit_detailed_file_share(&self) -> Option<&i32> {
        self.object_access__audit_detailed_file_share.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditFileShare
    pub fn set_object_access__audit_file_share(&mut self, value: i32) {
        self.object_access__audit_file_share = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditFileShare
    pub fn get_object_access__audit_file_share(&self) -> Option<&i32> {
        self.object_access__audit_file_share.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditFileSystem
    pub fn set_object_access__audit_file_system(&mut self, value: i32) {
        self.object_access__audit_file_system = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditFileSystem
    pub fn get_object_access__audit_file_system(&self) -> Option<&i32> {
        self.object_access__audit_file_system.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditFilteringPlatformConnection
    pub fn set_object_access__audit_filtering_platform_connection(&mut self, value: i32) {
        self.object_access__audit_filtering_platform_connection = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditFilteringPlatformConnection
    pub fn get_object_access__audit_filtering_platform_connection(&self) -> Option<&i32> {
        self.object_access__audit_filtering_platform_connection.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditFilteringPlatformPacketDrop
    pub fn set_object_access__audit_filtering_platform_packet_drop(&mut self, value: i32) {
        self.object_access__audit_filtering_platform_packet_drop = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditFilteringPlatformPacketDrop
    pub fn get_object_access__audit_filtering_platform_packet_drop(&self) -> Option<&i32> {
        self.object_access__audit_filtering_platform_packet_drop.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditHandleManipulation
    pub fn set_object_access__audit_handle_manipulation(&mut self, value: i32) {
        self.object_access__audit_handle_manipulation = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditHandleManipulation
    pub fn get_object_access__audit_handle_manipulation(&self) -> Option<&i32> {
        self.object_access__audit_handle_manipulation.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditKernelObject
    pub fn set_object_access__audit_kernel_object(&mut self, value: i32) {
        self.object_access__audit_kernel_object = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditKernelObject
    pub fn get_object_access__audit_kernel_object(&self) -> Option<&i32> {
        self.object_access__audit_kernel_object.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditOtherObjectAccessEvents
    pub fn set_object_access__audit_other_object_access_events(&mut self, value: i32) {
        self.object_access__audit_other_object_access_events = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditOtherObjectAccessEvents
    pub fn get_object_access__audit_other_object_access_events(&self) -> Option<&i32> {
        self.object_access__audit_other_object_access_events.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditRegistry
    pub fn set_object_access__audit_registry(&mut self, value: i32) {
        self.object_access__audit_registry = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditRegistry
    pub fn get_object_access__audit_registry(&self) -> Option<&i32> {
        self.object_access__audit_registry.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditRemovableStorage
    pub fn set_object_access__audit_removable_storage(&mut self, value: i32) {
        self.object_access__audit_removable_storage = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditRemovableStorage
    pub fn get_object_access__audit_removable_storage(&self) -> Option<&i32> {
        self.object_access__audit_removable_storage.as_ref()
    }

    /// Sets the value of ObjectAccess_AuditSAM
    pub fn set_object_access__audit_sam(&mut self, value: i32) {
        self.object_access__audit_sam = Some(value);
    }

    /// Gets the value of ObjectAccess_AuditSAM
    pub fn get_object_access__audit_sam(&self) -> Option<&i32> {
        self.object_access__audit_sam.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PolicyChange_AuditAuthenticationPolicyChange
    pub fn set_policy_change__audit_authentication_policy_change(&mut self, value: i32) {
        self.policy_change__audit_authentication_policy_change = Some(value);
    }

    /// Gets the value of PolicyChange_AuditAuthenticationPolicyChange
    pub fn get_policy_change__audit_authentication_policy_change(&self) -> Option<&i32> {
        self.policy_change__audit_authentication_policy_change.as_ref()
    }

    /// Sets the value of PolicyChange_AuditAuthorizationPolicyChange
    pub fn set_policy_change__audit_authorization_policy_change(&mut self, value: i32) {
        self.policy_change__audit_authorization_policy_change = Some(value);
    }

    /// Gets the value of PolicyChange_AuditAuthorizationPolicyChange
    pub fn get_policy_change__audit_authorization_policy_change(&self) -> Option<&i32> {
        self.policy_change__audit_authorization_policy_change.as_ref()
    }

    /// Sets the value of PolicyChange_AuditFilteringPlatformPolicyChange
    pub fn set_policy_change__audit_filtering_platform_policy_change(&mut self, value: i32) {
        self.policy_change__audit_filtering_platform_policy_change = Some(value);
    }

    /// Gets the value of PolicyChange_AuditFilteringPlatformPolicyChange
    pub fn get_policy_change__audit_filtering_platform_policy_change(&self) -> Option<&i32> {
        self.policy_change__audit_filtering_platform_policy_change.as_ref()
    }

    /// Sets the value of PolicyChange_AuditMPSSVCRuleLevelPolicyChange
    pub fn set_policy_change__audit_mpssvcrule_level_policy_change(&mut self, value: i32) {
        self.policy_change__audit_mpssvcrule_level_policy_change = Some(value);
    }

    /// Gets the value of PolicyChange_AuditMPSSVCRuleLevelPolicyChange
    pub fn get_policy_change__audit_mpssvcrule_level_policy_change(&self) -> Option<&i32> {
        self.policy_change__audit_mpssvcrule_level_policy_change.as_ref()
    }

    /// Sets the value of PolicyChange_AuditOtherPolicyChangeEvents
    pub fn set_policy_change__audit_other_policy_change_events(&mut self, value: i32) {
        self.policy_change__audit_other_policy_change_events = Some(value);
    }

    /// Gets the value of PolicyChange_AuditOtherPolicyChangeEvents
    pub fn get_policy_change__audit_other_policy_change_events(&self) -> Option<&i32> {
        self.policy_change__audit_other_policy_change_events.as_ref()
    }

    /// Sets the value of PolicyChange_AuditPolicyChange
    pub fn set_policy_change__audit_policy_change(&mut self, value: i32) {
        self.policy_change__audit_policy_change = Some(value);
    }

    /// Gets the value of PolicyChange_AuditPolicyChange
    pub fn get_policy_change__audit_policy_change(&self) -> Option<&i32> {
        self.policy_change__audit_policy_change.as_ref()
    }

    /// Sets the value of PrivilegeUse_AuditNonSensitivePrivilegeUse
    pub fn set_privilege_use__audit_non_sensitive_privilege_use(&mut self, value: i32) {
        self.privilege_use__audit_non_sensitive_privilege_use = Some(value);
    }

    /// Gets the value of PrivilegeUse_AuditNonSensitivePrivilegeUse
    pub fn get_privilege_use__audit_non_sensitive_privilege_use(&self) -> Option<&i32> {
        self.privilege_use__audit_non_sensitive_privilege_use.as_ref()
    }

    /// Sets the value of PrivilegeUse_AuditOtherPrivilegeUseEvents
    pub fn set_privilege_use__audit_other_privilege_use_events(&mut self, value: i32) {
        self.privilege_use__audit_other_privilege_use_events = Some(value);
    }

    /// Gets the value of PrivilegeUse_AuditOtherPrivilegeUseEvents
    pub fn get_privilege_use__audit_other_privilege_use_events(&self) -> Option<&i32> {
        self.privilege_use__audit_other_privilege_use_events.as_ref()
    }

    /// Sets the value of PrivilegeUse_AuditSensitivePrivilegeUse
    pub fn set_privilege_use__audit_sensitive_privilege_use(&mut self, value: i32) {
        self.privilege_use__audit_sensitive_privilege_use = Some(value);
    }

    /// Gets the value of PrivilegeUse_AuditSensitivePrivilegeUse
    pub fn get_privilege_use__audit_sensitive_privilege_use(&self) -> Option<&i32> {
        self.privilege_use__audit_sensitive_privilege_use.as_ref()
    }

    /// Sets the value of System_AuditIPsecDriver
    pub fn set_system__audit_ipsec_driver(&mut self, value: i32) {
        self.system__audit_ipsec_driver = Some(value);
    }

    /// Gets the value of System_AuditIPsecDriver
    pub fn get_system__audit_ipsec_driver(&self) -> Option<&i32> {
        self.system__audit_ipsec_driver.as_ref()
    }

    /// Sets the value of System_AuditOtherSystemEvents
    pub fn set_system__audit_other_system_events(&mut self, value: i32) {
        self.system__audit_other_system_events = Some(value);
    }

    /// Gets the value of System_AuditOtherSystemEvents
    pub fn get_system__audit_other_system_events(&self) -> Option<&i32> {
        self.system__audit_other_system_events.as_ref()
    }

    /// Sets the value of System_AuditSecurityStateChange
    pub fn set_system__audit_security_state_change(&mut self, value: i32) {
        self.system__audit_security_state_change = Some(value);
    }

    /// Gets the value of System_AuditSecurityStateChange
    pub fn get_system__audit_security_state_change(&self) -> Option<&i32> {
        self.system__audit_security_state_change.as_ref()
    }

    /// Sets the value of System_AuditSecuritySystemExtension
    pub fn set_system__audit_security_system_extension(&mut self, value: i32) {
        self.system__audit_security_system_extension = Some(value);
    }

    /// Gets the value of System_AuditSecuritySystemExtension
    pub fn get_system__audit_security_system_extension(&self) -> Option<&i32> {
        self.system__audit_security_system_extension.as_ref()
    }

    /// Sets the value of System_AuditSystemIntegrity
    pub fn set_system__audit_system_integrity(&mut self, value: i32) {
        self.system__audit_system_integrity = Some(value);
    }

    /// Gets the value of System_AuditSystemIntegrity
    pub fn get_system__audit_system_integrity(&self) -> Option<&i32> {
        self.system__audit_system_integrity.as_ref()
    }
}

