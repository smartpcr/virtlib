// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SoftwareLicensingProduct struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareLicensingProduct {

/// 
    #[serde(rename = "ADActivationCsvlkPid")]
    pub adactivation_csvlk_pid: Option<String>,

/// 
    #[serde(rename = "ADActivationCsvlkSkuId")]
    pub adactivation_csvlk_sku_id: Option<String>,

/// 
    #[serde(rename = "ADActivationObjectDN")]
    pub adactivation_object_dn: Option<String>,

/// 
    #[serde(rename = "ADActivationObjectName")]
    pub adactivation_object_name: Option<String>,

/// 
    #[serde(rename = "ApplicationID")]
    pub application_id: Option<String>,

/// 
    #[serde(rename = "AutomaticVMActivationHostDigitalPid2")]
    pub automatic_vmactivation_host_digital_pid2: Option<String>,

/// 
    #[serde(rename = "AutomaticVMActivationHostMachineName")]
    pub automatic_vmactivation_host_machine_name: Option<String>,

/// 
    #[serde(rename = "AutomaticVMActivationLastActivationTime")]
    pub automatic_vmactivation_last_activation_time: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

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
    #[serde(rename = "EvaluationEndDate")]
    pub evaluation_end_date: Option<String>,

/// 
    #[serde(rename = "ExtendedGrace")]
    pub extended_grace: Option<u32>,

/// 
    #[serde(rename = "GenuineStatus")]
    pub genuine_status: Option<u32>,

/// 
    #[serde(rename = "GracePeriodRemaining")]
    pub grace_period_remaining: Option<u32>,

/// 
    #[serde(rename = "IAID")]
    pub iaid: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsKeyManagementServiceMachine")]
    pub is_key_management_service_machine: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceCurrentCount")]
    pub key_management_service_current_count: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceFailedRequests")]
    pub key_management_service_failed_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceLicensedRequests")]
    pub key_management_service_licensed_requests: Option<u32>,

/// 
    #[serde(rename = "KeyManagementServiceLookupDomain")]
    pub key_management_service_lookup_domain: Option<String>,

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
    #[serde(rename = "LicenseDependsOn")]
    pub license_depends_on: Option<String>,

/// 
    #[serde(rename = "LicenseFamily")]
    pub license_family: Option<String>,

/// 
    #[serde(rename = "LicenseIsAddon")]
    pub license_is_addon: Option<bool>,

/// 
    #[serde(rename = "LicenseStatus")]
    pub license_status: Option<u32>,

/// 
    #[serde(rename = "LicenseStatusReason")]
    pub license_status_reason: Option<u32>,

/// 
    #[serde(rename = "MachineURL")]
    pub machine_url: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OfflineInstallationId")]
    pub offline_installation_id: Option<String>,

/// 
    #[serde(rename = "PartialProductKey")]
    pub partial_product_key: Option<String>,

/// 
    #[serde(rename = "ProcessorURL")]
    pub processor_url: Option<String>,

/// 
    #[serde(rename = "ProductKeyChannel")]
    pub product_key_channel: Option<String>,

/// 
    #[serde(rename = "ProductKeyID")]
    pub product_key_id: Option<String>,

/// 
    #[serde(rename = "ProductKeyID2")]
    pub product_key_id2: Option<String>,

/// 
    #[serde(rename = "ProductKeyURL")]
    pub product_key_url: Option<String>,

/// 
    #[serde(rename = "RemainingAppReArmCount")]
    pub remaining_app_re_arm_count: Option<u32>,

/// 
    #[serde(rename = "RemainingSkuReArmCount")]
    pub remaining_sku_re_arm_count: Option<u32>,

/// 
    #[serde(rename = "RequiredClientCount")]
    pub required_client_count: Option<u32>,

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
    #[serde(rename = "TrustedTime")]
    pub trusted_time: Option<String>,

/// 
    #[serde(rename = "UseLicenseURL")]
    pub use_license_url: Option<String>,

/// 
    #[serde(rename = "ValidationURL")]
    pub validation_url: Option<String>,

/// 
    #[serde(rename = "VLActivationInterval")]
    pub vlactivation_interval: Option<u32>,

/// 
    #[serde(rename = "VLActivationType")]
    pub vlactivation_type: Option<u32>,

/// 
    #[serde(rename = "VLActivationTypeEnabled")]
    pub vlactivation_type_enabled: Option<u32>,

/// 
    #[serde(rename = "VLRenewalInterval")]
    pub vlrenewal_interval: Option<u32>,
}

impl SoftwareLicensingProduct {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            adactivation_csvlk_pid: None,
            adactivation_csvlk_sku_id: None,
            adactivation_object_dn: None,
            adactivation_object_name: None,
            application_id: None,
            automatic_vmactivation_host_digital_pid2: None,
            automatic_vmactivation_host_machine_name: None,
            automatic_vmactivation_last_activation_time: None,
            description: None,
            discovered_key_management_service_machine_ip_address: None,
            discovered_key_management_service_machine_name: None,
            discovered_key_management_service_machine_port: None,
            evaluation_end_date: None,
            extended_grace: None,
            genuine_status: None,
            grace_period_remaining: None,
            iaid: None,
            id: None,
            is_key_management_service_machine: None,
            key_management_service_current_count: None,
            key_management_service_failed_requests: None,
            key_management_service_licensed_requests: None,
            key_management_service_lookup_domain: None,
            key_management_service_machine: None,
            key_management_service_non_genuine_grace_requests: None,
            key_management_service_notification_requests: None,
            key_management_service_oobgrace_requests: None,
            key_management_service_ootgrace_requests: None,
            key_management_service_port: None,
            key_management_service_product_key_id: None,
            key_management_service_total_requests: None,
            key_management_service_unlicensed_requests: None,
            license_depends_on: None,
            license_family: None,
            license_is_addon: None,
            license_status: None,
            license_status_reason: None,
            machine_url: None,
            name: None,
            offline_installation_id: None,
            partial_product_key: None,
            processor_url: None,
            product_key_channel: None,
            product_key_id: None,
            product_key_id2: None,
            product_key_url: None,
            remaining_app_re_arm_count: None,
            remaining_sku_re_arm_count: None,
            required_client_count: None,
            token_activation_additional_info: None,
            token_activation_certificate_thumbprint: None,
            token_activation_grant_number: None,
            token_activation_ilid: None,
            token_activation_ilvid: None,
            trusted_time: None,
            use_license_url: None,
            validation_url: None,
            vlactivation_interval: None,
            vlactivation_type: None,
            vlactivation_type_enabled: None,
            vlrenewal_interval: None,
        }
    }


    /// Sets the value of ADActivationCsvlkPid
    pub fn set_adactivation_csvlk_pid(&mut self, value: String) {
        self.adactivation_csvlk_pid = Some(value);
    }

    /// Gets the value of ADActivationCsvlkPid
    pub fn get_adactivation_csvlk_pid(&self) -> Option<&String> {
        self.adactivation_csvlk_pid.as_ref()
    }

    /// Sets the value of ADActivationCsvlkSkuId
    pub fn set_adactivation_csvlk_sku_id(&mut self, value: String) {
        self.adactivation_csvlk_sku_id = Some(value);
    }

    /// Gets the value of ADActivationCsvlkSkuId
    pub fn get_adactivation_csvlk_sku_id(&self) -> Option<&String> {
        self.adactivation_csvlk_sku_id.as_ref()
    }

    /// Sets the value of ADActivationObjectDN
    pub fn set_adactivation_object_dn(&mut self, value: String) {
        self.adactivation_object_dn = Some(value);
    }

    /// Gets the value of ADActivationObjectDN
    pub fn get_adactivation_object_dn(&self) -> Option<&String> {
        self.adactivation_object_dn.as_ref()
    }

    /// Sets the value of ADActivationObjectName
    pub fn set_adactivation_object_name(&mut self, value: String) {
        self.adactivation_object_name = Some(value);
    }

    /// Gets the value of ADActivationObjectName
    pub fn get_adactivation_object_name(&self) -> Option<&String> {
        self.adactivation_object_name.as_ref()
    }

    /// Sets the value of ApplicationID
    pub fn set_application_id(&mut self, value: String) {
        self.application_id = Some(value);
    }

    /// Gets the value of ApplicationID
    pub fn get_application_id(&self) -> Option<&String> {
        self.application_id.as_ref()
    }

    /// Sets the value of AutomaticVMActivationHostDigitalPid2
    pub fn set_automatic_vmactivation_host_digital_pid2(&mut self, value: String) {
        self.automatic_vmactivation_host_digital_pid2 = Some(value);
    }

    /// Gets the value of AutomaticVMActivationHostDigitalPid2
    pub fn get_automatic_vmactivation_host_digital_pid2(&self) -> Option<&String> {
        self.automatic_vmactivation_host_digital_pid2.as_ref()
    }

    /// Sets the value of AutomaticVMActivationHostMachineName
    pub fn set_automatic_vmactivation_host_machine_name(&mut self, value: String) {
        self.automatic_vmactivation_host_machine_name = Some(value);
    }

    /// Gets the value of AutomaticVMActivationHostMachineName
    pub fn get_automatic_vmactivation_host_machine_name(&self) -> Option<&String> {
        self.automatic_vmactivation_host_machine_name.as_ref()
    }

    /// Sets the value of AutomaticVMActivationLastActivationTime
    pub fn set_automatic_vmactivation_last_activation_time(&mut self, value: String) {
        self.automatic_vmactivation_last_activation_time = Some(value);
    }

    /// Gets the value of AutomaticVMActivationLastActivationTime
    pub fn get_automatic_vmactivation_last_activation_time(&self) -> Option<&String> {
        self.automatic_vmactivation_last_activation_time.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
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

    /// Sets the value of EvaluationEndDate
    pub fn set_evaluation_end_date(&mut self, value: String) {
        self.evaluation_end_date = Some(value);
    }

    /// Gets the value of EvaluationEndDate
    pub fn get_evaluation_end_date(&self) -> Option<&String> {
        self.evaluation_end_date.as_ref()
    }

    /// Sets the value of ExtendedGrace
    pub fn set_extended_grace(&mut self, value: u32) {
        self.extended_grace = Some(value);
    }

    /// Gets the value of ExtendedGrace
    pub fn get_extended_grace(&self) -> Option<&u32> {
        self.extended_grace.as_ref()
    }

    /// Sets the value of GenuineStatus
    pub fn set_genuine_status(&mut self, value: u32) {
        self.genuine_status = Some(value);
    }

    /// Gets the value of GenuineStatus
    pub fn get_genuine_status(&self) -> Option<&u32> {
        self.genuine_status.as_ref()
    }

    /// Sets the value of GracePeriodRemaining
    pub fn set_grace_period_remaining(&mut self, value: u32) {
        self.grace_period_remaining = Some(value);
    }

    /// Gets the value of GracePeriodRemaining
    pub fn get_grace_period_remaining(&self) -> Option<&u32> {
        self.grace_period_remaining.as_ref()
    }

    /// Sets the value of IAID
    pub fn set_iaid(&mut self, value: String) {
        self.iaid = Some(value);
    }

    /// Gets the value of IAID
    pub fn get_iaid(&self) -> Option<&String> {
        self.iaid.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
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

    /// Sets the value of KeyManagementServiceFailedRequests
    pub fn set_key_management_service_failed_requests(&mut self, value: u32) {
        self.key_management_service_failed_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceFailedRequests
    pub fn get_key_management_service_failed_requests(&self) -> Option<&u32> {
        self.key_management_service_failed_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceLicensedRequests
    pub fn set_key_management_service_licensed_requests(&mut self, value: u32) {
        self.key_management_service_licensed_requests = Some(value);
    }

    /// Gets the value of KeyManagementServiceLicensedRequests
    pub fn get_key_management_service_licensed_requests(&self) -> Option<&u32> {
        self.key_management_service_licensed_requests.as_ref()
    }

    /// Sets the value of KeyManagementServiceLookupDomain
    pub fn set_key_management_service_lookup_domain(&mut self, value: String) {
        self.key_management_service_lookup_domain = Some(value);
    }

    /// Gets the value of KeyManagementServiceLookupDomain
    pub fn get_key_management_service_lookup_domain(&self) -> Option<&String> {
        self.key_management_service_lookup_domain.as_ref()
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

    /// Sets the value of LicenseDependsOn
    pub fn set_license_depends_on(&mut self, value: String) {
        self.license_depends_on = Some(value);
    }

    /// Gets the value of LicenseDependsOn
    pub fn get_license_depends_on(&self) -> Option<&String> {
        self.license_depends_on.as_ref()
    }

    /// Sets the value of LicenseFamily
    pub fn set_license_family(&mut self, value: String) {
        self.license_family = Some(value);
    }

    /// Gets the value of LicenseFamily
    pub fn get_license_family(&self) -> Option<&String> {
        self.license_family.as_ref()
    }

    /// Sets the value of LicenseIsAddon
    pub fn set_license_is_addon(&mut self, value: bool) {
        self.license_is_addon = Some(value);
    }

    /// Gets the value of LicenseIsAddon
    pub fn get_license_is_addon(&self) -> Option<&bool> {
        self.license_is_addon.as_ref()
    }

    /// Sets the value of LicenseStatus
    pub fn set_license_status(&mut self, value: u32) {
        self.license_status = Some(value);
    }

    /// Gets the value of LicenseStatus
    pub fn get_license_status(&self) -> Option<&u32> {
        self.license_status.as_ref()
    }

    /// Sets the value of LicenseStatusReason
    pub fn set_license_status_reason(&mut self, value: u32) {
        self.license_status_reason = Some(value);
    }

    /// Gets the value of LicenseStatusReason
    pub fn get_license_status_reason(&self) -> Option<&u32> {
        self.license_status_reason.as_ref()
    }

    /// Sets the value of MachineURL
    pub fn set_machine_url(&mut self, value: String) {
        self.machine_url = Some(value);
    }

    /// Gets the value of MachineURL
    pub fn get_machine_url(&self) -> Option<&String> {
        self.machine_url.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OfflineInstallationId
    pub fn set_offline_installation_id(&mut self, value: String) {
        self.offline_installation_id = Some(value);
    }

    /// Gets the value of OfflineInstallationId
    pub fn get_offline_installation_id(&self) -> Option<&String> {
        self.offline_installation_id.as_ref()
    }

    /// Sets the value of PartialProductKey
    pub fn set_partial_product_key(&mut self, value: String) {
        self.partial_product_key = Some(value);
    }

    /// Gets the value of PartialProductKey
    pub fn get_partial_product_key(&self) -> Option<&String> {
        self.partial_product_key.as_ref()
    }

    /// Sets the value of ProcessorURL
    pub fn set_processor_url(&mut self, value: String) {
        self.processor_url = Some(value);
    }

    /// Gets the value of ProcessorURL
    pub fn get_processor_url(&self) -> Option<&String> {
        self.processor_url.as_ref()
    }

    /// Sets the value of ProductKeyChannel
    pub fn set_product_key_channel(&mut self, value: String) {
        self.product_key_channel = Some(value);
    }

    /// Gets the value of ProductKeyChannel
    pub fn get_product_key_channel(&self) -> Option<&String> {
        self.product_key_channel.as_ref()
    }

    /// Sets the value of ProductKeyID
    pub fn set_product_key_id(&mut self, value: String) {
        self.product_key_id = Some(value);
    }

    /// Gets the value of ProductKeyID
    pub fn get_product_key_id(&self) -> Option<&String> {
        self.product_key_id.as_ref()
    }

    /// Sets the value of ProductKeyID2
    pub fn set_product_key_id2(&mut self, value: String) {
        self.product_key_id2 = Some(value);
    }

    /// Gets the value of ProductKeyID2
    pub fn get_product_key_id2(&self) -> Option<&String> {
        self.product_key_id2.as_ref()
    }

    /// Sets the value of ProductKeyURL
    pub fn set_product_key_url(&mut self, value: String) {
        self.product_key_url = Some(value);
    }

    /// Gets the value of ProductKeyURL
    pub fn get_product_key_url(&self) -> Option<&String> {
        self.product_key_url.as_ref()
    }

    /// Sets the value of RemainingAppReArmCount
    pub fn set_remaining_app_re_arm_count(&mut self, value: u32) {
        self.remaining_app_re_arm_count = Some(value);
    }

    /// Gets the value of RemainingAppReArmCount
    pub fn get_remaining_app_re_arm_count(&self) -> Option<&u32> {
        self.remaining_app_re_arm_count.as_ref()
    }

    /// Sets the value of RemainingSkuReArmCount
    pub fn set_remaining_sku_re_arm_count(&mut self, value: u32) {
        self.remaining_sku_re_arm_count = Some(value);
    }

    /// Gets the value of RemainingSkuReArmCount
    pub fn get_remaining_sku_re_arm_count(&self) -> Option<&u32> {
        self.remaining_sku_re_arm_count.as_ref()
    }

    /// Sets the value of RequiredClientCount
    pub fn set_required_client_count(&mut self, value: u32) {
        self.required_client_count = Some(value);
    }

    /// Gets the value of RequiredClientCount
    pub fn get_required_client_count(&self) -> Option<&u32> {
        self.required_client_count.as_ref()
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

    /// Sets the value of TrustedTime
    pub fn set_trusted_time(&mut self, value: String) {
        self.trusted_time = Some(value);
    }

    /// Gets the value of TrustedTime
    pub fn get_trusted_time(&self) -> Option<&String> {
        self.trusted_time.as_ref()
    }

    /// Sets the value of UseLicenseURL
    pub fn set_use_license_url(&mut self, value: String) {
        self.use_license_url = Some(value);
    }

    /// Gets the value of UseLicenseURL
    pub fn get_use_license_url(&self) -> Option<&String> {
        self.use_license_url.as_ref()
    }

    /// Sets the value of ValidationURL
    pub fn set_validation_url(&mut self, value: String) {
        self.validation_url = Some(value);
    }

    /// Gets the value of ValidationURL
    pub fn get_validation_url(&self) -> Option<&String> {
        self.validation_url.as_ref()
    }

    /// Sets the value of VLActivationInterval
    pub fn set_vlactivation_interval(&mut self, value: u32) {
        self.vlactivation_interval = Some(value);
    }

    /// Gets the value of VLActivationInterval
    pub fn get_vlactivation_interval(&self) -> Option<&u32> {
        self.vlactivation_interval.as_ref()
    }

    /// Sets the value of VLActivationType
    pub fn set_vlactivation_type(&mut self, value: u32) {
        self.vlactivation_type = Some(value);
    }

    /// Gets the value of VLActivationType
    pub fn get_vlactivation_type(&self) -> Option<&u32> {
        self.vlactivation_type.as_ref()
    }

    /// Sets the value of VLActivationTypeEnabled
    pub fn set_vlactivation_type_enabled(&mut self, value: u32) {
        self.vlactivation_type_enabled = Some(value);
    }

    /// Gets the value of VLActivationTypeEnabled
    pub fn get_vlactivation_type_enabled(&self) -> Option<&u32> {
        self.vlactivation_type_enabled.as_ref()
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

    /// * `return_value` -  (u32)
    pub fn uninstall_product_key(&self) -> Result<(), WmiError> {
        self.invoke_method("UninstallProductKey", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn activate(&self) -> Result<(), WmiError> {
        self.invoke_method("Activate", &[])

    }


/// 

    /// * `confirmation_id` -  (String)
    /// * `installation_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn deposit_offline_confirmation_id(&self, installation_id: &String, confirmation_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InstallationId".to_string(), value: installation_id.into() });
        args.push(MethodParameter { name: "ConfirmationId".to_string(), value: confirmation_id.into() });
        self.invoke_method("DepositOfflineConfirmationId", &args)

    }


/// 

    /// * `policy_name` -  (String)

    /// * `policy_value` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_policy_information_dword(&self, policy_name: &String, policy_value: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PolicyName".to_string(), value: policy_name.into() });

        let result = self.invoke_method("GetPolicyInformationDWord", &args)?;
        let policy_value = result.get_value("PolicyValue")?;
        Ok(result.return_value)

    }


/// 

    /// * `policy_name` -  (String)

    /// * `policy_value` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_policy_information_string(&self, policy_name: &String, policy_value: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PolicyName".to_string(), value: policy_name.into() });

        let result = self.invoke_method("GetPolicyInformationString", &args)?;
        let policy_value = result.get_value("PolicyValue")?;
        Ok(result.return_value)

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

    /// * `grants` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_token_activation_grants(&self, grants: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetTokenActivationGrants", &[])?;
        let grants = result.get_value("Grants")?;
        Ok(result.return_value)

    }


/// 

    /// * `challenge` -  (String)
    /// * `return_value` -  (u32)
    pub fn generate_token_activation_challenge(&self, challenge: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GenerateTokenActivationChallenge", &[])?;
        let challenge = result.get_value("Challenge")?;
        Ok(result.return_value)

    }


/// 

    /// * `cert_chain` -  (String)
    /// * `challenge` -  (String)
    /// * `response` -  (String)

    /// * `return_value` -  (u32)
    pub fn deposit_token_activation_response(&self, challenge: &String, response: &String, cert_chain: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Challenge".to_string(), value: challenge.into() });
        args.push(MethodParameter { name: "Response".to_string(), value: response.into() });
        args.push(MethodParameter { name: "CertChain".to_string(), value: cert_chain.into() });
        self.invoke_method("DepositTokenActivationResponse", &args)

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


/// 

    /// * `return_value` -  (u32)
    pub fn re_arm_sku(&self) -> Result<(), WmiError> {
        self.invoke_method("ReArmSku", &[])

    }

}

