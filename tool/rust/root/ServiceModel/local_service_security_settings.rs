// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LocalServiceSecuritySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalServiceSecuritySettings {

/// A Boolean value that specifies whether replay attacks against the channel are detected and dealt with automatically.
    #[serde(rename = "DetectReplays")]
    pub detect_replays: Option<bool>,

/// A value that specifies the maximum number of pending security sessions that the service supports.
    #[serde(rename = "InactivityTimeout")]
    pub inactivity_timeout: Option<String>,

/// A TimeSpan that specifies the lifetime issued to all new security cookies.
    #[serde(rename = "IssuedCookieLifetime")]
    pub issued_cookie_lifetime: Option<String>,

/// A value that specifies the maximum number of cookies that can be cached.
    #[serde(rename = "MaxCachedCookies")]
    pub max_cached_cookies: Option<i32>,

/// A TimeSpan that specifies the maximum time difference between the system clocks of the two communicating parties.
    #[serde(rename = "MaxClockSkew")]
    pub max_clock_skew: Option<String>,

/// The maximum number of pending connections on the service.
    #[serde(rename = "MaxPendingSessions")]
    pub max_pending_sessions: Option<i32>,

/// A value that specifies the number of security negotiations that can be active concurrently.
    #[serde(rename = "MaxStatefulNegotiations")]
    pub max_stateful_negotiations: Option<i32>,

/// A TimeSpan that specifies the maximum duration for the security negotiation phase between server and client.
    #[serde(rename = "NegotiationTimeout")]
    pub negotiation_timeout: Option<String>,

/// A Boolean value that specifies whether connections using WS-Reliable messaging will attempt to reconnect after transport failures.
    #[serde(rename = "ReconnectTransportOnFailure")]
    pub reconnect_transport_on_failure: Option<bool>,

/// A value that specifies the number of cached nonces used for replay detection.
    #[serde(rename = "ReplayCacheSize")]
    pub replay_cache_size: Option<i32>,

/// A TimeSpan that specifies the duration in which individual message nonces are valid.
    #[serde(rename = "ReplayWindow")]
    pub replay_window: Option<String>,

/// A TimeSpan that specifies the duration after which the initiator will renew the key for the security session.
    #[serde(rename = "SessionKeyRenewalInterval")]
    pub session_key_renewal_interval: Option<String>,

/// A TimeSpan that specifies the time interval a previous session key is valid on incoming messages during a key renewal.
    #[serde(rename = "SessionKeyRolloverInterval")]
    pub session_key_rollover_interval: Option<String>,

/// A TimeSpan that specifies the duration in which a time stamp is valid.
    #[serde(rename = "TimestampValidityDuration")]
    pub timestamp_validity_duration: Option<String>,
}

impl LocalServiceSecuritySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            detect_replays: None,
            inactivity_timeout: None,
            issued_cookie_lifetime: None,
            max_cached_cookies: None,
            max_clock_skew: None,
            max_pending_sessions: None,
            max_stateful_negotiations: None,
            negotiation_timeout: None,
            reconnect_transport_on_failure: None,
            replay_cache_size: None,
            replay_window: None,
            session_key_renewal_interval: None,
            session_key_rollover_interval: None,
            timestamp_validity_duration: None,
        }
    }


    /// Sets the value of DetectReplays
    pub fn set_detect_replays(&mut self, value: bool) {
        self.detect_replays = Some(value);
    }

    /// Gets the value of DetectReplays
    pub fn get_detect_replays(&self) -> Option<&bool> {
        self.detect_replays.as_ref()
    }

    /// Sets the value of InactivityTimeout
    pub fn set_inactivity_timeout(&mut self, value: String) {
        self.inactivity_timeout = Some(value);
    }

    /// Gets the value of InactivityTimeout
    pub fn get_inactivity_timeout(&self) -> Option<&String> {
        self.inactivity_timeout.as_ref()
    }

    /// Sets the value of IssuedCookieLifetime
    pub fn set_issued_cookie_lifetime(&mut self, value: String) {
        self.issued_cookie_lifetime = Some(value);
    }

    /// Gets the value of IssuedCookieLifetime
    pub fn get_issued_cookie_lifetime(&self) -> Option<&String> {
        self.issued_cookie_lifetime.as_ref()
    }

    /// Sets the value of MaxCachedCookies
    pub fn set_max_cached_cookies(&mut self, value: i32) {
        self.max_cached_cookies = Some(value);
    }

    /// Gets the value of MaxCachedCookies
    pub fn get_max_cached_cookies(&self) -> Option<&i32> {
        self.max_cached_cookies.as_ref()
    }

    /// Sets the value of MaxClockSkew
    pub fn set_max_clock_skew(&mut self, value: String) {
        self.max_clock_skew = Some(value);
    }

    /// Gets the value of MaxClockSkew
    pub fn get_max_clock_skew(&self) -> Option<&String> {
        self.max_clock_skew.as_ref()
    }

    /// Sets the value of MaxPendingSessions
    pub fn set_max_pending_sessions(&mut self, value: i32) {
        self.max_pending_sessions = Some(value);
    }

    /// Gets the value of MaxPendingSessions
    pub fn get_max_pending_sessions(&self) -> Option<&i32> {
        self.max_pending_sessions.as_ref()
    }

    /// Sets the value of MaxStatefulNegotiations
    pub fn set_max_stateful_negotiations(&mut self, value: i32) {
        self.max_stateful_negotiations = Some(value);
    }

    /// Gets the value of MaxStatefulNegotiations
    pub fn get_max_stateful_negotiations(&self) -> Option<&i32> {
        self.max_stateful_negotiations.as_ref()
    }

    /// Sets the value of NegotiationTimeout
    pub fn set_negotiation_timeout(&mut self, value: String) {
        self.negotiation_timeout = Some(value);
    }

    /// Gets the value of NegotiationTimeout
    pub fn get_negotiation_timeout(&self) -> Option<&String> {
        self.negotiation_timeout.as_ref()
    }

    /// Sets the value of ReconnectTransportOnFailure
    pub fn set_reconnect_transport_on_failure(&mut self, value: bool) {
        self.reconnect_transport_on_failure = Some(value);
    }

    /// Gets the value of ReconnectTransportOnFailure
    pub fn get_reconnect_transport_on_failure(&self) -> Option<&bool> {
        self.reconnect_transport_on_failure.as_ref()
    }

    /// Sets the value of ReplayCacheSize
    pub fn set_replay_cache_size(&mut self, value: i32) {
        self.replay_cache_size = Some(value);
    }

    /// Gets the value of ReplayCacheSize
    pub fn get_replay_cache_size(&self) -> Option<&i32> {
        self.replay_cache_size.as_ref()
    }

    /// Sets the value of ReplayWindow
    pub fn set_replay_window(&mut self, value: String) {
        self.replay_window = Some(value);
    }

    /// Gets the value of ReplayWindow
    pub fn get_replay_window(&self) -> Option<&String> {
        self.replay_window.as_ref()
    }

    /// Sets the value of SessionKeyRenewalInterval
    pub fn set_session_key_renewal_interval(&mut self, value: String) {
        self.session_key_renewal_interval = Some(value);
    }

    /// Gets the value of SessionKeyRenewalInterval
    pub fn get_session_key_renewal_interval(&self) -> Option<&String> {
        self.session_key_renewal_interval.as_ref()
    }

    /// Sets the value of SessionKeyRolloverInterval
    pub fn set_session_key_rollover_interval(&mut self, value: String) {
        self.session_key_rollover_interval = Some(value);
    }

    /// Gets the value of SessionKeyRolloverInterval
    pub fn get_session_key_rollover_interval(&self) -> Option<&String> {
        self.session_key_rollover_interval.as_ref()
    }

    /// Sets the value of TimestampValidityDuration
    pub fn set_timestamp_validity_duration(&mut self, value: String) {
        self.timestamp_validity_duration = Some(value);
    }

    /// Gets the value of TimestampValidityDuration
    pub fn get_timestamp_validity_duration(&self) -> Option<&String> {
        self.timestamp_validity_duration.as_ref()
    }
}

