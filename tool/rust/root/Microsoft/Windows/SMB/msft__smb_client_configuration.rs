// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbClientConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbClientConfiguration {

/// 
    #[serde(rename = "AuditInsecureGuestLogon")]
    pub audit_insecure_guest_logon: Option<bool>,

/// 
    #[serde(rename = "AuditServerDoesNotSupportEncryption")]
    pub audit_server_does_not_support_encryption: Option<bool>,

/// 
    #[serde(rename = "AuditServerDoesNotSupportSigning")]
    pub audit_server_does_not_support_signing: Option<bool>,

/// 
    #[serde(rename = "BlockNTLM")]
    pub block_ntlm: Option<bool>,

/// 
    #[serde(rename = "BlockNTLMServerExceptionList")]
    pub block_ntlmserver_exception_list: Option<String>,

/// 
    #[serde(rename = "CompressibilitySamplingSize")]
    pub compressibility_sampling_size: Option<u64>,

/// 
    #[serde(rename = "CompressibleThreshold")]
    pub compressible_threshold: Option<u64>,

/// 
    #[serde(rename = "ConnectionCountPerRssNetworkInterface")]
    pub connection_count_per_rss_network_interface: Option<u32>,

/// 
    #[serde(rename = "DirectoryCacheEntriesMax")]
    pub directory_cache_entries_max: Option<u32>,

/// 
    #[serde(rename = "DirectoryCacheEntrySizeMax")]
    pub directory_cache_entry_size_max: Option<u32>,

/// 
    #[serde(rename = "DirectoryCacheLifetime")]
    pub directory_cache_lifetime: Option<u32>,

/// 
    #[serde(rename = "DisableCompression")]
    pub disable_compression: Option<bool>,

/// 
    #[serde(rename = "DisabledSMBQUICServerExceptionList")]
    pub disabled_smbquicserver_exception_list: Option<String>,

/// 
    #[serde(rename = "DormantFileLimit")]
    pub dormant_file_limit: Option<u32>,

/// 
    #[serde(rename = "EnableBandwidthThrottling")]
    pub enable_bandwidth_throttling: Option<bool>,

/// 
    #[serde(rename = "EnableByteRangeLockingOnReadOnlyFiles")]
    pub enable_byte_range_locking_on_read_only_files: Option<bool>,

/// 
    #[serde(rename = "EnableCompressibilitySampling")]
    pub enable_compressibility_sampling: Option<bool>,

/// 
    #[serde(rename = "EnableInsecureGuestLogons")]
    pub enable_insecure_guest_logons: Option<bool>,

/// 
    #[serde(rename = "EnableLargeMtu")]
    pub enable_large_mtu: Option<bool>,

/// 
    #[serde(rename = "EnableLoadBalanceScaleOut")]
    pub enable_load_balance_scale_out: Option<bool>,

/// 
    #[serde(rename = "EnableMailslots")]
    pub enable_mailslots: Option<bool>,

/// 
    #[serde(rename = "EnableMultiChannel")]
    pub enable_multi_channel: Option<bool>,

/// 
    #[serde(rename = "EnableSecuritySignature")]
    pub enable_security_signature: Option<bool>,

/// 
    #[serde(rename = "EnableSMBQUIC")]
    pub enable_smbquic: Option<bool>,

/// 
    #[serde(rename = "EncryptionCiphers")]
    pub encryption_ciphers: Option<String>,

/// 
    #[serde(rename = "ExtendedSessionTimeout")]
    pub extended_session_timeout: Option<u32>,

/// 
    #[serde(rename = "FileInfoCacheEntriesMax")]
    pub file_info_cache_entries_max: Option<u32>,

/// 
    #[serde(rename = "FileInfoCacheLifetime")]
    pub file_info_cache_lifetime: Option<u32>,

/// 
    #[serde(rename = "FileNotFoundCacheEntriesMax")]
    pub file_not_found_cache_entries_max: Option<u32>,

/// 
    #[serde(rename = "FileNotFoundCacheLifetime")]
    pub file_not_found_cache_lifetime: Option<u32>,

/// 
    #[serde(rename = "ForceSMBEncryptionOverQuic")]
    pub force_smbencryption_over_quic: Option<bool>,

/// 
    #[serde(rename = "InvalidAuthenticationCacheLifetime")]
    pub invalid_authentication_cache_lifetime: Option<u32>,

/// 
    #[serde(rename = "KeepConn")]
    pub keep_conn: Option<u32>,

/// 
    #[serde(rename = "MaxCmds")]
    pub max_cmds: Option<u32>,

/// 
    #[serde(rename = "MaximumConnectionCountPerServer")]
    pub maximum_connection_count_per_server: Option<u32>,

/// 
    #[serde(rename = "OplocksDisabled")]
    pub oplocks_disabled: Option<bool>,

/// 
    #[serde(rename = "RequestCompression")]
    pub request_compression: Option<bool>,

/// 
    #[serde(rename = "RequireEncryption")]
    pub require_encryption: Option<bool>,

/// 
    #[serde(rename = "RequireSecuritySignature")]
    pub require_security_signature: Option<bool>,

/// 
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<u32>,

/// 
    #[serde(rename = "SkipCertificateCheck")]
    pub skip_certificate_check: Option<bool>,

/// 
    #[serde(rename = "Smb2DialectMax")]
    pub smb2_dialect_max: Option<SmbClientConfiguration_Smb2DialectMax>,

/// 
    #[serde(rename = "Smb2DialectMin")]
    pub smb2_dialect_min: Option<SmbClientConfiguration_Smb2DialectMin>,

/// 
    #[serde(rename = "UseOpportunisticLocking")]
    pub use_opportunistic_locking: Option<bool>,

/// 
    #[serde(rename = "WindowSizeThreshold")]
    pub window_size_threshold: Option<u32>,
}

impl MSFT_SmbClientConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            audit_insecure_guest_logon: None,
            audit_server_does_not_support_encryption: None,
            audit_server_does_not_support_signing: None,
            block_ntlm: None,
            block_ntlmserver_exception_list: None,
            compressibility_sampling_size: None,
            compressible_threshold: None,
            connection_count_per_rss_network_interface: None,
            directory_cache_entries_max: None,
            directory_cache_entry_size_max: None,
            directory_cache_lifetime: None,
            disable_compression: None,
            disabled_smbquicserver_exception_list: None,
            dormant_file_limit: None,
            enable_bandwidth_throttling: None,
            enable_byte_range_locking_on_read_only_files: None,
            enable_compressibility_sampling: None,
            enable_insecure_guest_logons: None,
            enable_large_mtu: None,
            enable_load_balance_scale_out: None,
            enable_mailslots: None,
            enable_multi_channel: None,
            enable_security_signature: None,
            enable_smbquic: None,
            encryption_ciphers: None,
            extended_session_timeout: None,
            file_info_cache_entries_max: None,
            file_info_cache_lifetime: None,
            file_not_found_cache_entries_max: None,
            file_not_found_cache_lifetime: None,
            force_smbencryption_over_quic: None,
            invalid_authentication_cache_lifetime: None,
            keep_conn: None,
            max_cmds: None,
            maximum_connection_count_per_server: None,
            oplocks_disabled: None,
            request_compression: None,
            require_encryption: None,
            require_security_signature: None,
            session_timeout: None,
            skip_certificate_check: None,
            smb2_dialect_max: None,
            smb2_dialect_min: None,
            use_opportunistic_locking: None,
            window_size_threshold: None,
        }
    }


    /// Sets the value of AuditInsecureGuestLogon
    pub fn set_audit_insecure_guest_logon(&mut self, value: bool) {
        self.audit_insecure_guest_logon = Some(value);
    }

    /// Gets the value of AuditInsecureGuestLogon
    pub fn get_audit_insecure_guest_logon(&self) -> Option<&bool> {
        self.audit_insecure_guest_logon.as_ref()
    }

    /// Sets the value of AuditServerDoesNotSupportEncryption
    pub fn set_audit_server_does_not_support_encryption(&mut self, value: bool) {
        self.audit_server_does_not_support_encryption = Some(value);
    }

    /// Gets the value of AuditServerDoesNotSupportEncryption
    pub fn get_audit_server_does_not_support_encryption(&self) -> Option<&bool> {
        self.audit_server_does_not_support_encryption.as_ref()
    }

    /// Sets the value of AuditServerDoesNotSupportSigning
    pub fn set_audit_server_does_not_support_signing(&mut self, value: bool) {
        self.audit_server_does_not_support_signing = Some(value);
    }

    /// Gets the value of AuditServerDoesNotSupportSigning
    pub fn get_audit_server_does_not_support_signing(&self) -> Option<&bool> {
        self.audit_server_does_not_support_signing.as_ref()
    }

    /// Sets the value of BlockNTLM
    pub fn set_block_ntlm(&mut self, value: bool) {
        self.block_ntlm = Some(value);
    }

    /// Gets the value of BlockNTLM
    pub fn get_block_ntlm(&self) -> Option<&bool> {
        self.block_ntlm.as_ref()
    }

    /// Sets the value of BlockNTLMServerExceptionList
    pub fn set_block_ntlmserver_exception_list(&mut self, value: String) {
        self.block_ntlmserver_exception_list = Some(value);
    }

    /// Gets the value of BlockNTLMServerExceptionList
    pub fn get_block_ntlmserver_exception_list(&self) -> Option<&String> {
        self.block_ntlmserver_exception_list.as_ref()
    }

    /// Sets the value of CompressibilitySamplingSize
    pub fn set_compressibility_sampling_size(&mut self, value: u64) {
        self.compressibility_sampling_size = Some(value);
    }

    /// Gets the value of CompressibilitySamplingSize
    pub fn get_compressibility_sampling_size(&self) -> Option<&u64> {
        self.compressibility_sampling_size.as_ref()
    }

    /// Sets the value of CompressibleThreshold
    pub fn set_compressible_threshold(&mut self, value: u64) {
        self.compressible_threshold = Some(value);
    }

    /// Gets the value of CompressibleThreshold
    pub fn get_compressible_threshold(&self) -> Option<&u64> {
        self.compressible_threshold.as_ref()
    }

    /// Sets the value of ConnectionCountPerRssNetworkInterface
    pub fn set_connection_count_per_rss_network_interface(&mut self, value: u32) {
        self.connection_count_per_rss_network_interface = Some(value);
    }

    /// Gets the value of ConnectionCountPerRssNetworkInterface
    pub fn get_connection_count_per_rss_network_interface(&self) -> Option<&u32> {
        self.connection_count_per_rss_network_interface.as_ref()
    }

    /// Sets the value of DirectoryCacheEntriesMax
    pub fn set_directory_cache_entries_max(&mut self, value: u32) {
        self.directory_cache_entries_max = Some(value);
    }

    /// Gets the value of DirectoryCacheEntriesMax
    pub fn get_directory_cache_entries_max(&self) -> Option<&u32> {
        self.directory_cache_entries_max.as_ref()
    }

    /// Sets the value of DirectoryCacheEntrySizeMax
    pub fn set_directory_cache_entry_size_max(&mut self, value: u32) {
        self.directory_cache_entry_size_max = Some(value);
    }

    /// Gets the value of DirectoryCacheEntrySizeMax
    pub fn get_directory_cache_entry_size_max(&self) -> Option<&u32> {
        self.directory_cache_entry_size_max.as_ref()
    }

    /// Sets the value of DirectoryCacheLifetime
    pub fn set_directory_cache_lifetime(&mut self, value: u32) {
        self.directory_cache_lifetime = Some(value);
    }

    /// Gets the value of DirectoryCacheLifetime
    pub fn get_directory_cache_lifetime(&self) -> Option<&u32> {
        self.directory_cache_lifetime.as_ref()
    }

    /// Sets the value of DisableCompression
    pub fn set_disable_compression(&mut self, value: bool) {
        self.disable_compression = Some(value);
    }

    /// Gets the value of DisableCompression
    pub fn get_disable_compression(&self) -> Option<&bool> {
        self.disable_compression.as_ref()
    }

    /// Sets the value of DisabledSMBQUICServerExceptionList
    pub fn set_disabled_smbquicserver_exception_list(&mut self, value: String) {
        self.disabled_smbquicserver_exception_list = Some(value);
    }

    /// Gets the value of DisabledSMBQUICServerExceptionList
    pub fn get_disabled_smbquicserver_exception_list(&self) -> Option<&String> {
        self.disabled_smbquicserver_exception_list.as_ref()
    }

    /// Sets the value of DormantFileLimit
    pub fn set_dormant_file_limit(&mut self, value: u32) {
        self.dormant_file_limit = Some(value);
    }

    /// Gets the value of DormantFileLimit
    pub fn get_dormant_file_limit(&self) -> Option<&u32> {
        self.dormant_file_limit.as_ref()
    }

    /// Sets the value of EnableBandwidthThrottling
    pub fn set_enable_bandwidth_throttling(&mut self, value: bool) {
        self.enable_bandwidth_throttling = Some(value);
    }

    /// Gets the value of EnableBandwidthThrottling
    pub fn get_enable_bandwidth_throttling(&self) -> Option<&bool> {
        self.enable_bandwidth_throttling.as_ref()
    }

    /// Sets the value of EnableByteRangeLockingOnReadOnlyFiles
    pub fn set_enable_byte_range_locking_on_read_only_files(&mut self, value: bool) {
        self.enable_byte_range_locking_on_read_only_files = Some(value);
    }

    /// Gets the value of EnableByteRangeLockingOnReadOnlyFiles
    pub fn get_enable_byte_range_locking_on_read_only_files(&self) -> Option<&bool> {
        self.enable_byte_range_locking_on_read_only_files.as_ref()
    }

    /// Sets the value of EnableCompressibilitySampling
    pub fn set_enable_compressibility_sampling(&mut self, value: bool) {
        self.enable_compressibility_sampling = Some(value);
    }

    /// Gets the value of EnableCompressibilitySampling
    pub fn get_enable_compressibility_sampling(&self) -> Option<&bool> {
        self.enable_compressibility_sampling.as_ref()
    }

    /// Sets the value of EnableInsecureGuestLogons
    pub fn set_enable_insecure_guest_logons(&mut self, value: bool) {
        self.enable_insecure_guest_logons = Some(value);
    }

    /// Gets the value of EnableInsecureGuestLogons
    pub fn get_enable_insecure_guest_logons(&self) -> Option<&bool> {
        self.enable_insecure_guest_logons.as_ref()
    }

    /// Sets the value of EnableLargeMtu
    pub fn set_enable_large_mtu(&mut self, value: bool) {
        self.enable_large_mtu = Some(value);
    }

    /// Gets the value of EnableLargeMtu
    pub fn get_enable_large_mtu(&self) -> Option<&bool> {
        self.enable_large_mtu.as_ref()
    }

    /// Sets the value of EnableLoadBalanceScaleOut
    pub fn set_enable_load_balance_scale_out(&mut self, value: bool) {
        self.enable_load_balance_scale_out = Some(value);
    }

    /// Gets the value of EnableLoadBalanceScaleOut
    pub fn get_enable_load_balance_scale_out(&self) -> Option<&bool> {
        self.enable_load_balance_scale_out.as_ref()
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

    /// Sets the value of EnableSecuritySignature
    pub fn set_enable_security_signature(&mut self, value: bool) {
        self.enable_security_signature = Some(value);
    }

    /// Gets the value of EnableSecuritySignature
    pub fn get_enable_security_signature(&self) -> Option<&bool> {
        self.enable_security_signature.as_ref()
    }

    /// Sets the value of EnableSMBQUIC
    pub fn set_enable_smbquic(&mut self, value: bool) {
        self.enable_smbquic = Some(value);
    }

    /// Gets the value of EnableSMBQUIC
    pub fn get_enable_smbquic(&self) -> Option<&bool> {
        self.enable_smbquic.as_ref()
    }

    /// Sets the value of EncryptionCiphers
    pub fn set_encryption_ciphers(&mut self, value: String) {
        self.encryption_ciphers = Some(value);
    }

    /// Gets the value of EncryptionCiphers
    pub fn get_encryption_ciphers(&self) -> Option<&String> {
        self.encryption_ciphers.as_ref()
    }

    /// Sets the value of ExtendedSessionTimeout
    pub fn set_extended_session_timeout(&mut self, value: u32) {
        self.extended_session_timeout = Some(value);
    }

    /// Gets the value of ExtendedSessionTimeout
    pub fn get_extended_session_timeout(&self) -> Option<&u32> {
        self.extended_session_timeout.as_ref()
    }

    /// Sets the value of FileInfoCacheEntriesMax
    pub fn set_file_info_cache_entries_max(&mut self, value: u32) {
        self.file_info_cache_entries_max = Some(value);
    }

    /// Gets the value of FileInfoCacheEntriesMax
    pub fn get_file_info_cache_entries_max(&self) -> Option<&u32> {
        self.file_info_cache_entries_max.as_ref()
    }

    /// Sets the value of FileInfoCacheLifetime
    pub fn set_file_info_cache_lifetime(&mut self, value: u32) {
        self.file_info_cache_lifetime = Some(value);
    }

    /// Gets the value of FileInfoCacheLifetime
    pub fn get_file_info_cache_lifetime(&self) -> Option<&u32> {
        self.file_info_cache_lifetime.as_ref()
    }

    /// Sets the value of FileNotFoundCacheEntriesMax
    pub fn set_file_not_found_cache_entries_max(&mut self, value: u32) {
        self.file_not_found_cache_entries_max = Some(value);
    }

    /// Gets the value of FileNotFoundCacheEntriesMax
    pub fn get_file_not_found_cache_entries_max(&self) -> Option<&u32> {
        self.file_not_found_cache_entries_max.as_ref()
    }

    /// Sets the value of FileNotFoundCacheLifetime
    pub fn set_file_not_found_cache_lifetime(&mut self, value: u32) {
        self.file_not_found_cache_lifetime = Some(value);
    }

    /// Gets the value of FileNotFoundCacheLifetime
    pub fn get_file_not_found_cache_lifetime(&self) -> Option<&u32> {
        self.file_not_found_cache_lifetime.as_ref()
    }

    /// Sets the value of ForceSMBEncryptionOverQuic
    pub fn set_force_smbencryption_over_quic(&mut self, value: bool) {
        self.force_smbencryption_over_quic = Some(value);
    }

    /// Gets the value of ForceSMBEncryptionOverQuic
    pub fn get_force_smbencryption_over_quic(&self) -> Option<&bool> {
        self.force_smbencryption_over_quic.as_ref()
    }

    /// Sets the value of InvalidAuthenticationCacheLifetime
    pub fn set_invalid_authentication_cache_lifetime(&mut self, value: u32) {
        self.invalid_authentication_cache_lifetime = Some(value);
    }

    /// Gets the value of InvalidAuthenticationCacheLifetime
    pub fn get_invalid_authentication_cache_lifetime(&self) -> Option<&u32> {
        self.invalid_authentication_cache_lifetime.as_ref()
    }

    /// Sets the value of KeepConn
    pub fn set_keep_conn(&mut self, value: u32) {
        self.keep_conn = Some(value);
    }

    /// Gets the value of KeepConn
    pub fn get_keep_conn(&self) -> Option<&u32> {
        self.keep_conn.as_ref()
    }

    /// Sets the value of MaxCmds
    pub fn set_max_cmds(&mut self, value: u32) {
        self.max_cmds = Some(value);
    }

    /// Gets the value of MaxCmds
    pub fn get_max_cmds(&self) -> Option<&u32> {
        self.max_cmds.as_ref()
    }

    /// Sets the value of MaximumConnectionCountPerServer
    pub fn set_maximum_connection_count_per_server(&mut self, value: u32) {
        self.maximum_connection_count_per_server = Some(value);
    }

    /// Gets the value of MaximumConnectionCountPerServer
    pub fn get_maximum_connection_count_per_server(&self) -> Option<&u32> {
        self.maximum_connection_count_per_server.as_ref()
    }

    /// Sets the value of OplocksDisabled
    pub fn set_oplocks_disabled(&mut self, value: bool) {
        self.oplocks_disabled = Some(value);
    }

    /// Gets the value of OplocksDisabled
    pub fn get_oplocks_disabled(&self) -> Option<&bool> {
        self.oplocks_disabled.as_ref()
    }

    /// Sets the value of RequestCompression
    pub fn set_request_compression(&mut self, value: bool) {
        self.request_compression = Some(value);
    }

    /// Gets the value of RequestCompression
    pub fn get_request_compression(&self) -> Option<&bool> {
        self.request_compression.as_ref()
    }

    /// Sets the value of RequireEncryption
    pub fn set_require_encryption(&mut self, value: bool) {
        self.require_encryption = Some(value);
    }

    /// Gets the value of RequireEncryption
    pub fn get_require_encryption(&self) -> Option<&bool> {
        self.require_encryption.as_ref()
    }

    /// Sets the value of RequireSecuritySignature
    pub fn set_require_security_signature(&mut self, value: bool) {
        self.require_security_signature = Some(value);
    }

    /// Gets the value of RequireSecuritySignature
    pub fn get_require_security_signature(&self) -> Option<&bool> {
        self.require_security_signature.as_ref()
    }

    /// Sets the value of SessionTimeout
    pub fn set_session_timeout(&mut self, value: u32) {
        self.session_timeout = Some(value);
    }

    /// Gets the value of SessionTimeout
    pub fn get_session_timeout(&self) -> Option<&u32> {
        self.session_timeout.as_ref()
    }

    /// Sets the value of SkipCertificateCheck
    pub fn set_skip_certificate_check(&mut self, value: bool) {
        self.skip_certificate_check = Some(value);
    }

    /// Gets the value of SkipCertificateCheck
    pub fn get_skip_certificate_check(&self) -> Option<&bool> {
        self.skip_certificate_check.as_ref()
    }

    /// Sets the value of Smb2DialectMax
    pub fn set_smb2_dialect_max(&mut self, value: SmbClientConfiguration_Smb2DialectMax) {
        self.smb2_dialect_max = Some(value);
    }

    /// Gets the value of Smb2DialectMax
    pub fn get_smb2_dialect_max(&self) -> Option<&SmbClientConfiguration_Smb2DialectMax> {
        self.smb2_dialect_max.as_ref()
    }

    /// Sets the value of Smb2DialectMin
    pub fn set_smb2_dialect_min(&mut self, value: SmbClientConfiguration_Smb2DialectMin) {
        self.smb2_dialect_min = Some(value);
    }

    /// Gets the value of Smb2DialectMin
    pub fn get_smb2_dialect_min(&self) -> Option<&SmbClientConfiguration_Smb2DialectMin> {
        self.smb2_dialect_min.as_ref()
    }

    /// Sets the value of UseOpportunisticLocking
    pub fn set_use_opportunistic_locking(&mut self, value: bool) {
        self.use_opportunistic_locking = Some(value);
    }

    /// Gets the value of UseOpportunisticLocking
    pub fn get_use_opportunistic_locking(&self) -> Option<&bool> {
        self.use_opportunistic_locking.as_ref()
    }

    /// Sets the value of WindowSizeThreshold
    pub fn set_window_size_threshold(&mut self, value: u32) {
        self.window_size_threshold = Some(value);
    }

    /// Gets the value of WindowSizeThreshold
    pub fn get_window_size_threshold(&self) -> Option<&u32> {
        self.window_size_threshold.as_ref()
    }

/// 

    /// * `output` -  (MSFT_SmbClientConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_configuration(&self, output: &mut MSFT_SmbClientConfiguration) -> Result<(), WmiError> {

        let result = self.invoke_method("GetConfiguration", &[])?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `all` -  (bool)
    /// * `audit_insecure_guest_logon` -  (bool)
    /// * `audit_server_does_not_support_encryption` -  (bool)
    /// * `audit_server_does_not_support_signing` -  (bool)
    /// * `block_ntlm` -  (bool)
    /// * `block_ntlmserver_exception_list` -  (bool)
    /// * `compressibility_sampling` -  (bool)
    /// * `connection_count_per_rss_network_interface` -  (bool)
    /// * `directory_cache_entries_max` -  (bool)
    /// * `directory_cache_entry_size_max` -  (bool)
    /// * `directory_cache_lifetime` -  (bool)
    /// * `disable_compression` -  (bool)
    /// * `disabled_smbquicserver_exception_list` -  (bool)
    /// * `dormant_file_limit` -  (bool)
    /// * `enable_bandwidth_throttling` -  (bool)
    /// * `enable_byte_range_locking_on_read_only_files` -  (bool)
    /// * `enable_large_mtu` -  (bool)
    /// * `enable_load_balance_scale_out` -  (bool)
    /// * `enable_mailslots` -  (bool)
    /// * `enable_multi_channel` -  (bool)
    /// * `enable_smbquic` -  (bool)
    /// * `encryption_ciphers` -  (bool)
    /// * `extended_session_timeout` -  (bool)
    /// * `file_info_cache_entries_max` -  (bool)
    /// * `file_info_cache_lifetime` -  (bool)
    /// * `file_not_found_cache_entries_max` -  (bool)
    /// * `file_not_found_cache_lifetime` -  (bool)
    /// * `force_smbencryption_over_quic` -  (bool)
    /// * `invalid_authentication_cache_lifetime` -  (bool)
    /// * `keep_conn` -  (bool)
    /// * `max_cmds` -  (bool)
    /// * `maximum_connection_count_per_server` -  (bool)
    /// * `oplocks_disabled` -  (bool)
    /// * `request_compression` -  (bool)
    /// * `require_encryption` -  (bool)
    /// * `session_timeout` -  (bool)
    /// * `skip_certificate_check` -  (bool)
    /// * `smb2_dialect_max` -  (bool)
    /// * `smb2_dialect_min` -  (bool)
    /// * `use_opportunistic_locking` -  (bool)
    /// * `window_size_threshold` -  (bool)

    /// * `output` -  (MSFT_SmbClientConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset_configuration(&self, output: &mut MSFT_SmbClientConfiguration, all: Option<bool>, connection_count_per_rss_network_interface: Option<bool>, directory_cache_entries_max: Option<bool>, directory_cache_entry_size_max: Option<bool>, directory_cache_lifetime: Option<bool>, enable_bandwidth_throttling: Option<bool>, enable_byte_range_locking_on_read_only_files: Option<bool>, enable_large_mtu: Option<bool>, enable_mailslots: Option<bool>, enable_multi_channel: Option<bool>, dormant_file_limit: Option<bool>, extended_session_timeout: Option<bool>, file_info_cache_entries_max: Option<bool>, file_info_cache_lifetime: Option<bool>, file_not_found_cache_entries_max: Option<bool>, file_not_found_cache_lifetime: Option<bool>, keep_conn: Option<bool>, max_cmds: Option<bool>, maximum_connection_count_per_server: Option<bool>, oplocks_disabled: Option<bool>, session_timeout: Option<bool>, use_opportunistic_locking: Option<bool>, window_size_threshold: Option<bool>, enable_load_balance_scale_out: Option<bool>, force_smbencryption_over_quic: Option<bool>, skip_certificate_check: Option<bool>, request_compression: Option<bool>, disable_compression: Option<bool>, compressibility_sampling: Option<bool>, encryption_ciphers: Option<bool>, invalid_authentication_cache_lifetime: Option<bool>, smb2_dialect_max: Option<bool>, smb2_dialect_min: Option<bool>, block_ntlm: Option<bool>, require_encryption: Option<bool>, audit_server_does_not_support_encryption: Option<bool>, audit_server_does_not_support_signing: Option<bool>, audit_insecure_guest_logon: Option<bool>, block_ntlmserver_exception_list: Option<bool>, enable_smbquic: Option<bool>, disabled_smbquicserver_exception_list: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = all {
            args.push(MethodParameter { name: "All".to_string(), value: val.into() });
        }
        if let Some(val) = connection_count_per_rss_network_interface {
            args.push(MethodParameter { name: "ConnectionCountPerRssNetworkInterface".to_string(), value: val.into() });
        }
        if let Some(val) = directory_cache_entries_max {
            args.push(MethodParameter { name: "DirectoryCacheEntriesMax".to_string(), value: val.into() });
        }
        if let Some(val) = directory_cache_entry_size_max {
            args.push(MethodParameter { name: "DirectoryCacheEntrySizeMax".to_string(), value: val.into() });
        }
        if let Some(val) = directory_cache_lifetime {
            args.push(MethodParameter { name: "DirectoryCacheLifetime".to_string(), value: val.into() });
        }
        if let Some(val) = enable_bandwidth_throttling {
            args.push(MethodParameter { name: "EnableBandwidthThrottling".to_string(), value: val.into() });
        }
        if let Some(val) = enable_byte_range_locking_on_read_only_files {
            args.push(MethodParameter { name: "EnableByteRangeLockingOnReadOnlyFiles".to_string(), value: val.into() });
        }
        if let Some(val) = enable_large_mtu {
            args.push(MethodParameter { name: "EnableLargeMtu".to_string(), value: val.into() });
        }
        if let Some(val) = enable_mailslots {
            args.push(MethodParameter { name: "EnableMailslots".to_string(), value: val.into() });
        }
        if let Some(val) = enable_multi_channel {
            args.push(MethodParameter { name: "EnableMultiChannel".to_string(), value: val.into() });
        }
        if let Some(val) = dormant_file_limit {
            args.push(MethodParameter { name: "DormantFileLimit".to_string(), value: val.into() });
        }
        if let Some(val) = extended_session_timeout {
            args.push(MethodParameter { name: "ExtendedSessionTimeout".to_string(), value: val.into() });
        }
        if let Some(val) = file_info_cache_entries_max {
            args.push(MethodParameter { name: "FileInfoCacheEntriesMax".to_string(), value: val.into() });
        }
        if let Some(val) = file_info_cache_lifetime {
            args.push(MethodParameter { name: "FileInfoCacheLifetime".to_string(), value: val.into() });
        }
        if let Some(val) = file_not_found_cache_entries_max {
            args.push(MethodParameter { name: "FileNotFoundCacheEntriesMax".to_string(), value: val.into() });
        }
        if let Some(val) = file_not_found_cache_lifetime {
            args.push(MethodParameter { name: "FileNotFoundCacheLifetime".to_string(), value: val.into() });
        }
        if let Some(val) = keep_conn {
            args.push(MethodParameter { name: "KeepConn".to_string(), value: val.into() });
        }
        if let Some(val) = max_cmds {
            args.push(MethodParameter { name: "MaxCmds".to_string(), value: val.into() });
        }
        if let Some(val) = maximum_connection_count_per_server {
            args.push(MethodParameter { name: "MaximumConnectionCountPerServer".to_string(), value: val.into() });
        }
        if let Some(val) = oplocks_disabled {
            args.push(MethodParameter { name: "OplocksDisabled".to_string(), value: val.into() });
        }
        if let Some(val) = session_timeout {
            args.push(MethodParameter { name: "SessionTimeout".to_string(), value: val.into() });
        }
        if let Some(val) = use_opportunistic_locking {
            args.push(MethodParameter { name: "UseOpportunisticLocking".to_string(), value: val.into() });
        }
        if let Some(val) = window_size_threshold {
            args.push(MethodParameter { name: "WindowSizeThreshold".to_string(), value: val.into() });
        }
        if let Some(val) = enable_load_balance_scale_out {
            args.push(MethodParameter { name: "EnableLoadBalanceScaleOut".to_string(), value: val.into() });
        }
        if let Some(val) = force_smbencryption_over_quic {
            args.push(MethodParameter { name: "ForceSMBEncryptionOverQuic".to_string(), value: val.into() });
        }
        if let Some(val) = skip_certificate_check {
            args.push(MethodParameter { name: "SkipCertificateCheck".to_string(), value: val.into() });
        }
        if let Some(val) = request_compression {
            args.push(MethodParameter { name: "RequestCompression".to_string(), value: val.into() });
        }
        if let Some(val) = disable_compression {
            args.push(MethodParameter { name: "DisableCompression".to_string(), value: val.into() });
        }
        if let Some(val) = compressibility_sampling {
            args.push(MethodParameter { name: "CompressibilitySampling".to_string(), value: val.into() });
        }
        if let Some(val) = encryption_ciphers {
            args.push(MethodParameter { name: "EncryptionCiphers".to_string(), value: val.into() });
        }
        if let Some(val) = invalid_authentication_cache_lifetime {
            args.push(MethodParameter { name: "InvalidAuthenticationCacheLifetime".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_dialect_max {
            args.push(MethodParameter { name: "Smb2DialectMax".to_string(), value: val.into() });
        }
        if let Some(val) = smb2_dialect_min {
            args.push(MethodParameter { name: "Smb2DialectMin".to_string(), value: val.into() });
        }
        if let Some(val) = block_ntlm {
            args.push(MethodParameter { name: "BlockNTLM".to_string(), value: val.into() });
        }
        if let Some(val) = require_encryption {
            args.push(MethodParameter { name: "RequireEncryption".to_string(), value: val.into() });
        }
        if let Some(val) = audit_server_does_not_support_encryption {
            args.push(MethodParameter { name: "AuditServerDoesNotSupportEncryption".to_string(), value: val.into() });
        }
        if let Some(val) = audit_server_does_not_support_signing {
            args.push(MethodParameter { name: "AuditServerDoesNotSupportSigning".to_string(), value: val.into() });
        }
        if let Some(val) = audit_insecure_guest_logon {
            args.push(MethodParameter { name: "AuditInsecureGuestLogon".to_string(), value: val.into() });
        }
        if let Some(val) = block_ntlmserver_exception_list {
            args.push(MethodParameter { name: "BlockNTLMServerExceptionList".to_string(), value: val.into() });
        }
        if let Some(val) = enable_smbquic {
            args.push(MethodParameter { name: "EnableSMBQUIC".to_string(), value: val.into() });
        }
        if let Some(val) = disabled_smbquicserver_exception_list {
            args.push(MethodParameter { name: "DisabledSMBQUICServerExceptionList".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ResetConfiguration", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `audit_insecure_guest_logon` -  (bool)
    /// * `audit_server_does_not_support_encryption` -  (bool)
    /// * `audit_server_does_not_support_signing` -  (bool)
    /// * `block_ntlm` -  (bool)
    /// * `block_ntlmserver_exception_list` -  (String)
    /// * `compressibility_sampling_size` -  (u64)
    /// * `compressible_threshold` -  (u64)
    /// * `connection_count_per_rss_network_interface` -  (u32)
    /// * `directory_cache_entries_max` -  (u32)
    /// * `directory_cache_entry_size_max` -  (u32)
    /// * `directory_cache_lifetime` -  (u32)
    /// * `disable_compression` -  (bool)
    /// * `disabled_smbquicserver_exception_list` -  (String)
    /// * `dormant_file_limit` -  (u32)
    /// * `enable_bandwidth_throttling` -  (bool)
    /// * `enable_byte_range_locking_on_read_only_files` -  (bool)
    /// * `enable_compressibility_sampling` -  (bool)
    /// * `enable_insecure_guest_logons` -  (bool)
    /// * `enable_large_mtu` -  (bool)
    /// * `enable_load_balance_scale_out` -  (bool)
    /// * `enable_mailslots` -  (bool)
    /// * `enable_multi_channel` -  (bool)
    /// * `enable_security_signature` -  (bool)
    /// * `enable_smbquic` -  (bool)
    /// * `encryption_ciphers` -  (String)
    /// * `extended_session_timeout` -  (u32)
    /// * `file_info_cache_entries_max` -  (u32)
    /// * `file_info_cache_lifetime` -  (u32)
    /// * `file_not_found_cache_entries_max` -  (u32)
    /// * `file_not_found_cache_lifetime` -  (u32)
    /// * `force_smbencryption_over_quic` -  (bool)
    /// * `invalid_authentication_cache_lifetime` -  (u32)
    /// * `keep_conn` -  (u32)
    /// * `max_cmds` -  (u32)
    /// * `maximum_connection_count_per_server` -  (u32)
    /// * `oplocks_disabled` -  (bool)
    /// * `request_compression` -  (bool)
    /// * `require_encryption` -  (bool)
    /// * `require_security_signature` -  (bool)
    /// * `session_timeout` -  (u32)
    /// * `skip_certificate_check` -  (bool)
    /// * `smb2_dialect_max` -  (u32)
    /// * `smb2_dialect_min` -  (u32)
    /// * `use_opportunistic_locking` -  (bool)
    /// * `window_size_threshold` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_configuration(&self, connection_count_per_rss_network_interface: u32, directory_cache_entries_max: u32, directory_cache_entry_size_max: u32, directory_cache_lifetime: u32, enable_bandwidth_throttling: bool, enable_byte_range_locking_on_read_only_files: bool, enable_large_mtu: bool, enable_mailslots: bool, enable_multi_channel: bool, dormant_file_limit: u32, enable_security_signature: bool, extended_session_timeout: u32, file_info_cache_entries_max: u32, file_info_cache_lifetime: u32, file_not_found_cache_entries_max: u32, file_not_found_cache_lifetime: u32, keep_conn: u32, max_cmds: u32, maximum_connection_count_per_server: u32, oplocks_disabled: bool, require_security_signature: bool, session_timeout: u32, use_opportunistic_locking: bool, window_size_threshold: u32, enable_load_balance_scale_out: bool, enable_insecure_guest_logons: bool, force_smbencryption_over_quic: bool, skip_certificate_check: bool, request_compression: bool, disable_compression: bool, enable_compressibility_sampling: bool, compressibility_sampling_size: u64, compressible_threshold: u64, encryption_ciphers: &String, invalid_authentication_cache_lifetime: u32, smb2_dialect_max: u32, smb2_dialect_min: u32, block_ntlm: bool, require_encryption: bool, audit_server_does_not_support_encryption: bool, audit_server_does_not_support_signing: bool, audit_insecure_guest_logon: bool, block_ntlmserver_exception_list: &String, enable_smbquic: bool, disabled_smbquicserver_exception_list: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionCountPerRssNetworkInterface".to_string(), value: connection_count_per_rss_network_interface.into() });
        args.push(MethodParameter { name: "DirectoryCacheEntriesMax".to_string(), value: directory_cache_entries_max.into() });
        args.push(MethodParameter { name: "DirectoryCacheEntrySizeMax".to_string(), value: directory_cache_entry_size_max.into() });
        args.push(MethodParameter { name: "DirectoryCacheLifetime".to_string(), value: directory_cache_lifetime.into() });
        args.push(MethodParameter { name: "EnableBandwidthThrottling".to_string(), value: enable_bandwidth_throttling.into() });
        args.push(MethodParameter { name: "EnableByteRangeLockingOnReadOnlyFiles".to_string(), value: enable_byte_range_locking_on_read_only_files.into() });
        args.push(MethodParameter { name: "EnableLargeMtu".to_string(), value: enable_large_mtu.into() });
        args.push(MethodParameter { name: "EnableMailslots".to_string(), value: enable_mailslots.into() });
        args.push(MethodParameter { name: "EnableMultiChannel".to_string(), value: enable_multi_channel.into() });
        args.push(MethodParameter { name: "DormantFileLimit".to_string(), value: dormant_file_limit.into() });
        args.push(MethodParameter { name: "EnableSecuritySignature".to_string(), value: enable_security_signature.into() });
        args.push(MethodParameter { name: "ExtendedSessionTimeout".to_string(), value: extended_session_timeout.into() });
        args.push(MethodParameter { name: "FileInfoCacheEntriesMax".to_string(), value: file_info_cache_entries_max.into() });
        args.push(MethodParameter { name: "FileInfoCacheLifetime".to_string(), value: file_info_cache_lifetime.into() });
        args.push(MethodParameter { name: "FileNotFoundCacheEntriesMax".to_string(), value: file_not_found_cache_entries_max.into() });
        args.push(MethodParameter { name: "FileNotFoundCacheLifetime".to_string(), value: file_not_found_cache_lifetime.into() });
        args.push(MethodParameter { name: "KeepConn".to_string(), value: keep_conn.into() });
        args.push(MethodParameter { name: "MaxCmds".to_string(), value: max_cmds.into() });
        args.push(MethodParameter { name: "MaximumConnectionCountPerServer".to_string(), value: maximum_connection_count_per_server.into() });
        args.push(MethodParameter { name: "OplocksDisabled".to_string(), value: oplocks_disabled.into() });
        args.push(MethodParameter { name: "RequireSecuritySignature".to_string(), value: require_security_signature.into() });
        args.push(MethodParameter { name: "SessionTimeout".to_string(), value: session_timeout.into() });
        args.push(MethodParameter { name: "UseOpportunisticLocking".to_string(), value: use_opportunistic_locking.into() });
        args.push(MethodParameter { name: "WindowSizeThreshold".to_string(), value: window_size_threshold.into() });
        args.push(MethodParameter { name: "EnableLoadBalanceScaleOut".to_string(), value: enable_load_balance_scale_out.into() });
        args.push(MethodParameter { name: "EnableInsecureGuestLogons".to_string(), value: enable_insecure_guest_logons.into() });
        args.push(MethodParameter { name: "ForceSMBEncryptionOverQuic".to_string(), value: force_smbencryption_over_quic.into() });
        args.push(MethodParameter { name: "SkipCertificateCheck".to_string(), value: skip_certificate_check.into() });
        args.push(MethodParameter { name: "RequestCompression".to_string(), value: request_compression.into() });
        args.push(MethodParameter { name: "DisableCompression".to_string(), value: disable_compression.into() });
        args.push(MethodParameter { name: "EnableCompressibilitySampling".to_string(), value: enable_compressibility_sampling.into() });
        args.push(MethodParameter { name: "CompressibilitySamplingSize".to_string(), value: compressibility_sampling_size.into() });
        args.push(MethodParameter { name: "CompressibleThreshold".to_string(), value: compressible_threshold.into() });
        args.push(MethodParameter { name: "EncryptionCiphers".to_string(), value: encryption_ciphers.into() });
        args.push(MethodParameter { name: "InvalidAuthenticationCacheLifetime".to_string(), value: invalid_authentication_cache_lifetime.into() });
        args.push(MethodParameter { name: "Smb2DialectMax".to_string(), value: smb2_dialect_max.into() });
        args.push(MethodParameter { name: "Smb2DialectMin".to_string(), value: smb2_dialect_min.into() });
        args.push(MethodParameter { name: "BlockNTLM".to_string(), value: block_ntlm.into() });
        args.push(MethodParameter { name: "RequireEncryption".to_string(), value: require_encryption.into() });
        args.push(MethodParameter { name: "AuditServerDoesNotSupportEncryption".to_string(), value: audit_server_does_not_support_encryption.into() });
        args.push(MethodParameter { name: "AuditServerDoesNotSupportSigning".to_string(), value: audit_server_does_not_support_signing.into() });
        args.push(MethodParameter { name: "AuditInsecureGuestLogon".to_string(), value: audit_insecure_guest_logon.into() });
        args.push(MethodParameter { name: "BlockNTLMServerExceptionList".to_string(), value: block_ntlmserver_exception_list.into() });
        args.push(MethodParameter { name: "EnableSMBQUIC".to_string(), value: enable_smbquic.into() });
        args.push(MethodParameter { name: "DisabledSMBQUICServerExceptionList".to_string(), value: disabled_smbquicserver_exception_list.into() });
        self.invoke_method("SetConfiguration", &args)

    }

}

