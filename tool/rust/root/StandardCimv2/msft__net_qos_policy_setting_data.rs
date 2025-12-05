// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetQosPolicySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetQosPolicySettingData {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "AppPathNameMatchCondition")]
    pub app_path_name_match_condition: Option<String>,

/// 
    #[serde(rename = "DSCPAction")]
    pub dscpaction: Option<u8>,

/// 
    #[serde(rename = "IPDstPortEndMatchCondition")]
    pub ipdst_port_end_match_condition: Option<u16>,

/// 
    #[serde(rename = "IPDstPortStartMatchCondition")]
    pub ipdst_port_start_match_condition: Option<u16>,

/// 
    #[serde(rename = "IPDstPrefixMatchCondition")]
    pub ipdst_prefix_match_condition: Option<String>,

/// 
    #[serde(rename = "IPPortMatchCondition")]
    pub ipport_match_condition: Option<u16>,

/// 
    #[serde(rename = "IPProtocolMatchCondition")]
    pub ipprotocol_match_condition: Option<u32>,

/// 
    #[serde(rename = "IPSrcPortEndMatchCondition")]
    pub ipsrc_port_end_match_condition: Option<u16>,

/// 
    #[serde(rename = "IPSrcPortStartMatchCondition")]
    pub ipsrc_port_start_match_condition: Option<u16>,

/// 
    #[serde(rename = "IPSrcPrefixMatchCondition")]
    pub ipsrc_prefix_match_condition: Option<String>,

/// 
    #[serde(rename = "JobObjectMatchCondition")]
    pub job_object_match_condition: Option<String>,

/// 
    #[serde(rename = "MinBandwidthWeightAction")]
    pub min_bandwidth_weight_action: Option<u8>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NetDirectPortMatchCondition")]
    pub net_direct_port_match_condition: Option<u16>,

/// 
    #[serde(rename = "NetworkProfile")]
    pub network_profile: Option<u32>,

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

/// 
    #[serde(rename = "Precedence")]
    pub precedence: Option<u32>,

/// 
    #[serde(rename = "PriorityValue8021Action")]
    pub priority_value8021_action: Option<u8>,

/// 
    #[serde(rename = "TemplateMatchCondition")]
    pub template_match_condition: Option<u32>,

/// 
    #[serde(rename = "ThrottleRateAction")]
    pub throttle_rate_action: Option<u64>,

/// 
    #[serde(rename = "URIMatchCondition")]
    pub urimatch_condition: Option<String>,

/// 
    #[serde(rename = "URIRecursiveMatchCondition")]
    pub urirecursive_match_condition: Option<bool>,

/// 
    #[serde(rename = "UserMatchCondition")]
    pub user_match_condition: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MSFT_NetQosPolicySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            app_path_name_match_condition: None,
            dscpaction: None,
            ipdst_port_end_match_condition: None,
            ipdst_port_start_match_condition: None,
            ipdst_prefix_match_condition: None,
            ipport_match_condition: None,
            ipprotocol_match_condition: None,
            ipsrc_port_end_match_condition: None,
            ipsrc_port_start_match_condition: None,
            ipsrc_prefix_match_condition: None,
            job_object_match_condition: None,
            min_bandwidth_weight_action: None,
            name: None,
            net_direct_port_match_condition: None,
            network_profile: None,
            owner: None,
            precedence: None,
            priority_value8021_action: None,
            template_match_condition: None,
            throttle_rate_action: None,
            urimatch_condition: None,
            urirecursive_match_condition: None,
            user_match_condition: None,
            version: None,
        }
    }


    /// Sets the value of AppPathNameMatchCondition
    pub fn set_app_path_name_match_condition(&mut self, value: String) {
        self.app_path_name_match_condition = Some(value);
    }

    /// Gets the value of AppPathNameMatchCondition
    pub fn get_app_path_name_match_condition(&self) -> Option<&String> {
        self.app_path_name_match_condition.as_ref()
    }

    /// Sets the value of DSCPAction
    pub fn set_dscpaction(&mut self, value: u8) {
        self.dscpaction = Some(value);
    }

    /// Gets the value of DSCPAction
    pub fn get_dscpaction(&self) -> Option<&u8> {
        self.dscpaction.as_ref()
    }

    /// Sets the value of IPDstPortEndMatchCondition
    pub fn set_ipdst_port_end_match_condition(&mut self, value: u16) {
        self.ipdst_port_end_match_condition = Some(value);
    }

    /// Gets the value of IPDstPortEndMatchCondition
    pub fn get_ipdst_port_end_match_condition(&self) -> Option<&u16> {
        self.ipdst_port_end_match_condition.as_ref()
    }

    /// Sets the value of IPDstPortStartMatchCondition
    pub fn set_ipdst_port_start_match_condition(&mut self, value: u16) {
        self.ipdst_port_start_match_condition = Some(value);
    }

    /// Gets the value of IPDstPortStartMatchCondition
    pub fn get_ipdst_port_start_match_condition(&self) -> Option<&u16> {
        self.ipdst_port_start_match_condition.as_ref()
    }

    /// Sets the value of IPDstPrefixMatchCondition
    pub fn set_ipdst_prefix_match_condition(&mut self, value: String) {
        self.ipdst_prefix_match_condition = Some(value);
    }

    /// Gets the value of IPDstPrefixMatchCondition
    pub fn get_ipdst_prefix_match_condition(&self) -> Option<&String> {
        self.ipdst_prefix_match_condition.as_ref()
    }

    /// Sets the value of IPPortMatchCondition
    pub fn set_ipport_match_condition(&mut self, value: u16) {
        self.ipport_match_condition = Some(value);
    }

    /// Gets the value of IPPortMatchCondition
    pub fn get_ipport_match_condition(&self) -> Option<&u16> {
        self.ipport_match_condition.as_ref()
    }

    /// Sets the value of IPProtocolMatchCondition
    pub fn set_ipprotocol_match_condition(&mut self, value: u32) {
        self.ipprotocol_match_condition = Some(value);
    }

    /// Gets the value of IPProtocolMatchCondition
    pub fn get_ipprotocol_match_condition(&self) -> Option<&u32> {
        self.ipprotocol_match_condition.as_ref()
    }

    /// Sets the value of IPSrcPortEndMatchCondition
    pub fn set_ipsrc_port_end_match_condition(&mut self, value: u16) {
        self.ipsrc_port_end_match_condition = Some(value);
    }

    /// Gets the value of IPSrcPortEndMatchCondition
    pub fn get_ipsrc_port_end_match_condition(&self) -> Option<&u16> {
        self.ipsrc_port_end_match_condition.as_ref()
    }

    /// Sets the value of IPSrcPortStartMatchCondition
    pub fn set_ipsrc_port_start_match_condition(&mut self, value: u16) {
        self.ipsrc_port_start_match_condition = Some(value);
    }

    /// Gets the value of IPSrcPortStartMatchCondition
    pub fn get_ipsrc_port_start_match_condition(&self) -> Option<&u16> {
        self.ipsrc_port_start_match_condition.as_ref()
    }

    /// Sets the value of IPSrcPrefixMatchCondition
    pub fn set_ipsrc_prefix_match_condition(&mut self, value: String) {
        self.ipsrc_prefix_match_condition = Some(value);
    }

    /// Gets the value of IPSrcPrefixMatchCondition
    pub fn get_ipsrc_prefix_match_condition(&self) -> Option<&String> {
        self.ipsrc_prefix_match_condition.as_ref()
    }

    /// Sets the value of JobObjectMatchCondition
    pub fn set_job_object_match_condition(&mut self, value: String) {
        self.job_object_match_condition = Some(value);
    }

    /// Gets the value of JobObjectMatchCondition
    pub fn get_job_object_match_condition(&self) -> Option<&String> {
        self.job_object_match_condition.as_ref()
    }

    /// Sets the value of MinBandwidthWeightAction
    pub fn set_min_bandwidth_weight_action(&mut self, value: u8) {
        self.min_bandwidth_weight_action = Some(value);
    }

    /// Gets the value of MinBandwidthWeightAction
    pub fn get_min_bandwidth_weight_action(&self) -> Option<&u8> {
        self.min_bandwidth_weight_action.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NetDirectPortMatchCondition
    pub fn set_net_direct_port_match_condition(&mut self, value: u16) {
        self.net_direct_port_match_condition = Some(value);
    }

    /// Gets the value of NetDirectPortMatchCondition
    pub fn get_net_direct_port_match_condition(&self) -> Option<&u16> {
        self.net_direct_port_match_condition.as_ref()
    }

    /// Sets the value of NetworkProfile
    pub fn set_network_profile(&mut self, value: u32) {
        self.network_profile = Some(value);
    }

    /// Gets the value of NetworkProfile
    pub fn get_network_profile(&self) -> Option<&u32> {
        self.network_profile.as_ref()
    }

    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: String) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&String> {
        self.owner.as_ref()
    }

    /// Sets the value of Precedence
    pub fn set_precedence(&mut self, value: u32) {
        self.precedence = Some(value);
    }

    /// Gets the value of Precedence
    pub fn get_precedence(&self) -> Option<&u32> {
        self.precedence.as_ref()
    }

    /// Sets the value of PriorityValue8021Action
    pub fn set_priority_value8021_action(&mut self, value: u8) {
        self.priority_value8021_action = Some(value);
    }

    /// Gets the value of PriorityValue8021Action
    pub fn get_priority_value8021_action(&self) -> Option<&u8> {
        self.priority_value8021_action.as_ref()
    }

    /// Sets the value of TemplateMatchCondition
    pub fn set_template_match_condition(&mut self, value: u32) {
        self.template_match_condition = Some(value);
    }

    /// Gets the value of TemplateMatchCondition
    pub fn get_template_match_condition(&self) -> Option<&u32> {
        self.template_match_condition.as_ref()
    }

    /// Sets the value of ThrottleRateAction
    pub fn set_throttle_rate_action(&mut self, value: u64) {
        self.throttle_rate_action = Some(value);
    }

    /// Gets the value of ThrottleRateAction
    pub fn get_throttle_rate_action(&self) -> Option<&u64> {
        self.throttle_rate_action.as_ref()
    }

    /// Sets the value of URIMatchCondition
    pub fn set_urimatch_condition(&mut self, value: String) {
        self.urimatch_condition = Some(value);
    }

    /// Gets the value of URIMatchCondition
    pub fn get_urimatch_condition(&self) -> Option<&String> {
        self.urimatch_condition.as_ref()
    }

    /// Sets the value of URIRecursiveMatchCondition
    pub fn set_urirecursive_match_condition(&mut self, value: bool) {
        self.urirecursive_match_condition = Some(value);
    }

    /// Gets the value of URIRecursiveMatchCondition
    pub fn get_urirecursive_match_condition(&self) -> Option<&bool> {
        self.urirecursive_match_condition.as_ref()
    }

    /// Sets the value of UserMatchCondition
    pub fn set_user_match_condition(&mut self, value: String) {
        self.user_match_condition = Some(value);
    }

    /// Gets the value of UserMatchCondition
    pub fn get_user_match_condition(&self) -> Option<&String> {
        self.user_match_condition.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

