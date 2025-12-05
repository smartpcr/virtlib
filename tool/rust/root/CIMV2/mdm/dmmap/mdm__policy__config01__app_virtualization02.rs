// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_AppVirtualization02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_AppVirtualization02 {

/// 
    #[serde(rename = "AllowAppVClient")]
    pub allow_app_vclient: Option<String>,

/// 
    #[serde(rename = "AllowDynamicVirtualization")]
    pub allow_dynamic_virtualization: Option<String>,

/// 
    #[serde(rename = "AllowPackageCleanup")]
    pub allow_package_cleanup: Option<String>,

/// 
    #[serde(rename = "AllowPackageScripts")]
    pub allow_package_scripts: Option<String>,

/// 
    #[serde(rename = "AllowPublishingRefreshUX")]
    pub allow_publishing_refresh_ux: Option<String>,

/// 
    #[serde(rename = "AllowReportingServer")]
    pub allow_reporting_server: Option<String>,

/// 
    #[serde(rename = "AllowRoamingFileExclusions")]
    pub allow_roaming_file_exclusions: Option<String>,

/// 
    #[serde(rename = "AllowRoamingRegistryExclusions")]
    pub allow_roaming_registry_exclusions: Option<String>,

/// 
    #[serde(rename = "AllowStreamingAutoload")]
    pub allow_streaming_autoload: Option<String>,

/// 
    #[serde(rename = "ClientCoexistenceAllowMigrationmode")]
    pub client_coexistence_allow_migrationmode: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IntegrationAllowRootGlobal")]
    pub integration_allow_root_global: Option<String>,

/// 
    #[serde(rename = "IntegrationAllowRootUser")]
    pub integration_allow_root_user: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PublishingAllowServer1")]
    pub publishing_allow_server1: Option<String>,

/// 
    #[serde(rename = "PublishingAllowServer2")]
    pub publishing_allow_server2: Option<String>,

/// 
    #[serde(rename = "PublishingAllowServer3")]
    pub publishing_allow_server3: Option<String>,

/// 
    #[serde(rename = "PublishingAllowServer4")]
    pub publishing_allow_server4: Option<String>,

/// 
    #[serde(rename = "PublishingAllowServer5")]
    pub publishing_allow_server5: Option<String>,

/// 
    #[serde(rename = "StreamingAllowCertificateFilterForClient_SSL")]
    pub streaming_allow_certificate_filter_for_client__ssl: Option<String>,

/// 
    #[serde(rename = "StreamingAllowHighCostLaunch")]
    pub streaming_allow_high_cost_launch: Option<String>,

/// 
    #[serde(rename = "StreamingAllowLocationProvider")]
    pub streaming_allow_location_provider: Option<String>,

/// 
    #[serde(rename = "StreamingAllowPackageInstallationRoot")]
    pub streaming_allow_package_installation_root: Option<String>,

/// 
    #[serde(rename = "StreamingAllowPackageSourceRoot")]
    pub streaming_allow_package_source_root: Option<String>,

/// 
    #[serde(rename = "StreamingAllowReestablishmentInterval")]
    pub streaming_allow_reestablishment_interval: Option<String>,

/// 
    #[serde(rename = "StreamingAllowReestablishmentRetries")]
    pub streaming_allow_reestablishment_retries: Option<String>,

/// 
    #[serde(rename = "StreamingSharedContentStoreMode")]
    pub streaming_shared_content_store_mode: Option<String>,

/// 
    #[serde(rename = "StreamingSupportBranchCache")]
    pub streaming_support_branch_cache: Option<String>,

/// 
    #[serde(rename = "StreamingVerifyCertificateRevocationList")]
    pub streaming_verify_certificate_revocation_list: Option<String>,

/// 
    #[serde(rename = "VirtualComponentsAllowList")]
    pub virtual_components_allow_list: Option<String>,
}

impl MDM_Policy_Config01_AppVirtualization02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_app_vclient: None,
            allow_dynamic_virtualization: None,
            allow_package_cleanup: None,
            allow_package_scripts: None,
            allow_publishing_refresh_ux: None,
            allow_reporting_server: None,
            allow_roaming_file_exclusions: None,
            allow_roaming_registry_exclusions: None,
            allow_streaming_autoload: None,
            client_coexistence_allow_migrationmode: None,
            instance_id: None,
            integration_allow_root_global: None,
            integration_allow_root_user: None,
            parent_id: None,
            publishing_allow_server1: None,
            publishing_allow_server2: None,
            publishing_allow_server3: None,
            publishing_allow_server4: None,
            publishing_allow_server5: None,
            streaming_allow_certificate_filter_for_client__ssl: None,
            streaming_allow_high_cost_launch: None,
            streaming_allow_location_provider: None,
            streaming_allow_package_installation_root: None,
            streaming_allow_package_source_root: None,
            streaming_allow_reestablishment_interval: None,
            streaming_allow_reestablishment_retries: None,
            streaming_shared_content_store_mode: None,
            streaming_support_branch_cache: None,
            streaming_verify_certificate_revocation_list: None,
            virtual_components_allow_list: None,
        }
    }


    /// Sets the value of AllowAppVClient
    pub fn set_allow_app_vclient(&mut self, value: String) {
        self.allow_app_vclient = Some(value);
    }

    /// Gets the value of AllowAppVClient
    pub fn get_allow_app_vclient(&self) -> Option<&String> {
        self.allow_app_vclient.as_ref()
    }

    /// Sets the value of AllowDynamicVirtualization
    pub fn set_allow_dynamic_virtualization(&mut self, value: String) {
        self.allow_dynamic_virtualization = Some(value);
    }

    /// Gets the value of AllowDynamicVirtualization
    pub fn get_allow_dynamic_virtualization(&self) -> Option<&String> {
        self.allow_dynamic_virtualization.as_ref()
    }

    /// Sets the value of AllowPackageCleanup
    pub fn set_allow_package_cleanup(&mut self, value: String) {
        self.allow_package_cleanup = Some(value);
    }

    /// Gets the value of AllowPackageCleanup
    pub fn get_allow_package_cleanup(&self) -> Option<&String> {
        self.allow_package_cleanup.as_ref()
    }

    /// Sets the value of AllowPackageScripts
    pub fn set_allow_package_scripts(&mut self, value: String) {
        self.allow_package_scripts = Some(value);
    }

    /// Gets the value of AllowPackageScripts
    pub fn get_allow_package_scripts(&self) -> Option<&String> {
        self.allow_package_scripts.as_ref()
    }

    /// Sets the value of AllowPublishingRefreshUX
    pub fn set_allow_publishing_refresh_ux(&mut self, value: String) {
        self.allow_publishing_refresh_ux = Some(value);
    }

    /// Gets the value of AllowPublishingRefreshUX
    pub fn get_allow_publishing_refresh_ux(&self) -> Option<&String> {
        self.allow_publishing_refresh_ux.as_ref()
    }

    /// Sets the value of AllowReportingServer
    pub fn set_allow_reporting_server(&mut self, value: String) {
        self.allow_reporting_server = Some(value);
    }

    /// Gets the value of AllowReportingServer
    pub fn get_allow_reporting_server(&self) -> Option<&String> {
        self.allow_reporting_server.as_ref()
    }

    /// Sets the value of AllowRoamingFileExclusions
    pub fn set_allow_roaming_file_exclusions(&mut self, value: String) {
        self.allow_roaming_file_exclusions = Some(value);
    }

    /// Gets the value of AllowRoamingFileExclusions
    pub fn get_allow_roaming_file_exclusions(&self) -> Option<&String> {
        self.allow_roaming_file_exclusions.as_ref()
    }

    /// Sets the value of AllowRoamingRegistryExclusions
    pub fn set_allow_roaming_registry_exclusions(&mut self, value: String) {
        self.allow_roaming_registry_exclusions = Some(value);
    }

    /// Gets the value of AllowRoamingRegistryExclusions
    pub fn get_allow_roaming_registry_exclusions(&self) -> Option<&String> {
        self.allow_roaming_registry_exclusions.as_ref()
    }

    /// Sets the value of AllowStreamingAutoload
    pub fn set_allow_streaming_autoload(&mut self, value: String) {
        self.allow_streaming_autoload = Some(value);
    }

    /// Gets the value of AllowStreamingAutoload
    pub fn get_allow_streaming_autoload(&self) -> Option<&String> {
        self.allow_streaming_autoload.as_ref()
    }

    /// Sets the value of ClientCoexistenceAllowMigrationmode
    pub fn set_client_coexistence_allow_migrationmode(&mut self, value: String) {
        self.client_coexistence_allow_migrationmode = Some(value);
    }

    /// Gets the value of ClientCoexistenceAllowMigrationmode
    pub fn get_client_coexistence_allow_migrationmode(&self) -> Option<&String> {
        self.client_coexistence_allow_migrationmode.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IntegrationAllowRootGlobal
    pub fn set_integration_allow_root_global(&mut self, value: String) {
        self.integration_allow_root_global = Some(value);
    }

    /// Gets the value of IntegrationAllowRootGlobal
    pub fn get_integration_allow_root_global(&self) -> Option<&String> {
        self.integration_allow_root_global.as_ref()
    }

    /// Sets the value of IntegrationAllowRootUser
    pub fn set_integration_allow_root_user(&mut self, value: String) {
        self.integration_allow_root_user = Some(value);
    }

    /// Gets the value of IntegrationAllowRootUser
    pub fn get_integration_allow_root_user(&self) -> Option<&String> {
        self.integration_allow_root_user.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PublishingAllowServer1
    pub fn set_publishing_allow_server1(&mut self, value: String) {
        self.publishing_allow_server1 = Some(value);
    }

    /// Gets the value of PublishingAllowServer1
    pub fn get_publishing_allow_server1(&self) -> Option<&String> {
        self.publishing_allow_server1.as_ref()
    }

    /// Sets the value of PublishingAllowServer2
    pub fn set_publishing_allow_server2(&mut self, value: String) {
        self.publishing_allow_server2 = Some(value);
    }

    /// Gets the value of PublishingAllowServer2
    pub fn get_publishing_allow_server2(&self) -> Option<&String> {
        self.publishing_allow_server2.as_ref()
    }

    /// Sets the value of PublishingAllowServer3
    pub fn set_publishing_allow_server3(&mut self, value: String) {
        self.publishing_allow_server3 = Some(value);
    }

    /// Gets the value of PublishingAllowServer3
    pub fn get_publishing_allow_server3(&self) -> Option<&String> {
        self.publishing_allow_server3.as_ref()
    }

    /// Sets the value of PublishingAllowServer4
    pub fn set_publishing_allow_server4(&mut self, value: String) {
        self.publishing_allow_server4 = Some(value);
    }

    /// Gets the value of PublishingAllowServer4
    pub fn get_publishing_allow_server4(&self) -> Option<&String> {
        self.publishing_allow_server4.as_ref()
    }

    /// Sets the value of PublishingAllowServer5
    pub fn set_publishing_allow_server5(&mut self, value: String) {
        self.publishing_allow_server5 = Some(value);
    }

    /// Gets the value of PublishingAllowServer5
    pub fn get_publishing_allow_server5(&self) -> Option<&String> {
        self.publishing_allow_server5.as_ref()
    }

    /// Sets the value of StreamingAllowCertificateFilterForClient_SSL
    pub fn set_streaming_allow_certificate_filter_for_client__ssl(&mut self, value: String) {
        self.streaming_allow_certificate_filter_for_client__ssl = Some(value);
    }

    /// Gets the value of StreamingAllowCertificateFilterForClient_SSL
    pub fn get_streaming_allow_certificate_filter_for_client__ssl(&self) -> Option<&String> {
        self.streaming_allow_certificate_filter_for_client__ssl.as_ref()
    }

    /// Sets the value of StreamingAllowHighCostLaunch
    pub fn set_streaming_allow_high_cost_launch(&mut self, value: String) {
        self.streaming_allow_high_cost_launch = Some(value);
    }

    /// Gets the value of StreamingAllowHighCostLaunch
    pub fn get_streaming_allow_high_cost_launch(&self) -> Option<&String> {
        self.streaming_allow_high_cost_launch.as_ref()
    }

    /// Sets the value of StreamingAllowLocationProvider
    pub fn set_streaming_allow_location_provider(&mut self, value: String) {
        self.streaming_allow_location_provider = Some(value);
    }

    /// Gets the value of StreamingAllowLocationProvider
    pub fn get_streaming_allow_location_provider(&self) -> Option<&String> {
        self.streaming_allow_location_provider.as_ref()
    }

    /// Sets the value of StreamingAllowPackageInstallationRoot
    pub fn set_streaming_allow_package_installation_root(&mut self, value: String) {
        self.streaming_allow_package_installation_root = Some(value);
    }

    /// Gets the value of StreamingAllowPackageInstallationRoot
    pub fn get_streaming_allow_package_installation_root(&self) -> Option<&String> {
        self.streaming_allow_package_installation_root.as_ref()
    }

    /// Sets the value of StreamingAllowPackageSourceRoot
    pub fn set_streaming_allow_package_source_root(&mut self, value: String) {
        self.streaming_allow_package_source_root = Some(value);
    }

    /// Gets the value of StreamingAllowPackageSourceRoot
    pub fn get_streaming_allow_package_source_root(&self) -> Option<&String> {
        self.streaming_allow_package_source_root.as_ref()
    }

    /// Sets the value of StreamingAllowReestablishmentInterval
    pub fn set_streaming_allow_reestablishment_interval(&mut self, value: String) {
        self.streaming_allow_reestablishment_interval = Some(value);
    }

    /// Gets the value of StreamingAllowReestablishmentInterval
    pub fn get_streaming_allow_reestablishment_interval(&self) -> Option<&String> {
        self.streaming_allow_reestablishment_interval.as_ref()
    }

    /// Sets the value of StreamingAllowReestablishmentRetries
    pub fn set_streaming_allow_reestablishment_retries(&mut self, value: String) {
        self.streaming_allow_reestablishment_retries = Some(value);
    }

    /// Gets the value of StreamingAllowReestablishmentRetries
    pub fn get_streaming_allow_reestablishment_retries(&self) -> Option<&String> {
        self.streaming_allow_reestablishment_retries.as_ref()
    }

    /// Sets the value of StreamingSharedContentStoreMode
    pub fn set_streaming_shared_content_store_mode(&mut self, value: String) {
        self.streaming_shared_content_store_mode = Some(value);
    }

    /// Gets the value of StreamingSharedContentStoreMode
    pub fn get_streaming_shared_content_store_mode(&self) -> Option<&String> {
        self.streaming_shared_content_store_mode.as_ref()
    }

    /// Sets the value of StreamingSupportBranchCache
    pub fn set_streaming_support_branch_cache(&mut self, value: String) {
        self.streaming_support_branch_cache = Some(value);
    }

    /// Gets the value of StreamingSupportBranchCache
    pub fn get_streaming_support_branch_cache(&self) -> Option<&String> {
        self.streaming_support_branch_cache.as_ref()
    }

    /// Sets the value of StreamingVerifyCertificateRevocationList
    pub fn set_streaming_verify_certificate_revocation_list(&mut self, value: String) {
        self.streaming_verify_certificate_revocation_list = Some(value);
    }

    /// Gets the value of StreamingVerifyCertificateRevocationList
    pub fn get_streaming_verify_certificate_revocation_list(&self) -> Option<&String> {
        self.streaming_verify_certificate_revocation_list.as_ref()
    }

    /// Sets the value of VirtualComponentsAllowList
    pub fn set_virtual_components_allow_list(&mut self, value: String) {
        self.virtual_components_allow_list = Some(value);
    }

    /// Gets the value of VirtualComponentsAllowList
    pub fn get_virtual_components_allow_list(&self) -> Option<&String> {
        self.virtual_components_allow_list.as_ref()
    }
}

