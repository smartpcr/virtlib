// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTDomain {
    #[serde(flatten)]
    pub base: CIM_System,

/// 
    #[serde(rename = "ClientSiteName")]
    pub client_site_name: Option<String>,

/// 
    #[serde(rename = "DcSiteName")]
    pub dc_site_name: Option<String>,

/// 
    #[serde(rename = "DnsForestName")]
    pub dns_forest_name: Option<String>,

/// 
    #[serde(rename = "DomainControllerAddress")]
    pub domain_controller_address: Option<String>,

/// 
    #[serde(rename = "DomainControllerAddressType")]
    pub domain_controller_address_type: Option<i32>,

/// 
    #[serde(rename = "DomainControllerName")]
    pub domain_controller_name: Option<String>,

/// 
    #[serde(rename = "DomainGuid")]
    pub domain_guid: Option<String>,

/// 
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// 
    #[serde(rename = "DSDirectoryServiceFlag")]
    pub dsdirectory_service_flag: Option<bool>,

/// 
    #[serde(rename = "DSDnsControllerFlag")]
    pub dsdns_controller_flag: Option<bool>,

/// 
    #[serde(rename = "DSDnsDomainFlag")]
    pub dsdns_domain_flag: Option<bool>,

/// 
    #[serde(rename = "DSDnsForestFlag")]
    pub dsdns_forest_flag: Option<bool>,

/// 
    #[serde(rename = "DSGlobalCatalogFlag")]
    pub dsglobal_catalog_flag: Option<bool>,

/// 
    #[serde(rename = "DSKerberosDistributionCenterFlag")]
    pub dskerberos_distribution_center_flag: Option<bool>,

/// 
    #[serde(rename = "DSPrimaryDomainControllerFlag")]
    pub dsprimary_domain_controller_flag: Option<bool>,

/// 
    #[serde(rename = "DSTimeServiceFlag")]
    pub dstime_service_flag: Option<bool>,

/// 
    #[serde(rename = "DSWritableFlag")]
    pub dswritable_flag: Option<bool>,
}

impl Win32_NTDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_System::new(),
            client_site_name: None,
            dc_site_name: None,
            dns_forest_name: None,
            domain_controller_address: None,
            domain_controller_address_type: None,
            domain_controller_name: None,
            domain_guid: None,
            domain_name: None,
            dsdirectory_service_flag: None,
            dsdns_controller_flag: None,
            dsdns_domain_flag: None,
            dsdns_forest_flag: None,
            dsglobal_catalog_flag: None,
            dskerberos_distribution_center_flag: None,
            dsprimary_domain_controller_flag: None,
            dstime_service_flag: None,
            dswritable_flag: None,
        }
    }


    /// Sets the value of ClientSiteName
    pub fn set_client_site_name(&mut self, value: String) {
        self.client_site_name = Some(value);
    }

    /// Gets the value of ClientSiteName
    pub fn get_client_site_name(&self) -> Option<&String> {
        self.client_site_name.as_ref()
    }

    /// Sets the value of DcSiteName
    pub fn set_dc_site_name(&mut self, value: String) {
        self.dc_site_name = Some(value);
    }

    /// Gets the value of DcSiteName
    pub fn get_dc_site_name(&self) -> Option<&String> {
        self.dc_site_name.as_ref()
    }

    /// Sets the value of DnsForestName
    pub fn set_dns_forest_name(&mut self, value: String) {
        self.dns_forest_name = Some(value);
    }

    /// Gets the value of DnsForestName
    pub fn get_dns_forest_name(&self) -> Option<&String> {
        self.dns_forest_name.as_ref()
    }

    /// Sets the value of DomainControllerAddress
    pub fn set_domain_controller_address(&mut self, value: String) {
        self.domain_controller_address = Some(value);
    }

    /// Gets the value of DomainControllerAddress
    pub fn get_domain_controller_address(&self) -> Option<&String> {
        self.domain_controller_address.as_ref()
    }

    /// Sets the value of DomainControllerAddressType
    pub fn set_domain_controller_address_type(&mut self, value: i32) {
        self.domain_controller_address_type = Some(value);
    }

    /// Gets the value of DomainControllerAddressType
    pub fn get_domain_controller_address_type(&self) -> Option<&i32> {
        self.domain_controller_address_type.as_ref()
    }

    /// Sets the value of DomainControllerName
    pub fn set_domain_controller_name(&mut self, value: String) {
        self.domain_controller_name = Some(value);
    }

    /// Gets the value of DomainControllerName
    pub fn get_domain_controller_name(&self) -> Option<&String> {
        self.domain_controller_name.as_ref()
    }

    /// Sets the value of DomainGuid
    pub fn set_domain_guid(&mut self, value: String) {
        self.domain_guid = Some(value);
    }

    /// Gets the value of DomainGuid
    pub fn get_domain_guid(&self) -> Option<&String> {
        self.domain_guid.as_ref()
    }

    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    /// Sets the value of DSDirectoryServiceFlag
    pub fn set_dsdirectory_service_flag(&mut self, value: bool) {
        self.dsdirectory_service_flag = Some(value);
    }

    /// Gets the value of DSDirectoryServiceFlag
    pub fn get_dsdirectory_service_flag(&self) -> Option<&bool> {
        self.dsdirectory_service_flag.as_ref()
    }

    /// Sets the value of DSDnsControllerFlag
    pub fn set_dsdns_controller_flag(&mut self, value: bool) {
        self.dsdns_controller_flag = Some(value);
    }

    /// Gets the value of DSDnsControllerFlag
    pub fn get_dsdns_controller_flag(&self) -> Option<&bool> {
        self.dsdns_controller_flag.as_ref()
    }

    /// Sets the value of DSDnsDomainFlag
    pub fn set_dsdns_domain_flag(&mut self, value: bool) {
        self.dsdns_domain_flag = Some(value);
    }

    /// Gets the value of DSDnsDomainFlag
    pub fn get_dsdns_domain_flag(&self) -> Option<&bool> {
        self.dsdns_domain_flag.as_ref()
    }

    /// Sets the value of DSDnsForestFlag
    pub fn set_dsdns_forest_flag(&mut self, value: bool) {
        self.dsdns_forest_flag = Some(value);
    }

    /// Gets the value of DSDnsForestFlag
    pub fn get_dsdns_forest_flag(&self) -> Option<&bool> {
        self.dsdns_forest_flag.as_ref()
    }

    /// Sets the value of DSGlobalCatalogFlag
    pub fn set_dsglobal_catalog_flag(&mut self, value: bool) {
        self.dsglobal_catalog_flag = Some(value);
    }

    /// Gets the value of DSGlobalCatalogFlag
    pub fn get_dsglobal_catalog_flag(&self) -> Option<&bool> {
        self.dsglobal_catalog_flag.as_ref()
    }

    /// Sets the value of DSKerberosDistributionCenterFlag
    pub fn set_dskerberos_distribution_center_flag(&mut self, value: bool) {
        self.dskerberos_distribution_center_flag = Some(value);
    }

    /// Gets the value of DSKerberosDistributionCenterFlag
    pub fn get_dskerberos_distribution_center_flag(&self) -> Option<&bool> {
        self.dskerberos_distribution_center_flag.as_ref()
    }

    /// Sets the value of DSPrimaryDomainControllerFlag
    pub fn set_dsprimary_domain_controller_flag(&mut self, value: bool) {
        self.dsprimary_domain_controller_flag = Some(value);
    }

    /// Gets the value of DSPrimaryDomainControllerFlag
    pub fn get_dsprimary_domain_controller_flag(&self) -> Option<&bool> {
        self.dsprimary_domain_controller_flag.as_ref()
    }

    /// Sets the value of DSTimeServiceFlag
    pub fn set_dstime_service_flag(&mut self, value: bool) {
        self.dstime_service_flag = Some(value);
    }

    /// Gets the value of DSTimeServiceFlag
    pub fn get_dstime_service_flag(&self) -> Option<&bool> {
        self.dstime_service_flag.as_ref()
    }

    /// Sets the value of DSWritableFlag
    pub fn set_dswritable_flag(&mut self, value: bool) {
        self.dswritable_flag = Some(value);
    }

    /// Gets the value of DSWritableFlag
    pub fn get_dswritable_flag(&self) -> Option<&bool> {
        self.dswritable_flag.as_ref()
    }
}

