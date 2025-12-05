// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_LocalPoliciesSecurityOptions02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_LocalPoliciesSecurityOptions02 {

/// 
    #[serde(rename = "Accounts_BlockMicrosoftAccounts")]
    pub accounts__block_microsoft_accounts: Option<i32>,

/// 
    #[serde(rename = "Accounts_EnableAdministratorAccountStatus")]
    pub accounts__enable_administrator_account_status: Option<i32>,

/// 
    #[serde(rename = "Accounts_EnableGuestAccountStatus")]
    pub accounts__enable_guest_account_status: Option<i32>,

/// 
    #[serde(rename = "Accounts_LimitLocalAccountUseOfBlankPasswordsToConsoleLogonOnly")]
    pub accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only: Option<i32>,

/// 
    #[serde(rename = "Accounts_RenameAdministratorAccount")]
    pub accounts__rename_administrator_account: Option<String>,

/// 
    #[serde(rename = "Accounts_RenameGuestAccount")]
    pub accounts__rename_guest_account: Option<String>,

/// 
    #[serde(rename = "Devices_AllowedToFormatAndEjectRemovableMedia")]
    pub devices__allowed_to_format_and_eject_removable_media: Option<String>,

/// 
    #[serde(rename = "Devices_AllowUndockWithoutHavingToLogon")]
    pub devices__allow_undock_without_having_to_logon: Option<i32>,

/// 
    #[serde(rename = "Devices_PreventUsersFromInstallingPrinterDriversWhenConnectingToSharedPrinters")]
    pub devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers: Option<i32>,

/// 
    #[serde(rename = "Devices_RestrictCDROMAccessToLocallyLoggedOnUserOnly")]
    pub devices__restrict_cdromaccess_to_locally_logged_on_user_only: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "InteractiveLogon_DisplayUserInformationWhenTheSessionIsLocked")]
    pub interactive_logon__display_user_information_when_the_session_is_locked: Option<i32>,

/// 
    #[serde(rename = "InteractiveLogon_DoNotDisplayLastSignedIn")]
    pub interactive_logon__do_not_display_last_signed_in: Option<i32>,

/// 
    #[serde(rename = "InteractiveLogon_DoNotDisplayUsernameAtSignIn")]
    pub interactive_logon__do_not_display_username_at_sign_in: Option<i32>,

/// 
    #[serde(rename = "InteractiveLogon_DoNotRequireCTRLALTDEL")]
    pub interactive_logon__do_not_require_ctrlaltdel: Option<i32>,

/// 
    #[serde(rename = "InteractiveLogon_MachineInactivityLimit")]
    pub interactive_logon__machine_inactivity_limit: Option<i32>,

/// 
    #[serde(rename = "InteractiveLogon_MessageTextForUsersAttemptingToLogOn")]
    pub interactive_logon__message_text_for_users_attempting_to_log_on: Option<String>,

/// 
    #[serde(rename = "InteractiveLogon_MessageTitleForUsersAttemptingToLogOn")]
    pub interactive_logon__message_title_for_users_attempting_to_log_on: Option<String>,

/// 
    #[serde(rename = "InteractiveLogon_SmartCardRemovalBehavior")]
    pub interactive_logon__smart_card_removal_behavior: Option<String>,

/// 
    #[serde(rename = "MicrosoftNetworkClient_DigitallySignCommunicationsAlways")]
    pub microsoft_network_client__digitally_sign_communications_always: Option<i32>,

/// 
    #[serde(rename = "MicrosoftNetworkClient_DigitallySignCommunicationsIfServerAgrees")]
    pub microsoft_network_client__digitally_sign_communications_if_server_agrees: Option<i32>,

/// 
    #[serde(rename = "MicrosoftNetworkClient_SendUnencryptedPasswordToThirdPartySMBServers")]
    pub microsoft_network_client__send_unencrypted_password_to_third_party_smbservers: Option<i32>,

/// 
    #[serde(rename = "MicrosoftNetworkServer_DigitallySignCommunicationsAlways")]
    pub microsoft_network_server__digitally_sign_communications_always: Option<i32>,

/// 
    #[serde(rename = "MicrosoftNetworkServer_DigitallySignCommunicationsIfClientAgrees")]
    pub microsoft_network_server__digitally_sign_communications_if_client_agrees: Option<i32>,

/// 
    #[serde(rename = "NetworkAccess_DoNotAllowAnonymousEnumerationOfSAMAccounts")]
    pub network_access__do_not_allow_anonymous_enumeration_of_samaccounts: Option<i32>,

/// 
    #[serde(rename = "NetworkAccess_DoNotAllowAnonymousEnumerationOfSamAccountsAndShares")]
    pub network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares: Option<i32>,

/// 
    #[serde(rename = "NetworkAccess_RestrictAnonymousAccessToNamedPipesAndShares")]
    pub network_access__restrict_anonymous_access_to_named_pipes_and_shares: Option<i32>,

/// 
    #[serde(rename = "NetworkAccess_RestrictClientsAllowedToMakeRemoteCallsToSAM")]
    pub network_access__restrict_clients_allowed_to_make_remote_calls_to_sam: Option<String>,

/// 
    #[serde(rename = "NetworkSecurity_AllowLocalSystemToUseComputerIdentityForNTLM")]
    pub network_security__allow_local_system_to_use_computer_identity_for_ntlm: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_AllowPKU2UAuthenticationRequests")]
    pub network_security__allow_pku2_uauthentication_requests: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_DoNotStoreLANManagerHashValueOnNextPasswordChange")]
    pub network_security__do_not_store_lanmanager_hash_value_on_next_password_change: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_LANManagerAuthenticationLevel")]
    pub network_security__lanmanager_authentication_level: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedClients")]
    pub network_security__minimum_session_security_for_ntlmsspbased_clients: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedServers")]
    pub network_security__minimum_session_security_for_ntlmsspbased_servers: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_RestrictNTLM_AddRemoteServerExceptionsForNTLMAuthentication")]
    pub network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication: Option<String>,

/// 
    #[serde(rename = "NetworkSecurity_RestrictNTLM_AuditIncomingNTLMTraffic")]
    pub network_security__restrict_ntlm__audit_incoming_ntlmtraffic: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_RestrictNTLM_IncomingNTLMTraffic")]
    pub network_security__restrict_ntlm__incoming_ntlmtraffic: Option<i32>,

/// 
    #[serde(rename = "NetworkSecurity_RestrictNTLM_OutgoingNTLMTrafficToRemoteServers")]
    pub network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Shutdown_AllowSystemToBeShutDownWithoutHavingToLogOn")]
    pub shutdown__allow_system_to_be_shut_down_without_having_to_log_on: Option<i32>,

/// 
    #[serde(rename = "Shutdown_ClearVirtualMemoryPageFile")]
    pub shutdown__clear_virtual_memory_page_file: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_AllowUIAccessApplicationsToPromptForElevation")]
    pub user_account_control__allow_uiaccess_applications_to_prompt_for_elevation: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_BehaviorOfTheElevationPromptForAdministrators")]
    pub user_account_control__behavior_of_the_elevation_prompt_for_administrators: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_BehaviorOfTheElevationPromptForStandardUsers")]
    pub user_account_control__behavior_of_the_elevation_prompt_for_standard_users: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_DetectApplicationInstallationsAndPromptForElevation")]
    pub user_account_control__detect_application_installations_and_prompt_for_elevation: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_OnlyElevateExecutableFilesThatAreSignedAndValidated")]
    pub user_account_control__only_elevate_executable_files_that_are_signed_and_validated: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_OnlyElevateUIAccessApplicationsThatAreInstalledInSecureLocations")]
    pub user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_RunAllAdministratorsInAdminApprovalMode")]
    pub user_account_control__run_all_administrators_in_admin_approval_mode: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_SwitchToTheSecureDesktopWhenPromptingForElevation")]
    pub user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_UseAdminApprovalMode")]
    pub user_account_control__use_admin_approval_mode: Option<i32>,

/// 
    #[serde(rename = "UserAccountControl_VirtualizeFileAndRegistryWriteFailuresToPerUserLocations")]
    pub user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations: Option<i32>,
}

impl MDM_Policy_Result01_LocalPoliciesSecurityOptions02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            accounts__block_microsoft_accounts: None,
            accounts__enable_administrator_account_status: None,
            accounts__enable_guest_account_status: None,
            accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only: None,
            accounts__rename_administrator_account: None,
            accounts__rename_guest_account: None,
            devices__allowed_to_format_and_eject_removable_media: None,
            devices__allow_undock_without_having_to_logon: None,
            devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers: None,
            devices__restrict_cdromaccess_to_locally_logged_on_user_only: None,
            instance_id: None,
            interactive_logon__display_user_information_when_the_session_is_locked: None,
            interactive_logon__do_not_display_last_signed_in: None,
            interactive_logon__do_not_display_username_at_sign_in: None,
            interactive_logon__do_not_require_ctrlaltdel: None,
            interactive_logon__machine_inactivity_limit: None,
            interactive_logon__message_text_for_users_attempting_to_log_on: None,
            interactive_logon__message_title_for_users_attempting_to_log_on: None,
            interactive_logon__smart_card_removal_behavior: None,
            microsoft_network_client__digitally_sign_communications_always: None,
            microsoft_network_client__digitally_sign_communications_if_server_agrees: None,
            microsoft_network_client__send_unencrypted_password_to_third_party_smbservers: None,
            microsoft_network_server__digitally_sign_communications_always: None,
            microsoft_network_server__digitally_sign_communications_if_client_agrees: None,
            network_access__do_not_allow_anonymous_enumeration_of_samaccounts: None,
            network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares: None,
            network_access__restrict_anonymous_access_to_named_pipes_and_shares: None,
            network_access__restrict_clients_allowed_to_make_remote_calls_to_sam: None,
            network_security__allow_local_system_to_use_computer_identity_for_ntlm: None,
            network_security__allow_pku2_uauthentication_requests: None,
            network_security__do_not_store_lanmanager_hash_value_on_next_password_change: None,
            network_security__lanmanager_authentication_level: None,
            network_security__minimum_session_security_for_ntlmsspbased_clients: None,
            network_security__minimum_session_security_for_ntlmsspbased_servers: None,
            network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication: None,
            network_security__restrict_ntlm__audit_incoming_ntlmtraffic: None,
            network_security__restrict_ntlm__incoming_ntlmtraffic: None,
            network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers: None,
            parent_id: None,
            shutdown__allow_system_to_be_shut_down_without_having_to_log_on: None,
            shutdown__clear_virtual_memory_page_file: None,
            user_account_control__allow_uiaccess_applications_to_prompt_for_elevation: None,
            user_account_control__behavior_of_the_elevation_prompt_for_administrators: None,
            user_account_control__behavior_of_the_elevation_prompt_for_standard_users: None,
            user_account_control__detect_application_installations_and_prompt_for_elevation: None,
            user_account_control__only_elevate_executable_files_that_are_signed_and_validated: None,
            user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations: None,
            user_account_control__run_all_administrators_in_admin_approval_mode: None,
            user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation: None,
            user_account_control__use_admin_approval_mode: None,
            user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations: None,
        }
    }


    /// Sets the value of Accounts_BlockMicrosoftAccounts
    pub fn set_accounts__block_microsoft_accounts(&mut self, value: i32) {
        self.accounts__block_microsoft_accounts = Some(value);
    }

    /// Gets the value of Accounts_BlockMicrosoftAccounts
    pub fn get_accounts__block_microsoft_accounts(&self) -> Option<&i32> {
        self.accounts__block_microsoft_accounts.as_ref()
    }

    /// Sets the value of Accounts_EnableAdministratorAccountStatus
    pub fn set_accounts__enable_administrator_account_status(&mut self, value: i32) {
        self.accounts__enable_administrator_account_status = Some(value);
    }

    /// Gets the value of Accounts_EnableAdministratorAccountStatus
    pub fn get_accounts__enable_administrator_account_status(&self) -> Option<&i32> {
        self.accounts__enable_administrator_account_status.as_ref()
    }

    /// Sets the value of Accounts_EnableGuestAccountStatus
    pub fn set_accounts__enable_guest_account_status(&mut self, value: i32) {
        self.accounts__enable_guest_account_status = Some(value);
    }

    /// Gets the value of Accounts_EnableGuestAccountStatus
    pub fn get_accounts__enable_guest_account_status(&self) -> Option<&i32> {
        self.accounts__enable_guest_account_status.as_ref()
    }

    /// Sets the value of Accounts_LimitLocalAccountUseOfBlankPasswordsToConsoleLogonOnly
    pub fn set_accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only(&mut self, value: i32) {
        self.accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only = Some(value);
    }

    /// Gets the value of Accounts_LimitLocalAccountUseOfBlankPasswordsToConsoleLogonOnly
    pub fn get_accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only(&self) -> Option<&i32> {
        self.accounts__limit_local_account_use_of_blank_passwords_to_console_logon_only.as_ref()
    }

    /// Sets the value of Accounts_RenameAdministratorAccount
    pub fn set_accounts__rename_administrator_account(&mut self, value: String) {
        self.accounts__rename_administrator_account = Some(value);
    }

    /// Gets the value of Accounts_RenameAdministratorAccount
    pub fn get_accounts__rename_administrator_account(&self) -> Option<&String> {
        self.accounts__rename_administrator_account.as_ref()
    }

    /// Sets the value of Accounts_RenameGuestAccount
    pub fn set_accounts__rename_guest_account(&mut self, value: String) {
        self.accounts__rename_guest_account = Some(value);
    }

    /// Gets the value of Accounts_RenameGuestAccount
    pub fn get_accounts__rename_guest_account(&self) -> Option<&String> {
        self.accounts__rename_guest_account.as_ref()
    }

    /// Sets the value of Devices_AllowedToFormatAndEjectRemovableMedia
    pub fn set_devices__allowed_to_format_and_eject_removable_media(&mut self, value: String) {
        self.devices__allowed_to_format_and_eject_removable_media = Some(value);
    }

    /// Gets the value of Devices_AllowedToFormatAndEjectRemovableMedia
    pub fn get_devices__allowed_to_format_and_eject_removable_media(&self) -> Option<&String> {
        self.devices__allowed_to_format_and_eject_removable_media.as_ref()
    }

    /// Sets the value of Devices_AllowUndockWithoutHavingToLogon
    pub fn set_devices__allow_undock_without_having_to_logon(&mut self, value: i32) {
        self.devices__allow_undock_without_having_to_logon = Some(value);
    }

    /// Gets the value of Devices_AllowUndockWithoutHavingToLogon
    pub fn get_devices__allow_undock_without_having_to_logon(&self) -> Option<&i32> {
        self.devices__allow_undock_without_having_to_logon.as_ref()
    }

    /// Sets the value of Devices_PreventUsersFromInstallingPrinterDriversWhenConnectingToSharedPrinters
    pub fn set_devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers(&mut self, value: i32) {
        self.devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers = Some(value);
    }

    /// Gets the value of Devices_PreventUsersFromInstallingPrinterDriversWhenConnectingToSharedPrinters
    pub fn get_devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers(&self) -> Option<&i32> {
        self.devices__prevent_users_from_installing_printer_drivers_when_connecting_to_shared_printers.as_ref()
    }

    /// Sets the value of Devices_RestrictCDROMAccessToLocallyLoggedOnUserOnly
    pub fn set_devices__restrict_cdromaccess_to_locally_logged_on_user_only(&mut self, value: String) {
        self.devices__restrict_cdromaccess_to_locally_logged_on_user_only = Some(value);
    }

    /// Gets the value of Devices_RestrictCDROMAccessToLocallyLoggedOnUserOnly
    pub fn get_devices__restrict_cdromaccess_to_locally_logged_on_user_only(&self) -> Option<&String> {
        self.devices__restrict_cdromaccess_to_locally_logged_on_user_only.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of InteractiveLogon_DisplayUserInformationWhenTheSessionIsLocked
    pub fn set_interactive_logon__display_user_information_when_the_session_is_locked(&mut self, value: i32) {
        self.interactive_logon__display_user_information_when_the_session_is_locked = Some(value);
    }

    /// Gets the value of InteractiveLogon_DisplayUserInformationWhenTheSessionIsLocked
    pub fn get_interactive_logon__display_user_information_when_the_session_is_locked(&self) -> Option<&i32> {
        self.interactive_logon__display_user_information_when_the_session_is_locked.as_ref()
    }

    /// Sets the value of InteractiveLogon_DoNotDisplayLastSignedIn
    pub fn set_interactive_logon__do_not_display_last_signed_in(&mut self, value: i32) {
        self.interactive_logon__do_not_display_last_signed_in = Some(value);
    }

    /// Gets the value of InteractiveLogon_DoNotDisplayLastSignedIn
    pub fn get_interactive_logon__do_not_display_last_signed_in(&self) -> Option<&i32> {
        self.interactive_logon__do_not_display_last_signed_in.as_ref()
    }

    /// Sets the value of InteractiveLogon_DoNotDisplayUsernameAtSignIn
    pub fn set_interactive_logon__do_not_display_username_at_sign_in(&mut self, value: i32) {
        self.interactive_logon__do_not_display_username_at_sign_in = Some(value);
    }

    /// Gets the value of InteractiveLogon_DoNotDisplayUsernameAtSignIn
    pub fn get_interactive_logon__do_not_display_username_at_sign_in(&self) -> Option<&i32> {
        self.interactive_logon__do_not_display_username_at_sign_in.as_ref()
    }

    /// Sets the value of InteractiveLogon_DoNotRequireCTRLALTDEL
    pub fn set_interactive_logon__do_not_require_ctrlaltdel(&mut self, value: i32) {
        self.interactive_logon__do_not_require_ctrlaltdel = Some(value);
    }

    /// Gets the value of InteractiveLogon_DoNotRequireCTRLALTDEL
    pub fn get_interactive_logon__do_not_require_ctrlaltdel(&self) -> Option<&i32> {
        self.interactive_logon__do_not_require_ctrlaltdel.as_ref()
    }

    /// Sets the value of InteractiveLogon_MachineInactivityLimit
    pub fn set_interactive_logon__machine_inactivity_limit(&mut self, value: i32) {
        self.interactive_logon__machine_inactivity_limit = Some(value);
    }

    /// Gets the value of InteractiveLogon_MachineInactivityLimit
    pub fn get_interactive_logon__machine_inactivity_limit(&self) -> Option<&i32> {
        self.interactive_logon__machine_inactivity_limit.as_ref()
    }

    /// Sets the value of InteractiveLogon_MessageTextForUsersAttemptingToLogOn
    pub fn set_interactive_logon__message_text_for_users_attempting_to_log_on(&mut self, value: String) {
        self.interactive_logon__message_text_for_users_attempting_to_log_on = Some(value);
    }

    /// Gets the value of InteractiveLogon_MessageTextForUsersAttemptingToLogOn
    pub fn get_interactive_logon__message_text_for_users_attempting_to_log_on(&self) -> Option<&String> {
        self.interactive_logon__message_text_for_users_attempting_to_log_on.as_ref()
    }

    /// Sets the value of InteractiveLogon_MessageTitleForUsersAttemptingToLogOn
    pub fn set_interactive_logon__message_title_for_users_attempting_to_log_on(&mut self, value: String) {
        self.interactive_logon__message_title_for_users_attempting_to_log_on = Some(value);
    }

    /// Gets the value of InteractiveLogon_MessageTitleForUsersAttemptingToLogOn
    pub fn get_interactive_logon__message_title_for_users_attempting_to_log_on(&self) -> Option<&String> {
        self.interactive_logon__message_title_for_users_attempting_to_log_on.as_ref()
    }

    /// Sets the value of InteractiveLogon_SmartCardRemovalBehavior
    pub fn set_interactive_logon__smart_card_removal_behavior(&mut self, value: String) {
        self.interactive_logon__smart_card_removal_behavior = Some(value);
    }

    /// Gets the value of InteractiveLogon_SmartCardRemovalBehavior
    pub fn get_interactive_logon__smart_card_removal_behavior(&self) -> Option<&String> {
        self.interactive_logon__smart_card_removal_behavior.as_ref()
    }

    /// Sets the value of MicrosoftNetworkClient_DigitallySignCommunicationsAlways
    pub fn set_microsoft_network_client__digitally_sign_communications_always(&mut self, value: i32) {
        self.microsoft_network_client__digitally_sign_communications_always = Some(value);
    }

    /// Gets the value of MicrosoftNetworkClient_DigitallySignCommunicationsAlways
    pub fn get_microsoft_network_client__digitally_sign_communications_always(&self) -> Option<&i32> {
        self.microsoft_network_client__digitally_sign_communications_always.as_ref()
    }

    /// Sets the value of MicrosoftNetworkClient_DigitallySignCommunicationsIfServerAgrees
    pub fn set_microsoft_network_client__digitally_sign_communications_if_server_agrees(&mut self, value: i32) {
        self.microsoft_network_client__digitally_sign_communications_if_server_agrees = Some(value);
    }

    /// Gets the value of MicrosoftNetworkClient_DigitallySignCommunicationsIfServerAgrees
    pub fn get_microsoft_network_client__digitally_sign_communications_if_server_agrees(&self) -> Option<&i32> {
        self.microsoft_network_client__digitally_sign_communications_if_server_agrees.as_ref()
    }

    /// Sets the value of MicrosoftNetworkClient_SendUnencryptedPasswordToThirdPartySMBServers
    pub fn set_microsoft_network_client__send_unencrypted_password_to_third_party_smbservers(&mut self, value: i32) {
        self.microsoft_network_client__send_unencrypted_password_to_third_party_smbservers = Some(value);
    }

    /// Gets the value of MicrosoftNetworkClient_SendUnencryptedPasswordToThirdPartySMBServers
    pub fn get_microsoft_network_client__send_unencrypted_password_to_third_party_smbservers(&self) -> Option<&i32> {
        self.microsoft_network_client__send_unencrypted_password_to_third_party_smbservers.as_ref()
    }

    /// Sets the value of MicrosoftNetworkServer_DigitallySignCommunicationsAlways
    pub fn set_microsoft_network_server__digitally_sign_communications_always(&mut self, value: i32) {
        self.microsoft_network_server__digitally_sign_communications_always = Some(value);
    }

    /// Gets the value of MicrosoftNetworkServer_DigitallySignCommunicationsAlways
    pub fn get_microsoft_network_server__digitally_sign_communications_always(&self) -> Option<&i32> {
        self.microsoft_network_server__digitally_sign_communications_always.as_ref()
    }

    /// Sets the value of MicrosoftNetworkServer_DigitallySignCommunicationsIfClientAgrees
    pub fn set_microsoft_network_server__digitally_sign_communications_if_client_agrees(&mut self, value: i32) {
        self.microsoft_network_server__digitally_sign_communications_if_client_agrees = Some(value);
    }

    /// Gets the value of MicrosoftNetworkServer_DigitallySignCommunicationsIfClientAgrees
    pub fn get_microsoft_network_server__digitally_sign_communications_if_client_agrees(&self) -> Option<&i32> {
        self.microsoft_network_server__digitally_sign_communications_if_client_agrees.as_ref()
    }

    /// Sets the value of NetworkAccess_DoNotAllowAnonymousEnumerationOfSAMAccounts
    pub fn set_network_access__do_not_allow_anonymous_enumeration_of_samaccounts(&mut self, value: i32) {
        self.network_access__do_not_allow_anonymous_enumeration_of_samaccounts = Some(value);
    }

    /// Gets the value of NetworkAccess_DoNotAllowAnonymousEnumerationOfSAMAccounts
    pub fn get_network_access__do_not_allow_anonymous_enumeration_of_samaccounts(&self) -> Option<&i32> {
        self.network_access__do_not_allow_anonymous_enumeration_of_samaccounts.as_ref()
    }

    /// Sets the value of NetworkAccess_DoNotAllowAnonymousEnumerationOfSamAccountsAndShares
    pub fn set_network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares(&mut self, value: i32) {
        self.network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares = Some(value);
    }

    /// Gets the value of NetworkAccess_DoNotAllowAnonymousEnumerationOfSamAccountsAndShares
    pub fn get_network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares(&self) -> Option<&i32> {
        self.network_access__do_not_allow_anonymous_enumeration_of_sam_accounts_and_shares.as_ref()
    }

    /// Sets the value of NetworkAccess_RestrictAnonymousAccessToNamedPipesAndShares
    pub fn set_network_access__restrict_anonymous_access_to_named_pipes_and_shares(&mut self, value: i32) {
        self.network_access__restrict_anonymous_access_to_named_pipes_and_shares = Some(value);
    }

    /// Gets the value of NetworkAccess_RestrictAnonymousAccessToNamedPipesAndShares
    pub fn get_network_access__restrict_anonymous_access_to_named_pipes_and_shares(&self) -> Option<&i32> {
        self.network_access__restrict_anonymous_access_to_named_pipes_and_shares.as_ref()
    }

    /// Sets the value of NetworkAccess_RestrictClientsAllowedToMakeRemoteCallsToSAM
    pub fn set_network_access__restrict_clients_allowed_to_make_remote_calls_to_sam(&mut self, value: String) {
        self.network_access__restrict_clients_allowed_to_make_remote_calls_to_sam = Some(value);
    }

    /// Gets the value of NetworkAccess_RestrictClientsAllowedToMakeRemoteCallsToSAM
    pub fn get_network_access__restrict_clients_allowed_to_make_remote_calls_to_sam(&self) -> Option<&String> {
        self.network_access__restrict_clients_allowed_to_make_remote_calls_to_sam.as_ref()
    }

    /// Sets the value of NetworkSecurity_AllowLocalSystemToUseComputerIdentityForNTLM
    pub fn set_network_security__allow_local_system_to_use_computer_identity_for_ntlm(&mut self, value: i32) {
        self.network_security__allow_local_system_to_use_computer_identity_for_ntlm = Some(value);
    }

    /// Gets the value of NetworkSecurity_AllowLocalSystemToUseComputerIdentityForNTLM
    pub fn get_network_security__allow_local_system_to_use_computer_identity_for_ntlm(&self) -> Option<&i32> {
        self.network_security__allow_local_system_to_use_computer_identity_for_ntlm.as_ref()
    }

    /// Sets the value of NetworkSecurity_AllowPKU2UAuthenticationRequests
    pub fn set_network_security__allow_pku2_uauthentication_requests(&mut self, value: i32) {
        self.network_security__allow_pku2_uauthentication_requests = Some(value);
    }

    /// Gets the value of NetworkSecurity_AllowPKU2UAuthenticationRequests
    pub fn get_network_security__allow_pku2_uauthentication_requests(&self) -> Option<&i32> {
        self.network_security__allow_pku2_uauthentication_requests.as_ref()
    }

    /// Sets the value of NetworkSecurity_DoNotStoreLANManagerHashValueOnNextPasswordChange
    pub fn set_network_security__do_not_store_lanmanager_hash_value_on_next_password_change(&mut self, value: i32) {
        self.network_security__do_not_store_lanmanager_hash_value_on_next_password_change = Some(value);
    }

    /// Gets the value of NetworkSecurity_DoNotStoreLANManagerHashValueOnNextPasswordChange
    pub fn get_network_security__do_not_store_lanmanager_hash_value_on_next_password_change(&self) -> Option<&i32> {
        self.network_security__do_not_store_lanmanager_hash_value_on_next_password_change.as_ref()
    }

    /// Sets the value of NetworkSecurity_LANManagerAuthenticationLevel
    pub fn set_network_security__lanmanager_authentication_level(&mut self, value: i32) {
        self.network_security__lanmanager_authentication_level = Some(value);
    }

    /// Gets the value of NetworkSecurity_LANManagerAuthenticationLevel
    pub fn get_network_security__lanmanager_authentication_level(&self) -> Option<&i32> {
        self.network_security__lanmanager_authentication_level.as_ref()
    }

    /// Sets the value of NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedClients
    pub fn set_network_security__minimum_session_security_for_ntlmsspbased_clients(&mut self, value: i32) {
        self.network_security__minimum_session_security_for_ntlmsspbased_clients = Some(value);
    }

    /// Gets the value of NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedClients
    pub fn get_network_security__minimum_session_security_for_ntlmsspbased_clients(&self) -> Option<&i32> {
        self.network_security__minimum_session_security_for_ntlmsspbased_clients.as_ref()
    }

    /// Sets the value of NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedServers
    pub fn set_network_security__minimum_session_security_for_ntlmsspbased_servers(&mut self, value: i32) {
        self.network_security__minimum_session_security_for_ntlmsspbased_servers = Some(value);
    }

    /// Gets the value of NetworkSecurity_MinimumSessionSecurityForNTLMSSPBasedServers
    pub fn get_network_security__minimum_session_security_for_ntlmsspbased_servers(&self) -> Option<&i32> {
        self.network_security__minimum_session_security_for_ntlmsspbased_servers.as_ref()
    }

    /// Sets the value of NetworkSecurity_RestrictNTLM_AddRemoteServerExceptionsForNTLMAuthentication
    pub fn set_network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication(&mut self, value: String) {
        self.network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication = Some(value);
    }

    /// Gets the value of NetworkSecurity_RestrictNTLM_AddRemoteServerExceptionsForNTLMAuthentication
    pub fn get_network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication(&self) -> Option<&String> {
        self.network_security__restrict_ntlm__add_remote_server_exceptions_for_ntlmauthentication.as_ref()
    }

    /// Sets the value of NetworkSecurity_RestrictNTLM_AuditIncomingNTLMTraffic
    pub fn set_network_security__restrict_ntlm__audit_incoming_ntlmtraffic(&mut self, value: i32) {
        self.network_security__restrict_ntlm__audit_incoming_ntlmtraffic = Some(value);
    }

    /// Gets the value of NetworkSecurity_RestrictNTLM_AuditIncomingNTLMTraffic
    pub fn get_network_security__restrict_ntlm__audit_incoming_ntlmtraffic(&self) -> Option<&i32> {
        self.network_security__restrict_ntlm__audit_incoming_ntlmtraffic.as_ref()
    }

    /// Sets the value of NetworkSecurity_RestrictNTLM_IncomingNTLMTraffic
    pub fn set_network_security__restrict_ntlm__incoming_ntlmtraffic(&mut self, value: i32) {
        self.network_security__restrict_ntlm__incoming_ntlmtraffic = Some(value);
    }

    /// Gets the value of NetworkSecurity_RestrictNTLM_IncomingNTLMTraffic
    pub fn get_network_security__restrict_ntlm__incoming_ntlmtraffic(&self) -> Option<&i32> {
        self.network_security__restrict_ntlm__incoming_ntlmtraffic.as_ref()
    }

    /// Sets the value of NetworkSecurity_RestrictNTLM_OutgoingNTLMTrafficToRemoteServers
    pub fn set_network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers(&mut self, value: i32) {
        self.network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers = Some(value);
    }

    /// Gets the value of NetworkSecurity_RestrictNTLM_OutgoingNTLMTrafficToRemoteServers
    pub fn get_network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers(&self) -> Option<&i32> {
        self.network_security__restrict_ntlm__outgoing_ntlmtraffic_to_remote_servers.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Shutdown_AllowSystemToBeShutDownWithoutHavingToLogOn
    pub fn set_shutdown__allow_system_to_be_shut_down_without_having_to_log_on(&mut self, value: i32) {
        self.shutdown__allow_system_to_be_shut_down_without_having_to_log_on = Some(value);
    }

    /// Gets the value of Shutdown_AllowSystemToBeShutDownWithoutHavingToLogOn
    pub fn get_shutdown__allow_system_to_be_shut_down_without_having_to_log_on(&self) -> Option<&i32> {
        self.shutdown__allow_system_to_be_shut_down_without_having_to_log_on.as_ref()
    }

    /// Sets the value of Shutdown_ClearVirtualMemoryPageFile
    pub fn set_shutdown__clear_virtual_memory_page_file(&mut self, value: i32) {
        self.shutdown__clear_virtual_memory_page_file = Some(value);
    }

    /// Gets the value of Shutdown_ClearVirtualMemoryPageFile
    pub fn get_shutdown__clear_virtual_memory_page_file(&self) -> Option<&i32> {
        self.shutdown__clear_virtual_memory_page_file.as_ref()
    }

    /// Sets the value of UserAccountControl_AllowUIAccessApplicationsToPromptForElevation
    pub fn set_user_account_control__allow_uiaccess_applications_to_prompt_for_elevation(&mut self, value: i32) {
        self.user_account_control__allow_uiaccess_applications_to_prompt_for_elevation = Some(value);
    }

    /// Gets the value of UserAccountControl_AllowUIAccessApplicationsToPromptForElevation
    pub fn get_user_account_control__allow_uiaccess_applications_to_prompt_for_elevation(&self) -> Option<&i32> {
        self.user_account_control__allow_uiaccess_applications_to_prompt_for_elevation.as_ref()
    }

    /// Sets the value of UserAccountControl_BehaviorOfTheElevationPromptForAdministrators
    pub fn set_user_account_control__behavior_of_the_elevation_prompt_for_administrators(&mut self, value: i32) {
        self.user_account_control__behavior_of_the_elevation_prompt_for_administrators = Some(value);
    }

    /// Gets the value of UserAccountControl_BehaviorOfTheElevationPromptForAdministrators
    pub fn get_user_account_control__behavior_of_the_elevation_prompt_for_administrators(&self) -> Option<&i32> {
        self.user_account_control__behavior_of_the_elevation_prompt_for_administrators.as_ref()
    }

    /// Sets the value of UserAccountControl_BehaviorOfTheElevationPromptForStandardUsers
    pub fn set_user_account_control__behavior_of_the_elevation_prompt_for_standard_users(&mut self, value: i32) {
        self.user_account_control__behavior_of_the_elevation_prompt_for_standard_users = Some(value);
    }

    /// Gets the value of UserAccountControl_BehaviorOfTheElevationPromptForStandardUsers
    pub fn get_user_account_control__behavior_of_the_elevation_prompt_for_standard_users(&self) -> Option<&i32> {
        self.user_account_control__behavior_of_the_elevation_prompt_for_standard_users.as_ref()
    }

    /// Sets the value of UserAccountControl_DetectApplicationInstallationsAndPromptForElevation
    pub fn set_user_account_control__detect_application_installations_and_prompt_for_elevation(&mut self, value: i32) {
        self.user_account_control__detect_application_installations_and_prompt_for_elevation = Some(value);
    }

    /// Gets the value of UserAccountControl_DetectApplicationInstallationsAndPromptForElevation
    pub fn get_user_account_control__detect_application_installations_and_prompt_for_elevation(&self) -> Option<&i32> {
        self.user_account_control__detect_application_installations_and_prompt_for_elevation.as_ref()
    }

    /// Sets the value of UserAccountControl_OnlyElevateExecutableFilesThatAreSignedAndValidated
    pub fn set_user_account_control__only_elevate_executable_files_that_are_signed_and_validated(&mut self, value: i32) {
        self.user_account_control__only_elevate_executable_files_that_are_signed_and_validated = Some(value);
    }

    /// Gets the value of UserAccountControl_OnlyElevateExecutableFilesThatAreSignedAndValidated
    pub fn get_user_account_control__only_elevate_executable_files_that_are_signed_and_validated(&self) -> Option<&i32> {
        self.user_account_control__only_elevate_executable_files_that_are_signed_and_validated.as_ref()
    }

    /// Sets the value of UserAccountControl_OnlyElevateUIAccessApplicationsThatAreInstalledInSecureLocations
    pub fn set_user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations(&mut self, value: i32) {
        self.user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations = Some(value);
    }

    /// Gets the value of UserAccountControl_OnlyElevateUIAccessApplicationsThatAreInstalledInSecureLocations
    pub fn get_user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations(&self) -> Option<&i32> {
        self.user_account_control__only_elevate_uiaccess_applications_that_are_installed_in_secure_locations.as_ref()
    }

    /// Sets the value of UserAccountControl_RunAllAdministratorsInAdminApprovalMode
    pub fn set_user_account_control__run_all_administrators_in_admin_approval_mode(&mut self, value: i32) {
        self.user_account_control__run_all_administrators_in_admin_approval_mode = Some(value);
    }

    /// Gets the value of UserAccountControl_RunAllAdministratorsInAdminApprovalMode
    pub fn get_user_account_control__run_all_administrators_in_admin_approval_mode(&self) -> Option<&i32> {
        self.user_account_control__run_all_administrators_in_admin_approval_mode.as_ref()
    }

    /// Sets the value of UserAccountControl_SwitchToTheSecureDesktopWhenPromptingForElevation
    pub fn set_user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation(&mut self, value: i32) {
        self.user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation = Some(value);
    }

    /// Gets the value of UserAccountControl_SwitchToTheSecureDesktopWhenPromptingForElevation
    pub fn get_user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation(&self) -> Option<&i32> {
        self.user_account_control__switch_to_the_secure_desktop_when_prompting_for_elevation.as_ref()
    }

    /// Sets the value of UserAccountControl_UseAdminApprovalMode
    pub fn set_user_account_control__use_admin_approval_mode(&mut self, value: i32) {
        self.user_account_control__use_admin_approval_mode = Some(value);
    }

    /// Gets the value of UserAccountControl_UseAdminApprovalMode
    pub fn get_user_account_control__use_admin_approval_mode(&self) -> Option<&i32> {
        self.user_account_control__use_admin_approval_mode.as_ref()
    }

    /// Sets the value of UserAccountControl_VirtualizeFileAndRegistryWriteFailuresToPerUserLocations
    pub fn set_user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations(&mut self, value: i32) {
        self.user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations = Some(value);
    }

    /// Gets the value of UserAccountControl_VirtualizeFileAndRegistryWriteFailuresToPerUserLocations
    pub fn get_user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations(&self) -> Option<&i32> {
        self.user_account_control__virtualize_file_and_registry_write_failures_to_per_user_locations.as_ref()
    }
}

