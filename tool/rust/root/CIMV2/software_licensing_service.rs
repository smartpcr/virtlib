// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SoftwareLicensingService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareLicensingService {

/// 
    #[serde(rename = "AzureMetadataResponse")]
    pub azure_metadata_response: Option<u32>,

/// 
    #[serde(rename = "ClientMachineID")]
    pub client_machine_id: Option<String>,

/// 
    #[serde(rename = "DiscoveredKeyManagementServiceMachineIpAddress")]
    pub discovered_key_management_service_machine_ip_address: Option<String>,

/// 
    #[serde(rename = "DiscoveredKeyManagementServiceMachineName")]
    pub discovered_key_management_service_machine_name: Option<String>,

/// 
    #[serde(rename = "DiscoveredKeyManagementServiceMachinePort")]
    pub discovered_key_management_service_machine_port: Option<u32>,

/// 
    #[serde(rename = "EmitWindowsMeter")]
    pub emit_windows_meter: Option<u32>,

/// 
    #[serde(rename = "IsKeyManagementServiceMachine")]
    pub is_key_management_service_machine: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceCurrentCount")]
    pub key_management_service_current_count: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceDnsPublishing")]
    pub key_management_service_dns_publishing: Option<bool>,

/// 
    #[serde(rename = "KeyManagementServiceFailedRequests")]
    pub key_management_service_failed_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceHostCaching")]
    pub key_management_service_host_caching: Option<bool>,

/// 
    #[serde(rename = "KeyManagementServiceLicensedRequests")]
    pub key_management_service_licensed_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceListeningPort")]
    pub key_management_service_listening_port: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceLookupDomain")]
    pub key_management_service_lookup_domain: Option<String>,

/// 
    #[serde(rename = "KeyManagementServiceLowPriority")]
    pub key_management_service_low_priority: Option<bool>,

/// 
    #[serde(rename = "KeyManagementServiceMachine")]
    pub key_management_service_machine: Option<String>,

/// 
    #[serde(rename = "KeyManagementServiceNonGenuineGraceRequests")]
    pub key_management_service_non_genuine_grace_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceNotificationRequests")]
    pub key_management_service_notification_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceOOBGraceRequests")]
    pub key_management_service_oobgrace_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceOOTGraceRequests")]
    pub key_management_service_ootgrace_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServicePort")]
    pub key_management_service_port: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceProductKeyID")]
    pub key_management_service_product_key_id: Option<String>,

/// 
    #[serde(rename = "KeyManagementServiceTotalRequests")]
    pub key_management_service_total_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceUnlicensedRequests")]
    pub key_management_service_unlicensed_requests: Option<u32>,

/// 
    #[serde(rename = "OA2xBiosMarkerMinorVersion")]
    pub oa2x_bios_marker_minor_version: Option<u32>,

/// 
    #[serde(rename = "OA2xBiosMarkerStatus")]
    pub oa2x_bios_marker_status: Option<u32>,

/// 
    #[serde(rename = "OA3xOriginalProductKey")]
    pub oa3x_original_product_key: Option<String>,

/// 
    #[serde(rename = "OA3xOriginalProductKeyDescription")]
    pub oa3x_original_product_key_description: Option<String>,

/// 
    #[serde(rename = "OA3xOriginalProductKeyPkPn")]
    pub oa3x_original_product_key_pk_pn: Option<String>,

/// 
    #[serde(rename = "PolicyCacheRefreshRequired")]
    pub policy_cache_refresh_required: Option<u32>,

/// 
    #[serde(rename = "RemainingWindowsReArmCount")]
    pub remaining_windows_re_arm_count: Option<u32>,

/// 
    #[serde(rename = "RequiredClientCount")]
    pub required_client_count: Option<u32>,

/// 
    #[serde(rename = "SubscriptionEdition")]
    pub subscription_edition: Option<String>,

/// 
    #[serde(rename = "SubscriptionExpiry")]
    pub subscription_expiry: Option<String>,

/// 
    #[serde(rename = "SubscriptionStatus")]
    pub subscription_status: Option<u32>,

/// 
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: Option<u32>,

/// 
    #[serde(rename = "TokenActivationAdditionalInfo")]
    pub token_activation_additional_info: Option<String>,

/// 
    #[serde(rename = "TokenActivationCertificateThumbprint")]
    pub token_activation_certificate_thumbprint: Option<String>,

/// 
    #[serde(rename = "TokenActivationGrantNumber")]
    pub token_activation_grant_number: Option<u32>,

/// 
    #[serde(rename = "TokenActivationILID")]
    pub token_activation_ilid: Option<String>,

/// 
    #[serde(rename = "TokenActivationILVID")]
    pub token_activation_ilvid: Option<u32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VLActivationInterval")]
    pub vlactivation_interval: Option<u32>,

/// 
    #[serde(rename = "VLRenewalInterval")]
    pub vlrenewal_interval: Option<u32>,
}

impl SoftwareLicensingService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            azure_metadata_response: None,
            client_machine_id: None,
            discovered_key_management_service_machine_ip_address: None,
            discovered_key_management_service_machine_name: None,
            discovered_key_management_service_machine_port: None,
            emit_windows_meter: None,
            is_key_management_service_machine: None,
            key_management_service_current_count: None,
            key_management_service_dns_publishing: None,
            key_management_service_failed_requests: None,
            key_management_service_host_caching: None,
            key_management_service_licensed_requests: None,
            key_management_service_listening_port: None,
            key_management_service_lookup_domain: None,
            key_management_service_low_priority: None,
            key_management_service_machine: None,
            key_management_service_non_genuine_grace_requests: None,
            key_management_service_notification_requests: None,
            key_management_service_oobgrace_requests: None,
            key_management_service_ootgrace_requests: None,
            key_management_service_port: None,
            key_management_service_product_key_id: None,
            key_management_service_total_requests: None,
            key_management_service_unlicensed_requests: None,
            oa2x_bios_marker_minor_version: None,
            oa2x_bios_marker_status: None,
            oa3x_original_product_key: None,
            oa3x_original_product_key_description: None,
            oa3x_original_product_key_pk_pn: None,
            policy_cache_refresh_required: None,
            remaining_windows_re_arm_count: None,
            required_client_count: None,
            subscription_edition: None,
            subscription_expiry: None,
            subscription_status: None,
            subscription_type: None,
            token_activation_additional_info: None,
            token_activation_certificate_thumbprint: None,
            token_activation_grant_number: None,
            token_activation_ilid: None,
            token_activation_ilvid: None,
            version: None,
            vlactivation_interval: None,
            vlrenewal_interval: None,
        }
    }


    /// Sets the value of AzureMetadataResponse
    pub fn set_azure_metadata_response(&mut self, value: u32) {
        self.azure_metadata_response = Some(value);
    }

    /// Gets the value of AzureMetadataResponse
    pub fn get_azure_metadata_response(&self) -> Option<&u32> {
        self.azure_metadata_response.as_ref()
    }

    /// Sets the value of ClientMachineID
    pub fn set_client_machine_id(&mut self, value: String) {
        self.client_machine_id = Some(value);
    }

    /// Gets the value of ClientMachineID
    pub fn get_client_machine_id(&self) -> Option<&String> {
        self.client_machine_id.as_ref()
    }

    /// Sets the value of DiscoveredKeyManagementServiceMachineIpAddress
    pub fn set_discovered_key_management_service_machine_ip_address(&mut self, value: String) {
        self.discovered_key_management_service_machine_ip_address = Some(value);
    }

    /// Gets the value of DiscoveredKeyManagementServiceMachineIpAddress
    pub fn get_discovered_key_management_service_machine_ip_address(&self) -> Option<&String> {
        self.discovered_key_management_service_machine_ip_address.as_ref()
    }

    /// Sets the value of DiscoveredKeyManagementServiceMachineName
    pub fn set_discovered_key_management_service_machine_name(&mut self, value: String) {
        self.discovered_key_management_service_machine_name = Some(value);
    }

    /// Gets the value of DiscoveredKeyManagementServiceMachineName
    pub fn get_discovered_key_management_service_machine_name(&self) -> Option<&String> {
        self.discovered_key_management_service_machine_name.as_ref()
    }

    /// Sets the value of DiscoveredKeyManagementServiceMachinePort
    pub fn set_discovered_key_management_service_machine_port(&mut self, value: u32) {
        self.discovered_key_management_service_machine_port = Some(value);
    }

    /// Gets the value of DiscoveredKeyManagementServiceMachinePort
    pub fn get_discovered_key_management_service_machine_port(&self) -> Option<&u32> {
        self.discovered_key_management_service_machine_port.as_ref()
    }

    /// Sets the value of EmitWindowsMeter
    pub fn set_emit_windows_meter(&mut self, value: u32) {
        self.emit_windows_meter = Some(value);
    }

    /// Gets the value of EmitWindowsMeter
    pub fn get_emit_windows_meter(&self) -> Option<&u32> {
        self.emit_windows_meter.as_ref()
    }

    /// Sets the value of IsKeyManagementServiceMachine
    pub fn set_is_key_management_service_machine(&mut self, value: u32) {
        self.is_key_management_service_machine = Some(value);
    }

    /// Gets the value of IsKeyManagementServiceMachine
    pub fn get_is_key_management_service_machine(&self) -> Option<&u32> {
        self.is_key_management_service_machine.as_ref()
    }

    /// Sets the value of KeyManagementServiceCurrentCount
    pub fn set_key_management_service_current_count(&mut self, value: u32) {
        self.key_management_service_current_count = Some(value);
    }

    /// Gets the value of KeyManagementServiceCurrentCount
    pub fn get_key_management_service_current_count(&self) -> Option<&u32> {
        self.key_management_service_current_count.as_ref()
    }

    /// Sets the value of KeyManagementServiceDnsPublishing
    pub fn set_key_management_service_dns_publishing(&mut self, value: bool) {
        self.key_management_service_dns_publishing = Some(value);
    }

    /// Gets the value of KeyManagementServiceDnsPublishing
    pub fn get_key_management_service_dns_publishing(&self) -> Option<&bool> {
        self.key_management_service_dns_publishing.as_ref()
    }

    /// Sets the value of KeyManagementServiceFailedRequests
    pub fn set_key_management_service_failed_requests(&mut self, value: u32) {
        self.key_management_service_failed_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceFailedRequests
    pub fn get_key_management_service_failed_requests(&self) -> Option<&u32> {
        self.key_management_service_failed_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceHostCaching
    pub fn set_key_management_service_host_caching(&mut self, value: bool) {
        self.key_management_service_host_caching = Some(value);
    }

    /// Gets the value of KeyManagementServiceHostCaching
    pub fn get_key_management_service_host_caching(&self) -> Option<&bool> {
        self.key_management_service_host_caching.as_ref()
    }

    /// Sets the value of KeyManagementServiceLicensedRequests
    pub fn set_key_management_service_licensed_requests(&mut self, value: u32) {
        self.key_management_service_licensed_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceLicensedRequests
    pub fn get_key_management_service_licensed_requests(&self) -> Option<&u32> {
        self.key_management_service_licensed_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceListeningPort
    pub fn set_key_management_service_listening_port(&mut self, value: u32) {
        self.key_management_service_listening_port = Some(value);
    }

    /// Gets the value of KeyManagementServiceListeningPort
    pub fn get_key_management_service_listening_port(&self) -> Option<&u32> {
        self.key_management_service_listening_port.as_ref()
    }

    /// Sets the value of KeyManagementServiceLookupDomain
    pub fn set_key_management_service_lookup_domain(&mut self, value: String) {
        self.key_management_service_lookup_domain = Some(value);
    }

    /// Gets the value of KeyManagementServiceLookupDomain
    pub fn get_key_management_service_lookup_domain(&self) -> Option<&String> {
        self.key_management_service_lookup_domain.as_ref()
    }

    /// Sets the value of KeyManagementServiceLowPriority
    pub fn set_key_management_service_low_priority(&mut self, value: bool) {
        self.key_management_service_low_priority = Some(value);
    }

    /// Gets the value of KeyManagementServiceLowPriority
    pub fn get_key_management_service_low_priority(&self) -> Option<&bool> {
        self.key_management_service_low_priority.as_ref()
    }

    /// Sets the value of KeyManagementServiceMachine
    pub fn set_key_management_service_machine(&mut self, value: String) {
        self.key_management_service_machine = Some(value);
    }

    /// Gets the value of KeyManagementServiceMachine
    pub fn get_key_management_service_machine(&self) -> Option<&String> {
        self.key_management_service_machine.as_ref()
    }

    /// Sets the value of KeyManagementServiceNonGenuineGraceRequests
    pub fn set_key_management_service_non_genuine_grace_requests(&mut self, value: u32) {
        self.key_management_service_non_genuine_grace_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceNonGenuineGraceRequests
    pub fn get_key_management_service_non_genuine_grace_requests(&self) -> Option<&u32> {
        self.key_management_service_non_genuine_grace_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceNotificationRequests
    pub fn set_key_management_service_notification_requests(&mut self, value: u32) {
        self.key_management_service_notification_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceNotificationRequests
    pub fn get_key_management_service_notification_requests(&self) -> Option<&u32> {
        self.key_management_service_notification_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceOOBGraceRequests
    pub fn set_key_management_service_oobgrace_requests(&mut self, value: u32) {
        self.key_management_service_oobgrace_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceOOBGraceRequests
    pub fn get_key_management_service_oobgrace_requests(&self) -> Option<&u32> {
        self.key_management_service_oobgrace_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceOOTGraceRequests
    pub fn set_key_management_service_ootgrace_requests(&mut self, value: u32) {
        self.key_management_service_ootgrace_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceOOTGraceRequests
    pub fn get_key_management_service_ootgrace_requests(&self) -> Option<&u32> {
        self.key_management_service_ootgrace_requests.as_ref()
    }

    /// Sets the value of KeyManagementServicePort
    pub fn set_key_management_service_port(&mut self, value: u32) {
        self.key_management_service_port = Some(value);
    }

    /// Gets the value of KeyManagementServicePort
    pub fn get_key_management_service_port(&self) -> Option<&u32> {
        self.key_management_service_port.as_ref()
    }

    /// Sets the value of KeyManagementServiceProductKeyID
    pub fn set_key_management_service_product_key_id(&mut self, value: String) {
        self.key_management_service_product_key_id = Some(value);
    }

    /// Gets the value of KeyManagementServiceProductKeyID
    pub fn get_key_management_service_product_key_id(&self) -> Option<&String> {
        self.key_management_service_product_key_id.as_ref()
    }

    /// Sets the value of KeyManagementServiceTotalRequests
    pub fn set_key_management_service_total_requests(&mut self, value: u32) {
        self.key_management_service_total_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceTotalRequests
    pub fn get_key_management_service_total_requests(&self) -> Option<&u32> {
        self.key_management_service_total_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceUnlicensedRequests
    pub fn set_key_management_service_unlicensed_requests(&mut self, value: u32) {
        self.key_management_service_unlicensed_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceUnlicensedRequests
    pub fn get_key_management_service_unlicensed_requests(&self) -> Option<&u32> {
        self.key_management_service_unlicensed_requests.as_ref()
    }

    /// Sets the value of OA2xBiosMarkerMinorVersion
    pub fn set_oa2x_bios_marker_minor_version(&mut self, value: u32) {
        self.oa2x_bios_marker_minor_version = Some(value);
    }

    /// Gets the value of OA2xBiosMarkerMinorVersion
    pub fn get_oa2x_bios_marker_minor_version(&self) -> Option<&u32> {
        self.oa2x_bios_marker_minor_version.as_ref()
    }

    /// Sets the value of OA2xBiosMarkerStatus
    pub fn set_oa2x_bios_marker_status(&mut self, value: u32) {
        self.oa2x_bios_marker_status = Some(value);
    }

    /// Gets the value of OA2xBiosMarkerStatus
    pub fn get_oa2x_bios_marker_status(&self) -> Option<&u32> {
        self.oa2x_bios_marker_status.as_ref()
    }

    /// Sets the value of OA3xOriginalProductKey
    pub fn set_oa3x_original_product_key(&mut self, value: String) {
        self.oa3x_original_product_key = Some(value);
    }

    /// Gets the value of OA3xOriginalProductKey
    pub fn get_oa3x_original_product_key(&self) -> Option<&String> {
        self.oa3x_original_product_key.as_ref()
    }

    /// Sets the value of OA3xOriginalProductKeyDescription
    pub fn set_oa3x_original_product_key_description(&mut self, value: String) {
        self.oa3x_original_product_key_description = Some(value);
    }

    /// Gets the value of OA3xOriginalProductKeyDescription
    pub fn get_oa3x_original_product_key_description(&self) -> Option<&String> {
        self.oa3x_original_product_key_description.as_ref()
    }

    /// Sets the value of OA3xOriginalProductKeyPkPn
    pub fn set_oa3x_original_product_key_pk_pn(&mut self, value: String) {
        self.oa3x_original_product_key_pk_pn = Some(value);
    }

    /// Gets the value of OA3xOriginalProductKeyPkPn
    pub fn get_oa3x_original_product_key_pk_pn(&self) -> Option<&String> {
        self.oa3x_original_product_key_pk_pn.as_ref()
    }

    /// Sets the value of PolicyCacheRefreshRequired
    pub fn set_policy_cache_refresh_required(&mut self, value: u32) {
        self.policy_cache_refresh_required = Some(value);
    }

    /// Gets the value of PolicyCacheRefreshRequired
    pub fn get_policy_cache_refresh_required(&self) -> Option<&u32> {
        self.policy_cache_refresh_required.as_ref()
    }

    /// Sets the value of RemainingWindowsReArmCount
    pub fn set_remaining_windows_re_arm_count(&mut self, value: u32) {
        self.remaining_windows_re_arm_count = Some(value);
    }

    /// Gets the value of RemainingWindowsReArmCount
    pub fn get_remaining_windows_re_arm_count(&self) -> Option<&u32> {
        self.remaining_windows_re_arm_count.as_ref()
    }

    /// Sets the value of RequiredClientCount
    pub fn set_required_client_count(&mut self, value: u32) {
        self.required_client_count = Some(value);
    }

    /// Gets the value of RequiredClientCount
    pub fn get_required_client_count(&self) -> Option<&u32> {
        self.required_client_count.as_ref()
    }

    /// Sets the value of SubscriptionEdition
    pub fn set_subscription_edition(&mut self, value: String) {
        self.subscription_edition = Some(value);
    }

    /// Gets the value of SubscriptionEdition
    pub fn get_subscription_edition(&self) -> Option<&String> {
        self.subscription_edition.as_ref()
    }

    /// Sets the value of SubscriptionExpiry
    pub fn set_subscription_expiry(&mut self, value: String) {
        self.subscription_expiry = Some(value);
    }

    /// Gets the value of SubscriptionExpiry
    pub fn get_subscription_expiry(&self) -> Option<&String> {
        self.subscription_expiry.as_ref()
    }

    /// Sets the value of SubscriptionStatus
    pub fn set_subscription_status(&mut self, value: u32) {
        self.subscription_status = Some(value);
    }

    /// Gets the value of SubscriptionStatus
    pub fn get_subscription_status(&self) -> Option<&u32> {
        self.subscription_status.as_ref()
    }

    /// Sets the value of SubscriptionType
    pub fn set_subscription_type(&mut self, value: u32) {
        self.subscription_type = Some(value);
    }

    /// Gets the value of SubscriptionType
    pub fn get_subscription_type(&self) -> Option<&u32> {
        self.subscription_type.as_ref()
    }

    /// Sets the value of TokenActivationAdditionalInfo
    pub fn set_token_activation_additional_info(&mut self, value: String) {
        self.token_activation_additional_info = Some(value);
    }

    /// Gets the value of TokenActivationAdditionalInfo
    pub fn get_token_activation_additional_info(&self) -> Option<&String> {
        self.token_activation_additional_info.as_ref()
    }

    /// Sets the value of TokenActivationCertificateThumbprint
    pub fn set_token_activation_certificate_thumbprint(&mut self, value: String) {
        self.token_activation_certificate_thumbprint = Some(value);
    }

    /// Gets the value of TokenActivationCertificateThumbprint
    pub fn get_token_activation_certificate_thumbprint(&self) -> Option<&String> {
        self.token_activation_certificate_thumbprint.as_ref()
    }

    /// Sets the value of TokenActivationGrantNumber
    pub fn set_token_activation_grant_number(&mut self, value: u32) {
        self.token_activation_grant_number = Some(value);
    }

    /// Gets the value of TokenActivationGrantNumber
    pub fn get_token_activation_grant_number(&self) -> Option<&u32> {
        self.token_activation_grant_number.as_ref()
    }

    /// Sets the value of TokenActivationILID
    pub fn set_token_activation_ilid(&mut self, value: String) {
        self.token_activation_ilid = Some(value);
    }

    /// Gets the value of TokenActivationILID
    pub fn get_token_activation_ilid(&self) -> Option<&String> {
        self.token_activation_ilid.as_ref()
    }

    /// Sets the value of TokenActivationILVID
    pub fn set_token_activation_ilvid(&mut self, value: u32) {
        self.token_activation_ilvid = Some(value);
    }

    /// Gets the value of TokenActivationILVID
    pub fn get_token_activation_ilvid(&self) -> Option<&u32> {
        self.token_activation_ilvid.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VLActivationInterval
    pub fn set_vlactivation_interval(&mut self, value: u32) {
        self.vlactivation_interval = Some(value);
    }

    /// Gets the value of VLActivationInterval
    pub fn get_vlactivation_interval(&self) -> Option<&u32> {
        self.vlactivation_interval.as_ref()
    }

    /// Sets the value of VLRenewalInterval
    pub fn set_vlrenewal_interval(&mut self, value: u32) {
        self.vlrenewal_interval = Some(value);
    }

    /// Gets the value of VLRenewalInterval
    pub fn get_vlrenewal_interval(&self) -> Option<&u32> {
        self.vlrenewal_interval.as_ref()
    }

/// 

    /// * `product_key` -  (String)

    /// * `return_value` -  (u32)
    pub fn install_product_key(&self, product_key: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProductKey".to_string(), value: product_key.into() });
        self.invoke_method("InstallProductKey", &args)

    }


/// 

    /// * `license` -  (String)

    /// * `return_value` -  (u32)
    pub fn install_license(&self, license: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "License".to_string(), value: license.into() });
        self.invoke_method("InstallLicense", &args)

    }


/// 

    /// * `license_package` -  (String)

    /// * `return_value` -  (u32)
    pub fn install_license_package(&self, license_package: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LicensePackage".to_string(), value: license_package.into() });
        self.invoke_method("InstallLicensePackage", &args)

    }


/// 

    /// * `machine_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_key_management_service_machine(&self, machine_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MachineName".to_string(), value: machine_name.into() });
        self.invoke_method("SetKeyManagementServiceMachine", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_key_management_service_machine(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearKeyManagementServiceMachine", &[])

    }


/// 

    /// * `port_number` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_key_management_service_port(&self, port_number: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortNumber".to_string(), value: port_number.into() });
        self.invoke_method("SetKeyManagementServicePort", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_key_management_service_port(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearKeyManagementServicePort", &[])

    }


/// 

    /// * `lookup_domain` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_key_management_service_lookup_domain(&self, lookup_domain: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LookupDomain".to_string(), value: lookup_domain.into() });
        self.invoke_method("SetKeyManagementServiceLookupDomain", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_key_management_service_lookup_domain(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearKeyManagementServiceLookupDomain", &[])

    }


/// 

    /// * `activation_interval` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_vlactivation_interval(&self, activation_interval: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ActivationInterval".to_string(), value: activation_interval.into() });
        self.invoke_method("SetVLActivationInterval", &args)

    }


/// 

    /// * `renewal_interval` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_vlrenewal_interval(&self, renewal_interval: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RenewalInterval".to_string(), value: renewal_interval.into() });
        self.invoke_method("SetVLRenewalInterval", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_product_key_from_registry(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearProductKeyFromRegistry", &[])

    }


/// 

    /// * `server_url` -  (String)
    /// * `template_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn acquire_genuine_ticket(&self, template_id: &String, server_url: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        args.push(MethodParameter { name: "ServerUrl".to_string(), value: server_url.into() });
        self.invoke_method("AcquireGenuineTicket", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn re_arm_windows(&self) -> Result<(), WmiError> {
        self.invoke_method("ReArmWindows", &[])

    }


/// 

    /// * `application_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn re_arm_app(&self, application_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ApplicationId".to_string(), value: application_id.into() });
        self.invoke_method("ReArmApp", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn refresh_license_status(&self) -> Result<(), WmiError> {
        self.invoke_method("RefreshLicenseStatus", &[])

    }


/// 

    /// * `port_number` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_key_management_service_listening_port(&self, port_number: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortNumber".to_string(), value: port_number.into() });
        self.invoke_method("SetKeyManagementServiceListeningPort", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_key_management_service_listening_port(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearKeyManagementServiceListeningPort", &[])

    }


/// 

    /// * `disable_publishing` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable_key_management_service_dns_publishing(&self, disable_publishing: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisablePublishing".to_string(), value: disable_publishing.into() });
        self.invoke_method("DisableKeyManagementServiceDnsPublishing", &args)

    }


/// 

    /// * `enable_low_priority` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable_key_management_service_low_priority(&self, enable_low_priority: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableLowPriority".to_string(), value: enable_low_priority.into() });
        self.invoke_method("EnableKeyManagementServiceLowPriority", &args)

    }


/// 

    /// * `disable_caching` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable_key_management_service_host_caching(&self, disable_caching: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisableCaching".to_string(), value: disable_caching.into() });
        self.invoke_method("DisableKeyManagementServiceHostCaching", &args)

    }


/// 

    /// * `product_key` -  (String)

    /// * `installation_id` -  (String)
    /// * `return_value` -  (u32)
    pub fn generate_active_directory_offline_activation_id(&self, product_key: &String, installation_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProductKey".to_string(), value: product_key.into() });

        let result = self.invoke_method("GenerateActiveDirectoryOfflineActivationId", &args)?;
        let installation_id = result.get_value("InstallationID")?;
        Ok(result.return_value)

    }


/// 

    /// * `activation_object_name` -  (String)
    /// * `confirmation_id` -  (String)
    /// * `product_key` -  (String)

    /// * `return_value` -  (u32)
    pub fn deposit_active_directory_offline_activation_confirmation(&self, product_key: &String, confirmation_id: &String, activation_object_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProductKey".to_string(), value: product_key.into() });
        args.push(MethodParameter { name: "ConfirmationID".to_string(), value: confirmation_id.into() });
        args.push(MethodParameter { name: "ActivationObjectName".to_string(), value: activation_object_name.into() });
        self.invoke_method("DepositActiveDirectoryOfflineActivationConfirmation", &args)

    }


/// 

    /// * `activation_object_name` -  (String)
    /// * `product_key` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_active_directory_online_activation(&self, product_key: &String, activation_object_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProductKey".to_string(), value: product_key.into() });
        args.push(MethodParameter { name: "ActivationObjectName".to_string(), value: activation_object_name.into() });
        self.invoke_method("DoActiveDirectoryOnlineActivation", &args)

    }


/// 

    /// * `activation_type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_vlactivation_type_enabled(&self, activation_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ActivationType".to_string(), value: activation_type.into() });
        self.invoke_method("SetVLActivationTypeEnabled", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_vlactivation_type_enabled(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearVLActivationTypeEnabled", &[])

    }

}

