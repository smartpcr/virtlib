// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NCSIPolicyConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NCSIPolicyConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "CorporateDNSProbeHostAddress")]
    pub corporate_dnsprobe_host_address: Option<String>,

/// 
    #[serde(rename = "CorporateDNSProbeHostName")]
    pub corporate_dnsprobe_host_name: Option<String>,

/// 
    #[serde(rename = "CorporateSitePrefixList")]
    pub corporate_site_prefix_list: Vec<String>,

/// 
    #[serde(rename = "CorporateWebsiteProbeURL")]
    pub corporate_website_probe_url: Option<String>,

/// 
    #[serde(rename = "DomainLocationDeterminationURL")]
    pub domain_location_determination_url: Option<String>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,
}

impl MSFT_NCSIPolicyConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            corporate_dnsprobe_host_address: None,
            corporate_dnsprobe_host_name: None,
            corporate_site_prefix_list: Vec::new(),
            corporate_website_probe_url: None,
            domain_location_determination_url: None,
            policy_store: None,
        }
    }


    /// Sets the value of CorporateDNSProbeHostAddress
    pub fn set_corporate_dnsprobe_host_address(&mut self, value: String) {
        self.corporate_dnsprobe_host_address = Some(value);
    }

    /// Gets the value of CorporateDNSProbeHostAddress
    pub fn get_corporate_dnsprobe_host_address(&self) -> Option<&String> {
        self.corporate_dnsprobe_host_address.as_ref()
    }

    /// Sets the value of CorporateDNSProbeHostName
    pub fn set_corporate_dnsprobe_host_name(&mut self, value: String) {
        self.corporate_dnsprobe_host_name = Some(value);
    }

    /// Gets the value of CorporateDNSProbeHostName
    pub fn get_corporate_dnsprobe_host_name(&self) -> Option<&String> {
        self.corporate_dnsprobe_host_name.as_ref()
    }

    /// Sets the value of CorporateSitePrefixList
    pub fn set_corporate_site_prefix_list(&mut self, value: Vec<String>) {
        self.corporate_site_prefix_list = value;
    }

    /// Gets the value of CorporateSitePrefixList
    pub fn get_corporate_site_prefix_list(&self) -> &Vec<String> {
        &self.corporate_site_prefix_list
    }

    /// Sets the value of CorporateWebsiteProbeURL
    pub fn set_corporate_website_probe_url(&mut self, value: String) {
        self.corporate_website_probe_url = Some(value);
    }

    /// Gets the value of CorporateWebsiteProbeURL
    pub fn get_corporate_website_probe_url(&self) -> Option<&String> {
        self.corporate_website_probe_url.as_ref()
    }

    /// Sets the value of DomainLocationDeterminationURL
    pub fn set_domain_location_determination_url(&mut self, value: String) {
        self.domain_location_determination_url = Some(value);
    }

    /// Gets the value of DomainLocationDeterminationURL
    pub fn get_domain_location_determination_url(&self) -> Option<&String> {
        self.domain_location_determination_url.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

/// 

    /// * `corporate_dnsprobe_host_address` -  (bool)
    /// * `corporate_dnsprobe_host_name` -  (bool)
    /// * `corporate_site_prefix_list` -  (bool)
    /// * `corporate_website_probe_url` -  (bool)
    /// * `domain_location_determination_url` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_NCSIPolicyConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, corporate_dnsprobe_host_address: bool, corporate_dnsprobe_host_name: bool, corporate_site_prefix_list: bool, corporate_website_probe_url: bool, domain_location_determination_url: bool, pass_thru: bool, output_object: &mut MSFT_NCSIPolicyConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CorporateDNSProbeHostAddress".to_string(), value: corporate_dnsprobe_host_address.into() });
        args.push(MethodParameter { name: "CorporateDNSProbeHostName".to_string(), value: corporate_dnsprobe_host_name.into() });
        args.push(MethodParameter { name: "CorporateSitePrefixList".to_string(), value: corporate_site_prefix_list.into() });
        args.push(MethodParameter { name: "CorporateWebsiteProbeURL".to_string(), value: corporate_website_probe_url.into() });
        args.push(MethodParameter { name: "DomainLocationDeterminationURL".to_string(), value: domain_location_determination_url.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

