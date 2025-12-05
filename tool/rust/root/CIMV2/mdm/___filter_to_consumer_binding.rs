// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __FilterToConsumerBinding struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __FilterToConsumerBinding {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "Consumer")]
    pub consumer: Option<__EventConsumer>,

/// 
    #[serde(rename = "CreatorSID")]
    pub creator_sid: Vec<u8>,

/// 
    #[serde(rename = "DeliverSynchronously")]
    pub deliver_synchronously: Option<bool>,

/// 
    #[serde(rename = "DeliveryQoS")]
    pub delivery_qo_s: Option<u32>,

/// 
    #[serde(rename = "Filter")]
    pub filter: Option<__EventFilter>,

/// 
    #[serde(rename = "MaintainSecurityContext")]
    pub maintain_security_context: Option<bool>,

/// 
    #[serde(rename = "SlowDownProviders")]
    pub slow_down_providers: Option<bool>,
}

impl __FilterToConsumerBinding {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            consumer: None,
            creator_sid: Vec::new(),
            deliver_synchronously: None,
            delivery_qo_s: None,
            filter: None,
            maintain_security_context: None,
            slow_down_providers: None,
        }
    }


    /// Sets the value of Consumer
    pub fn set_consumer(&mut self, value: __EventConsumer) {
        self.consumer = Some(value);
    }

    /// Gets the value of Consumer
    pub fn get_consumer(&self) -> Option<&__EventConsumer> {
        self.consumer.as_ref()
    }

    /// Sets the value of CreatorSID
    pub fn set_creator_sid(&mut self, value: Vec<u8>) {
        self.creator_sid = value;
    }

    /// Gets the value of CreatorSID
    pub fn get_creator_sid(&self) -> &Vec<u8> {
        &self.creator_sid
    }

    /// Sets the value of DeliverSynchronously
    pub fn set_deliver_synchronously(&mut self, value: bool) {
        self.deliver_synchronously = Some(value);
    }

    /// Gets the value of DeliverSynchronously
    pub fn get_deliver_synchronously(&self) -> Option<&bool> {
        self.deliver_synchronously.as_ref()
    }

    /// Sets the value of DeliveryQoS
    pub fn set_delivery_qo_s(&mut self, value: u32) {
        self.delivery_qo_s = Some(value);
    }

    /// Gets the value of DeliveryQoS
    pub fn get_delivery_qo_s(&self) -> Option<&u32> {
        self.delivery_qo_s.as_ref()
    }

    /// Sets the value of Filter
    pub fn set_filter(&mut self, value: __EventFilter) {
        self.filter = Some(value);
    }

    /// Gets the value of Filter
    pub fn get_filter(&self) -> Option<&__EventFilter> {
        self.filter.as_ref()
    }

    /// Sets the value of MaintainSecurityContext
    pub fn set_maintain_security_context(&mut self, value: bool) {
        self.maintain_security_context = Some(value);
    }

    /// Gets the value of MaintainSecurityContext
    pub fn get_maintain_security_context(&self) -> Option<&bool> {
        self.maintain_security_context.as_ref()
    }

    /// Sets the value of SlowDownProviders
    pub fn set_slow_down_providers(&mut self, value: bool) {
        self.slow_down_providers = Some(value);
    }

    /// Gets the value of SlowDownProviders
    pub fn get_slow_down_providers(&self) -> Option<&bool> {
        self.slow_down_providers.as_ref()
    }
}

