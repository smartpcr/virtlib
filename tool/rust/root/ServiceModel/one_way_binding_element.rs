// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OneWayBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OneWayBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The channel pool settings.
    #[serde(rename = "ChannelPoolSettings")]
    pub channel_pool_settings: Option<ChannelPoolSettings>,

/// The maximum number of accepted channels.
    #[serde(rename = "MaxAcceptedChannels")]
    pub max_accepted_channels: Option<i32>,

/// Whether the packet is routable.
    #[serde(rename = "PacketRoutable")]
    pub packet_routable: Option<bool>,
}

impl OneWayBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            channel_pool_settings: None,
            max_accepted_channels: None,
            packet_routable: None,
        }
    }


    /// Sets the value of ChannelPoolSettings
    pub fn set_channel_pool_settings(&mut self, value: ChannelPoolSettings) {
        self.channel_pool_settings = Some(value);
    }

    /// Gets the value of ChannelPoolSettings
    pub fn get_channel_pool_settings(&self) -> Option<&ChannelPoolSettings> {
        self.channel_pool_settings.as_ref()
    }

    /// Sets the value of MaxAcceptedChannels
    pub fn set_max_accepted_channels(&mut self, value: i32) {
        self.max_accepted_channels = Some(value);
    }

    /// Gets the value of MaxAcceptedChannels
    pub fn get_max_accepted_channels(&self) -> Option<&i32> {
        self.max_accepted_channels.as_ref()
    }

    /// Sets the value of PacketRoutable
    pub fn set_packet_routable(&mut self, value: bool) {
        self.packet_routable = Some(value);
    }

    /// Gets the value of PacketRoutable
    pub fn get_packet_routable(&self) -> Option<&bool> {
        self.packet_routable.as_ref()
    }
}

