// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbServerConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbServerConfiguration {

/// 
    #[serde(rename = "AnnounceComment")]
    pub announce_comment: Option<String>,

/// 
    #[serde(rename = "AnnounceServer")]
    pub announce_server: Option<bool>,

/// 
    #[serde(rename = "AsynchronousCredits")]
    pub asynchronous_credits: Option<u32>,

/// 
    #[serde(rename = "AuditClientCertificateAccess")]
    pub audit_client_certificate_access: Option<bool>,

/// 
    #[serde(rename = "AuditClientDoesNotSupportEncryption")]
    pub audit_client_does_not_support_encryption: Option<bool>,

/// 
    #[serde(rename = "AuditClientDoesNotSupportSigning")]
    pub audit_client_does_not_support_signing: Option<bool>,

/// 
    #[serde(rename = "AuditInsecureGuestLogon")]
    pub audit_insecure_guest_logon: Option<bool>,

/// 
    #[serde(rename = "AuditSmb1Access")]
    pub audit_smb1_access: Option<bool>,

/// 
    #[serde(rename = "AutoDisconnectTimeoutInMinutesV1")]
    pub auto_disconnect_timeout_in_minutes_v1: Option<u32>,

/// 
    #[serde(rename = "AutoDisconnectTimeoutInSecondsV2")]
    pub auto_disconnect_timeout_in_seconds_v2: Option<u32>,

/// 
    #[serde(rename = "AutoShareServer")]
    pub auto_share_server: Option<bool>,

/// 
    #[serde(rename = "AutoShareWorkstation")]
    pub auto_share_workstation: Option<bool>,

/// 
    #[serde(rename = "CachedOpenLimit")]
    pub cached_open_limit: Option<u32>,

/// 
    #[serde(rename = "DisableCompression")]
    pub disable_compression: Option<bool>,

/// 
    #[serde(rename = "DisableSmbEncryptionOnSecureConnection")]
    pub disable_smb_encryption_on_secure_connection: Option<bool>,

/// 
    #[serde(rename = "DurableHandleV2TimeoutInSeconds")]
    pub durable_handle_v2_timeout_in_seconds: Option<u32>,

/// 
    #[serde(rename = "EnableAuthenticateUserSharing")]
    pub enable_authenticate_user_sharing: Option<bool>,

/// 
    #[serde(rename = "EnableAuthRateLimiter")]
    pub enable_auth_rate_limiter: Option<bool>,

/// 
    #[serde(rename = "EnableDirectoryHandleLeasing")]
    pub enable_directory_handle_leasing: Option<bool>,

/// 
    #[serde(rename = "EnableDownlevelTimewarp")]
    pub enable_downlevel_timewarp: Option<bool>,

/// 
    #[serde(rename = "EnableForcedLogoff")]
    pub enable_forced_logoff: Option<bool>,

/// 
    #[serde(rename = "EnableLeasing")]
    pub enable_leasing: Option<bool>,

/// 
    #[serde(rename = "EnableMailslots")]
    pub enable_mailslots: Option<bool>,

/// 
    #[serde(rename = "EnableMultiChannel")]
    pub enable_multi_channel: Option<bool>,

/// 
    #[serde(rename = "EnableOplocks")]
    pub enable_oplocks: Option<bool>,

/// 
    #[serde(rename = "EnableSecuritySignature")]
    pub enable_security_signature: Option<bool>,

/// 
    #[serde(rename = "EnableSMB1Protocol")]
    pub enable_smb1_protocol: Option<bool>,

/// 
    #[serde(rename = "EnableSMB2Protocol")]
    pub enable_smb2_protocol: Option<bool>,

/// 
    #[serde(rename = "EnableSMBQUIC")]
    pub enable_smbquic: Option<bool>,

/// 
    #[serde(rename = "EnableStrictNameChecking")]
    pub enable_strict_name_checking: Option<bool>,

/// 
    #[serde(rename = "EncryptData")]
    pub encrypt_data: Option<bool>,

/// 
    #[serde(rename = "EncryptionCiphers")]
    pub encryption_ciphers: Option<String>,

/// 
    #[serde(rename = "InvalidAuthenticationDelayTimeInMs")]
    pub invalid_authentication_delay_time_in_ms: Option<u32>,

/// 
    #[serde(rename = "IrpStackSize")]
    pub irp_stack_size: Option<u32>,

/// 
    #[serde(rename = "KeepAliveTime")]
    pub keep_alive_time: Option<u32>,

/// 
    #[serde(rename = "MaxChannelPerSession")]
    pub max_channel_per_session: Option<u32>,

/// 
    #[serde(rename = "MaxMpxCount")]
    pub max_mpx_count: Option<u32>,

/// 
    #[serde(rename = "MaxSessionPerConnection")]
    pub max_session_per_connection: Option<u32>,

/// 
    #[serde(rename = "MaxThreadsPerQueue")]
    pub max_threads_per_queue: Option<u32>,

/// 
    #[serde(rename = "MaxWorkItems")]
    pub max_work_items: Option<u32>,

/// 
    #[serde(rename = "NullSessionPipes")]
    pub null_session_pipes: Option<String>,

/// 
    #[serde(rename = "NullSessionShares")]
    pub null_session_shares: Option<String>,

/// 
    #[serde(rename = "OplockBreakWait")]
    pub oplock_break_wait: Option<u32>,

/// 
    #[serde(rename = "PendingClientTimeoutInSeconds")]
    pub pending_client_timeout_in_seconds: Option<u32>,

/// 
    #[serde(rename = "RejectUnencryptedAccess")]
    pub reject_unencrypted_access: Option<bool>,

/// 
    #[serde(rename = "RequestCompression")]
    pub request_compression: Option<bool>,

/// 
    #[serde(rename = "RequireSecuritySignature")]
    pub require_security_signature: Option<bool>,

/// 
    #[serde(rename = "RestrictNamedpipeAccessViaQuic")]
    pub restrict_namedpipe_access_via_quic: Option<bool>,

/// 
    #[serde(rename = "ServerHidden")]
    pub server_hidden: Option<bool>,

/// 
    #[serde(rename = "Smb2CreditsMax")]
    pub smb2_credits_max: Option<u32>,

/// 
    #[serde(rename = "Smb2CreditsMin")]
    pub smb2_credits_min: Option<u32>,

/// 
    #[serde(rename = "Smb2DialectMax")]
    pub smb2_dialect_max: Option<SmbServerConfiguration_Smb2DialectMax>,

/// 
    #[serde(rename = "Smb2DialectMin")]
    pub smb2_dialect_min: Option<SmbServerConfiguration_Smb2DialectMin>,

/// 
    #[serde(rename = "SmbServerNameHardeningLevel")]
    pub smb_server_name_hardening_level: Option<u32>,

/// 
    #[serde(rename = "TreatHostAsStableStorage")]
    pub treat_host_as_stable_storage: Option<bool>,

/// 
    #[serde(rename = "ValidateAliasNotCircular")]
    pub validate_alias_not_circular: Option<bool>,

/// 
    #[serde(rename = "ValidateShareScope")]
    pub validate_share_scope: Option<bool>,

/// 
    #[serde(rename = "ValidateShareScopeNotAliased")]
    pub validate_share_scope_not_aliased: Option<bool>,

/// 
    #[serde(rename = "ValidateTargetName")]
    pub validate_target_name: Option<bool>,
}

impl MSFT_SmbServerConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            announce_comment: None,
            announce_server: None,
            asynchronous_credits: None,
            audit_client_certificate_access: None,
            audit_client_does_not_support_encryption: None,
            audit_client_does_not_support_signing: None,
            audit_insecure_guest_logon: None,
            audit_smb1_access: None,
            auto_disconnect_timeout_in_minutes_v1: None,
            auto_disconnect_timeout_in_seconds_v2: None,
            auto_share_server: None,
            auto_share_workstation: None,
            cached_open_limit: None,
            disable_compression: None,
            disable_smb_encryption_on_secure_connection: None,
            durable_handle_v2_timeout_in_seconds: None,
            enable_authenticate_user_sharing: None,
            enable_auth_rate_limiter: None,
            enable_directory_handle_leasing: None,
            enable_downlevel_timewarp: None,
            enable_forced_logoff: None,
            enable_leasing: None,
            enable_mailslots: None,
            enable_multi_channel: None,
            enable_oplocks: None,
            enable_security_signature: None,
            enable_smb1_protocol: None,
            enable_smb2_protocol: None,
            enable_smbquic: None,
            enable_strict_name_checking: None,
            encrypt_data: None,
            encryption_ciphers: None,
            invalid_authentication_delay_time_in_ms: None,
            irp_stack_size: None,
            keep_alive_time: None,
            max_channel_per_session: None,
            max_mpx_count: None,
            max_session_per_connection: None,
            max_threads_per_queue: None,
            max_work_items: None,
            null_session_pipes: None,
            null_session_shares: None,
            oplock_break_wait: None,
            pending_client_timeout_in_seconds: None,
            reject_unencrypted_access: None,
            request_compression: None,
            require_security_signature: None,
            restrict_namedpipe_access_via_quic: None,
            server_hidden: None,
            smb2_credits_max: None,
            smb2_credits_min: None,
            smb2_dialect_max: None,
            smb2_dialect_min: None,
            smb_server_name_hardening_level: None,
            treat_host_as_stable_storage: None,
            validate_alias_not_circular: None,
            validate_share_scope: None,
            validate_share_scope_not_aliased: None,
            validate_target_name: None,
        }
    }


    /// Sets the value of AnnounceComment
    pub fn set_announce_comment(&mut self, value: String) {
        self.announce_comment = Some(value);
    }

    /// Gets the value of AnnounceComment
    pub fn get_announce_comment(&self) -> Option<&String> {
        self.announce_comment.as_ref()
    }

    /// Sets the value of AnnounceServer
    pub fn set_announce_server(&mut self, value: bool) {
        self.announce_server = Some(value);
    }

    /// Gets the value of AnnounceServer
    pub fn get_announce_server(&self) -> Option<&bool> {
        self.announce_server.as_ref()
    }

    /// Sets the value of AsynchronousCredits
    pub fn set_asynchronous_credits(&mut self, value: u32) {
        self.asynchronous_credits = Some(value);
    }

    /// Gets the value of AsynchronousCredits
    pub fn get_asynchronous_credits(&self) -> Option<&u32> {
        self.asynchronous_credits.as_ref()
    }

    /// Sets the value of AuditClientCertificateAccess
    pub fn set_audit_client_certificate_access(&mut self, value: bool) {
        self.audit_client_certificate_access = Some(value);
    }

    /// Gets the value of AuditClientCertificateAccess
    pub fn get_audit_client_certificate_access(&self) -> Option<&bool> {
        self.audit_client_certificate_access.as_ref()
    }

    /// Sets the value of AuditClientDoesNotSupportEncryption
    pub fn set_audit_client_does_not_support_encryption(&mut self, value: bool) {
        self.audit_client_does_not_support_encryption = Some(value);
    }

    /// Gets the value of AuditClientDoesNotSupportEncryption
    pub fn get_audit_client_does_not_support_encryption(&self) -> Option<&bool> {
        self.audit_client_does_not_support_encryption.as_ref()
    }

    /// Sets the value of AuditClientDoesNotSupportSigning
    pub fn set_audit_client_does_not_support_signing(&mut self, value: bool) {
        self.audit_client_does_not_support_signing = Some(value);
    }

    /// Gets the value of AuditClientDoesNotSupportSigning
    pub fn get_audit_client_does_not_support_signing(&self) -> Option<&bool> {
        self.audit_client_does_not_support_signing.as_ref()
    }

    /// Sets the value of AuditInsecureGuestLogon
    pub fn set_audit_insecure_guest_logon(&mut self, value: bool) {
        self.audit_insecure_guest_logon = Some(value);
    }

    /// Gets the value of AuditInsecureGuestLogon
    pub fn get_audit_insecure_guest_logon(&self) -> Option<&bool> {
        self.audit_insecure_guest_logon.as_ref()
    }

    /// Sets the value of AuditSmb1Access
    pub fn set_audit_smb1_access(&mut self, value: bool) {
        self.audit_smb1_access = Some(value);
    }

    /// Gets the value of AuditSmb1Access
    pub fn get_audit_smb1_access(&self) -> Option<&bool> {
        self.audit_smb1_access.as_ref()
    }

    /// Sets the value of AutoDisconnectTimeoutInMinutesV1
    pub fn set_auto_disconnect_timeout_in_minutes_v1(&mut self, value: u32) {
        self.auto_disconnect_timeout_in_minutes_v1 = Some(value);
    }

    /// Gets the value of AutoDisconnectTimeoutInMinutesV1
    pub fn get_auto_disconnect_timeout_in_minutes_v1(&self) -> Option<&u32> {
        self.auto_disconnect_timeout_in_minutes_v1.as_ref()
    }

    /// Sets the value of AutoDisconnectTimeoutInSecondsV2
    pub fn set_auto_disconnect_timeout_in_seconds_v2(&mut self, value: u32) {
        self.auto_disconnect_timeout_in_seconds_v2 = Some(value);
    }

    /// Gets the value of AutoDisconnectTimeoutInSecondsV2
    pub fn get_auto_disconnect_timeout_in_seconds_v2(&self) -> Option<&u32> {
        self.auto_disconnect_timeout_in_seconds_v2.as_ref()
    }

    /// Sets the value of AutoShareServer
    pub fn set_auto_share_server(&mut self, value: bool) {
        self.auto_share_server = Some(value);
    }

    /// Gets the value of AutoShareServer
    pub fn get_auto_share_server(&self) -> Option<&bool> {
        self.auto_share_server.as_ref()
    }

    /// Sets the value of AutoShareWorkstation
    pub fn set_auto_share_workstation(&mut self, value: bool) {
        self.auto_share_workstation = Some(value);
    }

    /// Gets the value of AutoShareWorkstation
    pub fn get_auto_share_workstation(&self) -> Option<&bool> {
        self.auto_share_workstation.as_ref()
    }

    /// Sets the value of CachedOpenLimit
    pub fn set_cached_open_limit(&mut self, value: u32) {
        self.cached_open_limit = Some(value);
    }

    /// Gets the value of CachedOpenLimit
    pub fn get_cached_open_limit(&self) -> Option<&u32> {
        self.cached_open_limit.as_ref()
    }

    /// Sets the value of DisableCompression
    pub fn set_disable_compression(&mut self, value: bool) {
        self.disable_compression = Some(value);
    }

    /// Gets the value of DisableCompression
    pub fn get_disable_compression(&self) -> Option<&bool> {
        self.disable_compression.as_ref()
    }

    /// Sets the value of DisableSmbEncryptionOnSecureConnection
    pub fn set_disable_smb_encryption_on_secure_connection(&mut self, value: bool) {
        self.disable_smb_encryption_on_secure_connection = Some(value);
    }

    /// Gets the value of DisableSmbEncryptionOnSecureConnection
    pub fn get_disable_smb_encryption_on_secure_connection(&self) -> Option<&bool> {
        self.disable_smb_encryption_on_secure_connection.as_ref()
    }

    /// Sets the value of DurableHandleV2TimeoutInSeconds
    pub fn set_durable_handle_v2_timeout_in_seconds(&mut self, value: u32) {
        self.durable_handle_v2_timeout_in_seconds = Some(value);
    }

    /// Gets the value of DurableHandleV2TimeoutInSeconds
    pub fn get_durable_handle_v2_timeout_in_seconds(&self) -> Option<&u32> {
        self.durable_handle_v2_timeout_in_seconds.as_ref()
    }

    /// Sets the value of EnableAuthenticateUserSharing
    pub fn set_enable_authenticate_user_sharing(&mut self, value: bool) {
        self.enable_authenticate_user_sharing = Some(value);
    }

    /// Gets the value of EnableAuthenticateUserSharing
    pub fn get_enable_authenticate_user_sharing(&self) -> Option<&bool> {
        self.enable_authenticate_user_sharing.as_ref()
    }

    /// Sets the value of EnableAuthRateLimiter
    pub fn set_enable_auth_rate_limiter(&mut self, value: bool) {
        self.enable_auth_rate_limiter = Some(value);
    }

    /// Gets the value of EnableAuthRateLimiter
    pub fn get_enable_auth_rate_limiter(&self) -> Option<&bool> {
        self.enable_auth_rate_limiter.as_ref()
    }

    /// Sets the value of EnableDirectoryHandleLeasing
    pub fn set_enable_directory_handle_leasing(&mut self, value: bool) {
        self.enable_directory_handle_leasing = Some(value);
    }

    /// Gets the value of EnableDirectoryHandleLeasing
    pub fn get_enable_directory_handle_leasing(&self) -> Option<&bool> {
        self.enable_directory_handle_leasing.as_ref()
    }

    /// Sets the value of EnableDownlevelTimewarp
    pub fn set_enable_downlevel_timewarp(&mut self, value: bool) {
        self.enable_downlevel_timewarp = Some(value);
    }

    /// Gets the value of EnableDownlevelTimewarp
    pub fn get_enable_downlevel_timewarp(&self) -> Option<&bool> {
        self.enable_downlevel_timewarp.as_ref()
    }

    /// Sets the value of EnableForcedLogoff
    pub fn set_enable_forced_logoff(&mut self, value: bool) {
        self.enable_forced_logoff = Some(value);
    }

    /// Gets the value of EnableForcedLogoff
    pub fn get_enable_forced_logoff(&self) -> Option<&bool> {
        self.enable_forced_logoff.as_ref()
    }

    /// Sets the value of EnableLeasing
    pub fn set_enable_leasing(&mut self, value: bool) {
        self.enable_leasing = Some(value);
    }

    /// Gets the value of EnableLeasing
    pub fn get_enable_leasing(&self) -> Option<&bool> {
        self.enable_leasing.as_ref()
    }

    /// Sets the value of EnableMailslots
    pub fn set_enable_mailslots(&mut self, value: bool) {
        self.enable_mailslots = Some(value);
    }

    /// Gets the value of EnableMailslots
    pub fn get_enable_mailslots(&self) -> Option<&bool> {
        self.enable_mailslots.as_ref()
    }

    /// Sets the value of EnableMultiChannel
    pub fn set_enable_multi_channel(&mut self, value: bool) {
        self.enable_multi_channel = Some(value);
    }

    /// Gets the value of EnableMultiChannel
    pub fn get_enable_multi_channel(&self) -> Option<&bool> {
        self.enable_multi_channel.as_ref()
    }

    /// Sets the value of EnableOplocks
    pub fn set_enable_oplocks(&mut self, value: bool) {
        self.enable_oplocks = Some(value);
    }

    /// Gets the value of EnableOplocks
    pub fn get_enable_oplocks(&self) -> Option<&bool> {
        self.enable_oplocks.as_ref()
    }

    /// Sets the value of EnableSecuritySignature
    pub fn set_enable_security_signature(&mut self, value: bool) {
        self.enable_security_signature = Some(value);
    }

    /// Gets the value of EnableSecuritySignature
    pub fn get_enable_security_signature(&self) -> Option<&bool> {
        self.enable_security_signature.as_ref()
    }

    /// Sets the value of EnableSMB1Protocol
    pub fn set_enable_smb1_protocol(&mut self, value: bool) {
        self.enable_smb1_protocol = Some(value);
    }

    /// Gets the value of EnableSMB1Protocol
    pub fn get_enable_smb1_protocol(&self) -> Option<&bool> {
        self.enable_smb1_protocol.as_ref()
    }

    /// Sets the value of EnableSMB2Protocol
    pub fn set_enable_smb2_protocol(&mut self, value: bool) {
        self.enable_smb2_protocol = Some(value);
    }

    /// Gets the value of EnableSMB2Protocol
    pub fn get_enable_smb2_protocol(&self) -> Option<&bool> {
        self.enable_smb2_protocol.as_ref()
    }

    /// Sets the value of EnableSMBQUIC
    pub fn set_enable_smbquic(&mut self, value: bool) {
        self.enable_smbquic = Some(value);
    }

    /// Gets the value of EnableSMBQUIC
    pub fn get_enable_smbquic(&self) -> Option<&bool> {
        self.enable_smbquic.as_ref()
    }

    /// Sets the value of EnableStrictNameChecking
    pub fn set_enable_strict_name_checking(&mut self, value: bool) {
        self.enable_strict_name_checking = Some(value);
    }

    /// Gets the value of EnableStrictNameChecking
    pub fn get_enable_strict_name_checking(&self) -> Option<&bool> {
        self.enable_strict_name_checking.as_ref()
    }

    /// Sets the value of EncryptData
    pub fn set_encrypt_data(&mut self, value: bool) {
        self.encrypt_data = Some(value);
    }

    /// Gets the value of EncryptData
    pub fn get_encrypt_data(&self) -> Option<&bool> {
        self.encrypt_data.as_ref()
    }

    /// Sets the value of EncryptionCiphers
    pub fn set_encryption_ciphers(&mut self, value: String) {
        self.encryption_ciphers = Some(value);
    }

    /// Gets the value of EncryptionCiphers
    pub fn get_encryption_ciphers(&self) -> Option<&String> {
        self.encryption_ciphers.as_ref()
    }

    /// Sets the value of InvalidAuthenticationDelayTimeInMs
    pub fn set_invalid_authentication_delay_time_in_ms(&mut self, value: u32) {
        self.invalid_authentication_delay_time_in_ms = Some(value);
    }

    /// Gets the value of InvalidAuthenticationDelayTimeInMs
    pub fn get_invalid_authentication_delay_time_in_ms(&self) -> Option<&u32> {
        self.invalid_authentication_delay_time_in_ms.as_ref()
    }

    /// Sets the value of IrpStackSize
    pub fn set_irp_stack_size(&mut self, value: u32) {
        self.irp_stack_size = Some(value);
    }

    /// Gets the value of IrpStackSize
    pub fn get_irp_stack_size(&self) -> Option<&u32> {
        self.irp_stack_size.as_ref()
    }

    /// Sets the value of KeepAliveTime
    pub fn set_keep_alive_time(&mut self, value: u32) {
        self.keep_alive_time = Some(value);
    }

    /// Gets the value of KeepAliveTime
    pub fn get_keep_alive_time(&self) -> Option<&u32> {
        self.keep_alive_time.as_ref()
    }

    /// Sets the value of MaxChannelPerSession
    pub fn set_max_channel_per_session(&mut self, value: u32) {
        self.max_channel_per_session = Some(value);
    }

    /// Gets the value of MaxChannelPerSession
    pub fn get_max_channel_per_session(&self) -> Option<&u32> {
        self.max_channel_per_session.as_ref()
    }

    /// Sets the value of MaxMpxCount
    pub fn set_max_mpx_count(&mut self, value: u32) {
        self.max_mpx_count = Some(value);
    }

    /// Gets the value of MaxMpxCount
    pub fn get_max_mpx_count(&self) -> Option<&u32> {
        self.max_mpx_count.as_ref()
    }

    /// Sets the value of MaxSessionPerConnection
    pub fn set_max_session_per_connection(&mut self, value: u32) {
        self.max_session_per_connection = Some(value);
    }

    /// Gets the value of MaxSessionPerConnection
    pub fn get_max_session_per_connection(&self) -> Option<&u32> {
        self.max_session_per_connection.as_ref()
    }

    /// Sets the value of MaxThreadsPerQueue
    pub fn set_max_threads_per_queue(&mut self, value: u32) {
        self.max_threads_per_queue = Some(value);
    }

    /// Gets the value of MaxThreadsPerQueue
    pub fn get_max_threads_per_queue(&self) -> Option<&u32> {
        self.max_threads_per_queue.as_ref()
    }

    /// Sets the value of MaxWorkItems
    pub fn set_max_work_items(&mut self, value: u32) {
        self.max_work_items = Some(value);
    }

    /// Gets the value of MaxWorkItems
    pub fn get_max_work_items(&self) -> Option<&u32> {
        self.max_work_items.as_ref()
    }

    /// Sets the value of NullSessionPipes
    pub fn set_null_session_pipes(&mut self, value: String) {
        self.null_session_pipes = Some(value);
    }

    /// Gets the value of NullSessionPipes
    pub fn get_null_session_pipes(&self) -> Option<&String> {
        self.null_session_pipes.as_ref()
    }

    /// Sets the value of NullSessionShares
    pub fn set_null_session_shares(&mut self, value: String) {
        self.null_session_shares = Some(value);
    }

    /// Gets the value of NullSessionShares
    pub fn get_null_session_shares(&self) -> Option<&String> {
        self.null_session_shares.as_ref()
    }

    /// Sets the value of OplockBreakWait
    pub fn set_oplock_break_wait(&mut self, value: u32) {
        self.oplock_break_wait = Some(value);
    }

    /// Gets the value of OplockBreakWait
    pub fn get_oplock_break_wait(&self) -> Option<&u32> {
        self.oplock_break_wait.as_ref()
    }

    /// Sets the value of PendingClientTimeoutInSeconds
    pub fn set_pending_client_timeout_in_seconds(&mut self, value: u32) {
        self.pending_client_timeout_in_seconds = Some(value);
    }

    /// Gets the value of PendingClientTimeoutInSeconds
    pub fn get_pending_client_timeout_in_seconds(&self) -> Option<&u32> {
        self.pending_client_timeout_in_seconds.as_ref()
    }

    /// Sets the value of RejectUnencryptedAccess
    pub fn set_reject_unencrypted_access(&mut self, value: bool) {
        self.reject_unencrypted_access = Some(value);
    }

    /// Gets the value of RejectUnencryptedAccess
    pub fn get_reject_unencrypted_access(&self) -> Option<&bool> {
        self.reject_unencrypted_access.as_ref()
    }

    /// Sets the value of RequestCompression
    pub fn set_request_compression(&mut self, value: bool) {
        self.request_compression = Some(value);
    }

    /// Gets the value of RequestCompression
    pub fn get_request_compression(&self) -> Option<&bool> {
        self.request_compression.as_ref()
    }

    /// Sets the value of RequireSecuritySignature
    pub fn set_require_security_signature(&mut self, value: bool) {
        self.require_security_signature = Some(value);
    }

    /// Gets the value of RequireSecuritySignature
    pub fn get_require_security_signature(&self) -> Option<&bool> {
        self.require_security_signature.as_ref()
    }

    /// Sets the value of RestrictNamedpipeAccessViaQuic
    pub fn set_restrict_namedpipe_access_via_quic(&mut self, value: bool) {
        self.restrict_namedpipe_access_via_quic = Some(value);
    }

    /// Gets the value of RestrictNamedpipeAccessViaQuic
    pub fn get_restrict_namedpipe_access_via_quic(&self) -> Option<&bool> {
        self.restrict_namedpipe_access_via_quic.as_ref()
    }

    /// Sets the value of ServerHidden
    pub fn set_server_hidden(&mut self, value: bool) {
        self.server_hidden = Some(value);
    }

    /// Gets the value of ServerHidden
    pub fn get_server_hidden(&self) -> Option<&bool> {
        self.server_hidden.as_ref()
    }

    /// Sets the value of Smb2CreditsMax
    pub fn set_smb2_credits_max(&mut self, value: u32) {
        self.smb2_credits_max = Some(value);
    }

    /// Gets the value of Smb2CreditsMax
    pub fn get_smb2_credits_max(&self) -> Option<&u32> {
        self.smb2_credits_max.as_ref()
    }

    /// Sets the value of Smb2CreditsMin
    pub fn set_smb2_credits_min(&mut self, value: u32) {
        self.smb2_credits_min = Some(value);
    }

    /// Gets the value of Smb2CreditsMin
    pub fn get_smb2_credits_min(&self) -> Option<&u32> {
        self.smb2_credits_min.as_ref()
    }

    /// Sets the value of Smb2DialectMax
    pub fn set_smb2_dialect_max(&mut self, value: SmbServerConfiguration_Smb2DialectMax) {
        self.smb2_dialect_max = Some(value);
    }

    /// Gets the value of Smb2DialectMax
    pub fn get_smb2_dialect_max(&self) -> Option<&SmbServerConfiguration_Smb2DialectMax> {
        self.smb2_dialect_max.as_ref()
    }

    /// Sets the value of Smb2DialectMin
    pub fn set_smb2_dialect_min(&mut self, value: SmbServerConfiguration_Smb2DialectMin) {
        self.smb2_dialect_min = Some(value);
    }

    /// Gets the value of Smb2DialectMin
    pub fn get_smb2_dialect_min(&self) -> Option<&SmbServerConfiguration_Smb2DialectMin> {
        self.smb2_dialect_min.as_ref()
    }

    /// Sets the value of SmbServerNameHardeningLevel
    pub fn set_smb_server_name_hardening_level(&mut self, value: u32) {
        self.smb_server_name_hardening_level = Some(value);
    }

    /// Gets the value of SmbServerNameHardeningLevel
    pub fn get_smb_server_name_hardening_level(&self) -> Option<&u32> {
        self.smb_server_name_hardening_level.as_ref()
    }

    /// Sets the value of TreatHostAsStableStorage
    pub fn set_treat_host_as_stable_storage(&mut self, value: bool) {
        self.treat_host_as_stable_storage = Some(value);
    }

    /// Gets the value of TreatHostAsStableStorage
    pub fn get_treat_host_as_stable_storage(&self) -> Option<&bool> {
        self.treat_host_as_stable_storage.as_ref()
    }

    /// Sets the value of ValidateAliasNotCircular
    pub fn set_validate_alias_not_circular(&mut self, value: bool) {
        self.validate_alias_not_circular = Some(value);
    }

    /// Gets the value of ValidateAliasNotCircular
    pub fn get_validate_alias_not_circular(&self) -> Option<&bool> {
        self.validate_alias_not_circular.as_ref()
    }

    /// Sets the value of ValidateShareScope
    pub fn set_validate_share_scope(&mut self, value: bool) {
        self.validate_share_scope = Some(value);
    }

    /// Gets the value of ValidateShareScope
    pub fn get_validate_share_scope(&self) -> Option<&bool> {
        self.validate_share_scope.as_ref()
    }

    /// Sets the value of ValidateShareScopeNotAliased
    pub fn set_validate_share_scope_not_aliased(&mut self, value: bool) {
        self.validate_share_scope_not_aliased = Some(value);
    }

    /// Gets the value of ValidateShareScopeNotAliased
    pub fn get_validate_share_scope_not_aliased(&self) -> Option<&bool> {
        self.validate_share_scope_not_aliased.as_ref()
    }

    /// Sets the value of ValidateTargetName
    pub fn set_validate_target_name(&mut self, value: bool) {
        self.validate_target_name = Some(value);
    }

    /// Gets the value of ValidateTargetName
    pub fn get_validate_target_name(&self) -> Option<&bool> {
        self.validate_target_name.as_ref()
    }

/// 

    /// * `output` -  (MSFT_SmbServerConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_configuration(&self, output: &mut MSFT_SmbServerConfiguration) -> Result<(), WmiError> {

        let result = self.invoke_method("GetConfiguration", &[])?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `all` -  (bool)
    /// * `announce_comment` -  (bool)
    /// * `announce_server` -  (bool)
    /// * `asynchronous_credits` -  (bool)
    /// * `audit_client_certificate_access` -  (bool)
    /// * `audit_client_does_not_support_encryption` -  (bool)
    /// * `audit_client_does_not_support_signing` -  (bool)
    /// * `audit_insecure_guest_logon` -  (bool)
    /// * `audit_smb1_access` -  (bool)
    /// * `auto_share_server` -  (bool)
    /// * `auto_share_workstation` -  (bool)
    /// * `cached_open_limit` -  (bool)
    /// * `disable_compression` -  (bool)
    /// * `disable_smb_encryption_on_secure_connection` -  (bool)
    /// * `durable_handle_v2_timeout_in_seconds` -  (bool)
    /// * `enable_auth_rate_limiter` -  (bool)
    /// * `enable_directory_handle_leasing` -  (bool)
    /// * `enable_downlevel_timewarp` -  (bool)
    /// * `enable_leasing` -  (bool)
    /// * `enable_mailslots` -  (bool)
    /// * `enable_multi_channel` -  (bool)
    /// * `enable_oplocks` -  (bool)
    /// * `enable_smb2_protocol` -  (bool)
    /// * `enable_smbquic` -  (bool)
    /// * `enable_strict_name_checking` -  (bool)
    /// * `encrypt_data` -  (bool)
    /// * `encryption_ciphers` -  (bool)
    /// * `irp_stack_size` -  (bool)
    /// * `keep_alive_time` -  (bool)
    /// * `max_channel_per_session` -  (bool)
    /// * `max_mpx_count` -  (bool)
    /// * `max_session_per_connection` -  (bool)
    /// * `max_threads_per_queue` -  (bool)
    /// * `max_work_items` -  (bool)
    /// * `null_session_shares` -  (bool)
    /// * `oplock_break_wait` -  (bool)
    /// * `pending_client_timeout_in_seconds` -  (bool)
    /// * `reject_unencrypted_access` -  (bool)
    /// * `request_compression` -  (bool)
    /// * `restrict_namedpipe_access_via_quic` -  (bool)
    /// * `server_hidden` -  (bool)
    /// * `smb2_credits_max` -  (bool)
    /// * `smb2_credits_min` -  (bool)
    /// * `smb2_dialect_max` -  (bool)
    /// * `smb2_dialect_min` -  (bool)
    /// * `smb_server_name_hardening_level` -  (bool)
    /// * `treat_host_as_stable_storage` -  (bool)
    /// * `validate_alias_not_circular` -  (bool)
    /// * `validate_share_scope` -  (bool)
    /// * `validate_share_scope_not_aliased` -  (bool)
    /// * `validate_target_name` -  (bool)

    /// * `output` -  (MSFT_SmbServerConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset_configuration(&self, output: &mut MSFT_SmbServerConfiguration, all: Option<bool>, announce_server: Option<bool>, asynchronous_credits: Option<bool>, auto_share_server: Option<bool>, auto_share_workstation: Option<bool>, cached_open_limit: Option<bool>, announce_comment: Option<bool>, enable_auth_rate_limiter: Option<bool>, enable_downlevel_timewarp: Option<bool>, enable_leasing: Option<bool>, enable_mailslots: Option<bool>, enable_multi_channel: Option<bool>, enable_strict_name_checking: Option<bool>, disable_compression: Option<bool>, durable_handle_v2_timeout_in_seconds: Option<bool>, enable_oplocks: Option<bool>, server_hidden: Option<bool>, irp_stack_size: Option<bool>, keep_alive_time: Option<bool>, max_channel_per_session: Option<bool>, max_mpx_count: Option<bool>, max_session_per_connection: Option<bool>, max_threads_per_queue: Option<bool>, max_work_items: Option<bool>, null_session_shares: Option<bool>, oplock_break_wait: Option<bool>, pending_client_timeout_in_seconds: Option<bool>, enable_smb2_protocol: Option<bool>, smb2_credits_max: Option<bool>, smb2_credits_min: Option<bool>, smb_server_name_hardening_level: Option<bool>, treat_host_as_stable_storage: Option<bool>, validate_alias_not_circular: Option<bool>, validate_share_scope: Option<bool>, validate_share_scope_not_aliased: Option<bool>, validate_target_name: Option<bool>, encrypt_data: Option<bool>, reject_unencrypted_access: Option<bool>, audit_smb1_access: Option<bool>, disable_smb_encryption_on_secure_connection: Option<bool>, request_compression: Option<bool>, restrict_namedpipe_access_via_quic: Option<bool>, enable_smbquic: Option<bool>, enable_directory_handle_leasing: Option<bool>, encryption_ciphers: Option<bool>, smb2_dialect_max: Option<bool>, smb2_dialect_min: Option<bool>, audit_client_certificate_access: Option<bool>, audit_client_does_not_support_encryption: Option<bool>, audit_client_does_not_support_signing: Option<bool>, audit_insecure_guest_logon: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = all {
            args.push(MethodParameter { name: "All".to_string(), value: val.into() });
        }
        if let Some(val) = announce_server {
            args.push(MethodParameter { name: "AnnounceServer".to_string(), value: val.into() });
        }
        if let Some(val) = asynchronous_credits {
            args.push(MethodParameter { name: "AsynchronousCredits".to_string(), value: val.into() });
        }
        if let Some(val) = auto_share_server {
            args.push(MethodParameter { name: "AutoShareServer".to_string(), value: val.into() });
        }
        if let Some(val) = auto_share_workstation {
            args.push(MethodParameter { name: "AutoShareWorkstation".to_string(), value: val.into() });
        }
        if let Some(val) = cached_open_limit {
            args.push(MethodParameter { name: "CachedOpenLimit".to_string(), value: val.into() });
        }
        if let Some(val) = announce_comment {
            args.push(MethodParameter { name: "AnnounceComment".to_string(), value: val.into() });
        }
        if let Some(val) = enable_auth_rate_limiter {
            args.push(MethodParameter { name: "EnableAuthRateLimiter".to_string(), value: val.into() });
        }
        if let Some(val) = enable_downlevel_timewarp {
            args.push(MethodParameter { name: "EnableDownlevelTimewarp".to_string(), value: val.into() });
        }
        if let Some(val) = enable_leasing {
            args.push(MethodParameter { name: "EnableLeasing".to_string(), value: val.into() });
        }
        if let Some(val) = enable_mailslots {
            args.push(MethodParameter { name: "EnableMailslots".to_string(), value: val.into() });
        }
        if let Some(val) = enable_multi_channel {
            args.push(MethodParameter { name: "EnableMultiChannel".to_string(), value: val.into() });
        }
        if let Some(val) = enable_strict_name_checking {
            args.push(MethodParameter { name: "EnableStrictNameChecking".to_string(), value: val.into() });
        }
        if let Some(val) = disable_compression {
            args.push(MethodParameter { name: "DisableCompression".to_string(), value: val.into() });
        }
        if let Some(val) = durable_handle_v2_timeout_in_seconds {
            args.push(MethodParameter { name: "DurableHandleV2TimeoutInSeconds".to_string(), value: val.into() });
        }
        if let Some(val) = enable_oplocks {
            args.push(MethodParameter { name: "EnableOplocks".to_string(), value: val.into() });
        }
        if let Some(val) = server_hidden {
            args.push(MethodParameter { name: "ServerHidden".to_string(), value: val.into() });
        }
        if let Some(val) = irp_stack_size {
            args.push(MethodParameter { name: "IrpStackSize".to_string(), value: val.into() });
        }
        if let Some(val) = keep_alive_time {
            args.push(MethodParameter { name: "KeepAliveTime".to_string(), value: val.into() });
        }
        if let Some(val) = max_channel_per_session {
            args.push(MethodParameter { name: "MaxChannelPerSession".to_string(), value: val.into() });
        }
        if let Some(val) = max_mpx_count {
            args.push(MethodParameter { name: "MaxMpxCount".to_string(), value: val.into() });
        }
        if let Some(val) = max_session_per_connection {
            args.push(MethodParameter { name: "MaxSessionPerConnection".to_string(), value: val.into() });
        }
        if let Some(val) = max_threads_per_queue {
            args.push(MethodParameter { name: "MaxThreadsPerQueue".to_string(), value: val.into() });
        }
        if let Some(val) = max_work_items {
            args.push(MethodParameter { name: "MaxWorkItems".to_string(), value: val.into() });
        }
        if let Some(val) = null_session_shares {
            args.push(MethodParameter { name: "NullSessionShares".to_string(), value: val.into() });
        }
        if let Some(val) = oplock_break_wait {
            args.push(MethodParameter { name: "OplockBreakWait".to_string(), value: val.into() });
        }
        if let Some(val) = pending_client_timeout_in_seconds {
            args.push(MethodParameter { name: "PendingClientTimeoutInSeconds".to_string(), value: val.into() });
        }
        if let Some(val) = enable_smb2_protocol {
            args.push(MethodParameter { name: "EnableSMB2Protocol".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_credits_max {
            args.push(MethodParameter { name: "Smb2CreditsMax".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_credits_min {
            args.push(MethodParameter { name: "Smb2CreditsMin".to_string(), value: val.into() });
        }
        if let Some(val) = smb_server_name_hardening_level {
            args.push(MethodParameter { name: "SmbServerNameHardeningLevel".to_string(), value: val.into() });
        }
        if let Some(val) = treat_host_as_stable_storage {
            args.push(MethodParameter { name: "TreatHostAsStableStorage".to_string(), value: val.into() });
        }
        if let Some(val) = validate_alias_not_circular {
            args.push(MethodParameter { name: "ValidateAliasNotCircular".to_string(), value: val.into() });
        }
        if let Some(val) = validate_share_scope {
            args.push(MethodParameter { name: "ValidateShareScope".to_string(), value: val.into() });
        }
        if let Some(val) = validate_share_scope_not_aliased {
            args.push(MethodParameter { name: "ValidateShareScopeNotAliased".to_string(), value: val.into() });
        }
        if let Some(val) = validate_target_name {
            args.push(MethodParameter { name: "ValidateTargetName".to_string(), value: val.into() });
        }
        if let Some(val) = encrypt_data {
            args.push(MethodParameter { name: "EncryptData".to_string(), value: val.into() });
        }
        if let Some(val) = reject_unencrypted_access {
            args.push(MethodParameter { name: "RejectUnencryptedAccess".to_string(), value: val.into() });
        }
        if let Some(val) = audit_smb1_access {
            args.push(MethodParameter { name: "AuditSmb1Access".to_string(), value: val.into() });
        }
        if let Some(val) = disable_smb_encryption_on_secure_connection {
            args.push(MethodParameter { name: "DisableSmbEncryptionOnSecureConnection".to_string(), value: val.into() });
        }
        if let Some(val) = request_compression {
            args.push(MethodParameter { name: "RequestCompression".to_string(), value: val.into() });
        }
        if let Some(val) = restrict_namedpipe_access_via_quic {
            args.push(MethodParameter { name: "RestrictNamedpipeAccessViaQuic".to_string(), value: val.into() });
        }
        if let Some(val) = enable_smbquic {
            args.push(MethodParameter { name: "EnableSMBQUIC".to_string(), value: val.into() });
        }
        if let Some(val) = enable_directory_handle_leasing {
            args.push(MethodParameter { name: "EnableDirectoryHandleLeasing".to_string(), value: val.into() });
        }
        if let Some(val) = encryption_ciphers {
            args.push(MethodParameter { name: "EncryptionCiphers".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_dialect_max {
            args.push(MethodParameter { name: "Smb2DialectMax".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_dialect_min {
            args.push(MethodParameter { name: "Smb2DialectMin".to_string(), value: val.into() });
        }
        if let Some(val) = audit_client_certificate_access {
            args.push(MethodParameter { name: "AuditClientCertificateAccess".to_string(), value: val.into() });
        }
        if let Some(val) = audit_client_does_not_support_encryption {
            args.push(MethodParameter { name: "AuditClientDoesNotSupportEncryption".to_string(), value: val.into() });
        }
        if let Some(val) = audit_client_does_not_support_signing {
            args.push(MethodParameter { name: "AuditClientDoesNotSupportSigning".to_string(), value: val.into() });
        }
        if let Some(val) = audit_insecure_guest_logon {
            args.push(MethodParameter { name: "AuditInsecureGuestLogon".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ResetConfiguration", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `announce_comment` -  (String)
    /// * `announce_server` -  (bool)
    /// * `asynchronous_credits` -  (u32)
    /// * `audit_client_certificate_access` -  (bool)
    /// * `audit_client_does_not_support_encryption` -  (bool)
    /// * `audit_client_does_not_support_signing` -  (bool)
    /// * `audit_insecure_guest_logon` -  (bool)
    /// * `audit_smb1_access` -  (bool)
    /// * `auto_disconnect_timeout_in_minutes_v1` -  (u32)
    /// * `auto_disconnect_timeout_in_seconds_v2` -  (u32)
    /// * `auto_share_server` -  (bool)
    /// * `auto_share_workstation` -  (bool)
    /// * `cached_open_limit` -  (u32)
    /// * `disable_compression` -  (bool)
    /// * `disable_smb_encryption_on_secure_connection` -  (bool)
    /// * `durable_handle_v2_timeout_in_seconds` -  (u32)
    /// * `enable_authenticate_user_sharing` -  (bool)
    /// * `enable_auth_rate_limiter` -  (bool)
    /// * `enable_directory_handle_leasing` -  (bool)
    /// * `enable_downlevel_timewarp` -  (bool)
    /// * `enable_forced_logoff` -  (bool)
    /// * `enable_leasing` -  (bool)
    /// * `enable_mailslots` -  (bool)
    /// * `enable_multi_channel` -  (bool)
    /// * `enable_oplocks` -  (bool)
    /// * `enable_security_signature` -  (bool)
    /// * `enable_smb1_protocol` -  (bool)
    /// * `enable_smb2_protocol` -  (bool)
    /// * `enable_smbquic` -  (bool)
    /// * `enable_strict_name_checking` -  (bool)
    /// * `encrypt_data` -  (bool)
    /// * `encryption_ciphers` -  (String)
    /// * `invalid_authentication_delay_time_in_ms` -  (u32)
    /// * `irp_stack_size` -  (u32)
    /// * `keep_alive_time` -  (u32)
    /// * `max_channel_per_session` -  (u32)
    /// * `max_mpx_count` -  (u32)
    /// * `max_session_per_connection` -  (u32)
    /// * `max_threads_per_queue` -  (u32)
    /// * `max_work_items` -  (u32)
    /// * `null_session_pipes` -  (String)
    /// * `null_session_shares` -  (String)
    /// * `oplock_break_wait` -  (u32)
    /// * `pending_client_timeout_in_seconds` -  (u32)
    /// * `reject_unencrypted_access` -  (bool)
    /// * `request_compression` -  (bool)
    /// * `require_security_signature` -  (bool)
    /// * `restrict_namedpipe_access_via_quic` -  (bool)
    /// * `server_hidden` -  (bool)
    /// * `smb2_credits_max` -  (u32)
    /// * `smb2_credits_min` -  (u32)
    /// * `smb2_dialect_max` -  (u32)
    /// * `smb2_dialect_min` -  (u32)
    /// * `smb_server_name_hardening_level` -  (u32)
    /// * `treat_host_as_stable_storage` -  (bool)
    /// * `validate_alias_not_circular` -  (bool)
    /// * `validate_share_scope` -  (bool)
    /// * `validate_share_scope_not_aliased` -  (bool)
    /// * `validate_target_name` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_configuration(&self, announce_server: bool, asynchronous_credits: u32, auto_share_server: bool, auto_share_workstation: bool, cached_open_limit: u32, announce_comment: &String, enable_downlevel_timewarp: bool, enable_leasing: bool, enable_mailslots: bool, enable_multi_channel: bool, enable_strict_name_checking: bool, auto_disconnect_timeout_in_minutes_v1: u32, auto_disconnect_timeout_in_seconds_v2: u32, disable_compression: bool, durable_handle_v2_timeout_in_seconds: u32, enable_authenticate_user_sharing: bool, enable_auth_rate_limiter: bool, enable_forced_logoff: bool, enable_oplocks: bool, enable_security_signature: bool, server_hidden: bool, irp_stack_size: u32, keep_alive_time: u32, max_channel_per_session: u32, max_mpx_count: u32, max_session_per_connection: u32, max_threads_per_queue: u32, max_work_items: u32, null_session_pipes: &String, null_session_shares: &String, oplock_break_wait: u32, pending_client_timeout_in_seconds: u32, request_compression: bool, require_security_signature: bool, enable_smb1_protocol: bool, enable_smb2_protocol: bool, smb2_credits_max: u32, smb2_credits_min: u32, smb_server_name_hardening_level: u32, treat_host_as_stable_storage: bool, validate_alias_not_circular: bool, validate_share_scope: bool, validate_share_scope_not_aliased: bool, validate_target_name: bool, encrypt_data: bool, reject_unencrypted_access: bool, audit_smb1_access: bool, disable_smb_encryption_on_secure_connection: bool, restrict_namedpipe_access_via_quic: bool, enable_smbquic: bool, enable_directory_handle_leasing: bool, encryption_ciphers: &String, invalid_authentication_delay_time_in_ms: u32, smb2_dialect_max: u32, smb2_dialect_min: u32, audit_client_certificate_access: bool, audit_client_does_not_support_encryption: bool, audit_client_does_not_support_signing: bool, audit_insecure_guest_logon: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AnnounceServer".to_string(), value: announce_server.into() });
        args.push(MethodParameter { name: "AsynchronousCredits".to_string(), value: asynchronous_credits.into() });
        args.push(MethodParameter { name: "AutoShareServer".to_string(), value: auto_share_server.into() });
        args.push(MethodParameter { name: "AutoShareWorkstation".to_string(), value: auto_share_workstation.into() });
        args.push(MethodParameter { name: "CachedOpenLimit".to_string(), value: cached_open_limit.into() });
        args.push(MethodParameter { name: "AnnounceComment".to_string(), value: announce_comment.into() });
        args.push(MethodParameter { name: "EnableDownlevelTimewarp".to_string(), value: enable_downlevel_timewarp.into() });
        args.push(MethodParameter { name: "EnableLeasing".to_string(), value: enable_leasing.into() });
        args.push(MethodParameter { name: "EnableMailslots".to_string(), value: enable_mailslots.into() });
        args.push(MethodParameter { name: "EnableMultiChannel".to_string(), value: enable_multi_channel.into() });
        args.push(MethodParameter { name: "EnableStrictNameChecking".to_string(), value: enable_strict_name_checking.into() });
        args.push(MethodParameter { name: "AutoDisconnectTimeoutInMinutesV1".to_string(), value: auto_disconnect_timeout_in_minutes_v1.into() });
        args.push(MethodParameter { name: "AutoDisconnectTimeoutInSecondsV2".to_string(), value: auto_disconnect_timeout_in_seconds_v2.into() });
        args.push(MethodParameter { name: "DisableCompression".to_string(), value: disable_compression.into() });
        args.push(MethodParameter { name: "DurableHandleV2TimeoutInSeconds".to_string(), value: durable_handle_v2_timeout_in_seconds.into() });
        args.push(MethodParameter { name: "EnableAuthenticateUserSharing".to_string(), value: enable_authenticate_user_sharing.into() });
        args.push(MethodParameter { name: "EnableAuthRateLimiter".to_string(), value: enable_auth_rate_limiter.into() });
        args.push(MethodParameter { name: "EnableForcedLogoff".to_string(), value: enable_forced_logoff.into() });
        args.push(MethodParameter { name: "EnableOplocks".to_string(), value: enable_oplocks.into() });
        args.push(MethodParameter { name: "EnableSecuritySignature".to_string(), value: enable_security_signature.into() });
        args.push(MethodParameter { name: "ServerHidden".to_string(), value: server_hidden.into() });
        args.push(MethodParameter { name: "IrpStackSize".to_string(), value: irp_stack_size.into() });
        args.push(MethodParameter { name: "KeepAliveTime".to_string(), value: keep_alive_time.into() });
        args.push(MethodParameter { name: "MaxChannelPerSession".to_string(), value: max_channel_per_session.into() });
        args.push(MethodParameter { name: "MaxMpxCount".to_string(), value: max_mpx_count.into() });
        args.push(MethodParameter { name: "MaxSessionPerConnection".to_string(), value: max_session_per_connection.into() });
        args.push(MethodParameter { name: "MaxThreadsPerQueue".to_string(), value: max_threads_per_queue.into() });
        args.push(MethodParameter { name: "MaxWorkItems".to_string(), value: max_work_items.into() });
        args.push(MethodParameter { name: "NullSessionPipes".to_string(), value: null_session_pipes.into() });
        args.push(MethodParameter { name: "NullSessionShares".to_string(), value: null_session_shares.into() });
        args.push(MethodParameter { name: "OplockBreakWait".to_string(), value: oplock_break_wait.into() });
        args.push(MethodParameter { name: "PendingClientTimeoutInSeconds".to_string(), value: pending_client_timeout_in_seconds.into() });
        args.push(MethodParameter { name: "RequestCompression".to_string(), value: request_compression.into() });
        args.push(MethodParameter { name: "RequireSecuritySignature".to_string(), value: require_security_signature.into() });
        args.push(MethodParameter { name: "EnableSMB1Protocol".to_string(), value: enable_smb1_protocol.into() });
        args.push(MethodParameter { name: "EnableSMB2Protocol".to_string(), value: enable_smb2_protocol.into() });
        args.push(MethodParameter { name: "Smb2CreditsMax".to_string(), value: smb2_credits_max.into() });
        args.push(MethodParameter { name: "Smb2CreditsMin".to_string(), value: smb2_credits_min.into() });
        args.push(MethodParameter { name: "SmbServerNameHardeningLevel".to_string(), value: smb_server_name_hardening_level.into() });
        args.push(MethodParameter { name: "TreatHostAsStableStorage".to_string(), value: treat_host_as_stable_storage.into() });
        args.push(MethodParameter { name: "ValidateAliasNotCircular".to_string(), value: validate_alias_not_circular.into() });
        args.push(MethodParameter { name: "ValidateShareScope".to_string(), value: validate_share_scope.into() });
        args.push(MethodParameter { name: "ValidateShareScopeNotAliased".to_string(), value: validate_share_scope_not_aliased.into() });
        args.push(MethodParameter { name: "ValidateTargetName".to_string(), value: validate_target_name.into() });
        args.push(MethodParameter { name: "EncryptData".to_string(), value: encrypt_data.into() });
        args.push(MethodParameter { name: "RejectUnencryptedAccess".to_string(), value: reject_unencrypted_access.into() });
        args.push(MethodParameter { name: "AuditSmb1Access".to_string(), value: audit_smb1_access.into() });
        args.push(MethodParameter { name: "DisableSmbEncryptionOnSecureConnection".to_string(), value: disable_smb_encryption_on_secure_connection.into() });
        args.push(MethodParameter { name: "RestrictNamedpipeAccessViaQuic".to_string(), value: restrict_namedpipe_access_via_quic.into() });
        args.push(MethodParameter { name: "EnableSMBQUIC".to_string(), value: enable_smbquic.into() });
        args.push(MethodParameter { name: "EnableDirectoryHandleLeasing".to_string(), value: enable_directory_handle_leasing.into() });
        args.push(MethodParameter { name: "EncryptionCiphers".to_string(), value: encryption_ciphers.into() });
        args.push(MethodParameter { name: "InvalidAuthenticationDelayTimeInMs".to_string(), value: invalid_authentication_delay_time_in_ms.into() });
        args.push(MethodParameter { name: "Smb2DialectMax".to_string(), value: smb2_dialect_max.into() });
        args.push(MethodParameter { name: "Smb2DialectMin".to_string(), value: smb2_dialect_min.into() });
        args.push(MethodParameter { name: "AuditClientCertificateAccess".to_string(), value: audit_client_certificate_access.into() });
        args.push(MethodParameter { name: "AuditClientDoesNotSupportEncryption".to_string(), value: audit_client_does_not_support_encryption.into() });
        args.push(MethodParameter { name: "AuditClientDoesNotSupportSigning".to_string(), value: audit_client_does_not_support_signing.into() });
        args.push(MethodParameter { name: "AuditInsecureGuestLogon".to_string(), value: audit_insecure_guest_logon.into() });
        self.invoke_method("SetConfiguration", &args)

    }

}

