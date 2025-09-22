//! # OXC WebRTC and Real-time Communication Rules
//!
//! This module implements WASM-safe OXC rules for WebRTC peer-to-peer communication,
//! real-time data synchronization, and media streaming patterns.
//!
//! ## Rule Categories:
//! - **Peer-to-Peer Connection Management**: Proper connection lifecycle and ICE handling
//! - **Media Stream Optimization**: Video/audio streaming best practices
//! - **Data Channel Security**: Secure data transmission patterns
//! - **Real-time Performance**: Low-latency communication optimization
//! - **Connection Resilience**: Error handling and reconnection strategies
//! - **WebSocket Integration**: Real-time bidirectional communication
//! - **Audio/Video Processing**: Media processing and optimization
//! - **Signaling Server Patterns**: WebRTC signaling best practices
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub rule_name: String,
    pub suggestion: String,
    pub confidence: f32,
    pub auto_fixable: bool,
}

// ================================================================================================
// WebRTC Peer Connection Management Rules
// ================================================================================================

/// Enforces proper cleanup of WebRTC peer connections to prevent memory leaks
pub struct RequirePeerConnectionCleanup;

impl RequirePeerConnectionCleanup {
    pub const NAME: &'static str = "require-peer-connection-cleanup";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequirePeerConnectionCleanup {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("new RTCPeerConnection") && !code.contains("close()") {
            diagnostics.push(create_peer_connection_cleanup_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePeerConnectionCleanup {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add cleanup logic in componentWillUnmount or useEffect cleanup function with pc.close() call".to_string(),
            confidence: 0.95,
            auto_fixable: true,
        }).collect()
    }
}

/// Ensures proper ICE candidate gathering timeout configuration
pub struct RequireIceCandidateTimeout;

impl RequireIceCandidateTimeout {
    pub const NAME: &'static str = "require-ice-candidate-timeout";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireIceCandidateTimeout {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("iceGatheringState") && !code.contains("iceCandidatePoolSize") {
            diagnostics.push(create_ice_candidate_timeout_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireIceCandidateTimeout {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Configure ICE candidate pool size and gathering timeout for optimal connection establishment".to_string(),
            confidence: 0.88,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Media Stream and Processing Rules
// ================================================================================================

/// Prevents memory leaks from unreleased media streams
pub struct RequireMediaStreamCleanup;

impl RequireMediaStreamCleanup {
    pub const NAME: &'static str = "require-media-stream-cleanup";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireMediaStreamCleanup {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("getUserMedia") && !code.contains("getTracks().forEach") {
            diagnostics.push(create_media_stream_cleanup_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMediaStreamCleanup {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add stream.getTracks().forEach(track => track.stop()) to properly release media devices".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces video quality adaptation based on network conditions
pub struct RequireAdaptiveVideoQuality;

impl RequireAdaptiveVideoQuality {
    pub const NAME: &'static str = "require-adaptive-video-quality";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAdaptiveVideoQuality {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("addTrack") && code.contains("video") && !code.contains("getStats") {
            diagnostics.push(create_adaptive_video_quality_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAdaptiveVideoQuality {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement adaptive bitrate using getStats() to monitor connection quality and adjust video parameters".to_string(),
            confidence: 0.85,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Data Channel Security and Performance Rules
// ================================================================================================

/// Ensures data channels use proper buffering to prevent overflow
pub struct RequireDataChannelBuffering;

impl RequireDataChannelBuffering {
    pub const NAME: &'static str = "require-data-channel-buffering";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDataChannelBuffering {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("createDataChannel") && !code.contains("bufferedAmount") {
            diagnostics.push(create_data_channel_buffering_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDataChannelBuffering {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Monitor bufferedAmount before sending data to prevent channel overflow and message loss".to_string(),
            confidence: 0.90,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents sending sensitive data over unordered data channels
pub struct NoSensitiveDataUnordered;

impl NoSensitiveDataUnordered {
    pub const NAME: &'static str = "no-sensitive-data-unordered";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoSensitiveDataUnordered {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("ordered: false") &&
           (code.contains("password") || code.contains("token") || code.contains("secret")) {
            diagnostics.push(create_sensitive_data_unordered_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoSensitiveDataUnordered {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use ordered data channels for sensitive data transmission to ensure message integrity and delivery order".to_string(),
            confidence: 0.96,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// WebSocket and Real-time Communication Rules
// ================================================================================================

/// Enforces WebSocket connection heartbeat mechanism
pub struct RequireWebsocketHeartbeat;

impl RequireWebsocketHeartbeat {
    pub const NAME: &'static str = "require-websocket-heartbeat";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebsocketHeartbeat {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("new WebSocket") && !code.contains("ping") && !code.contains("pong") {
            diagnostics.push(create_websocket_heartbeat_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebsocketHeartbeat {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement ping/pong heartbeat mechanism to detect connection drops and enable automatic reconnection".to_string(),
            confidence: 0.87,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires exponential backoff for WebSocket reconnection attempts
pub struct RequireExponentialBackoff;

impl RequireExponentialBackoff {
    pub const NAME: &'static str = "require-exponential-backoff";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireExponentialBackoff {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("reconnect") && code.contains("setTimeout") && !code.contains("Math.pow") {
            diagnostics.push(create_exponential_backoff_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireExponentialBackoff {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use exponential backoff with jitter for reconnection attempts to prevent server overload".to_string(),
            confidence: 0.89,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_peer_connection_cleanup_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePeerConnectionCleanup::NAME.to_string(),
        message: "WebRTC peer connection must be properly closed to prevent memory leaks".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add pc.close() in cleanup function".to_string()),
    }
}

fn create_ice_candidate_timeout_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireIceCandidateTimeout::NAME.to_string(),
        message: "Configure ICE candidate gathering timeout for optimal connection performance".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add iceCandidatePoolSize configuration".to_string()),
    }
}

fn create_media_stream_cleanup_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMediaStreamCleanup::NAME.to_string(),
        message: "Media stream tracks must be stopped to release camera/microphone resources".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add stream.getTracks().forEach(track => track.stop())".to_string()),
    }
}

fn create_adaptive_video_quality_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAdaptiveVideoQuality::NAME.to_string(),
        message: "Implement adaptive video quality based on network conditions".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Monitor connection stats and adjust video bitrate".to_string()),
    }
}

fn create_data_channel_buffering_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataChannelBuffering::NAME.to_string(),
        message: "Monitor data channel buffered amount to prevent overflow".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Check bufferedAmount before sending data".to_string()),
    }
}

fn create_sensitive_data_unordered_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoSensitiveDataUnordered::NAME.to_string(),
        message: "Sensitive data should not be sent over unordered data channels".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use ordered: true for sensitive data transmission".to_string()),
    }
}

fn create_websocket_heartbeat_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebsocketHeartbeat::NAME.to_string(),
        message: "WebSocket connections should implement heartbeat mechanism".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add ping/pong heartbeat implementation".to_string()),
    }
}

fn create_exponential_backoff_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireExponentialBackoff::NAME.to_string(),
        message: "Use exponential backoff for reconnection attempts".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement exponential backoff with jitter".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_connection_cleanup_detection() {
        let code = "const pc = new RTCPeerConnection(config);";
        let rule = RequirePeerConnectionCleanup;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequirePeerConnectionCleanup::NAME);
    }

    #[test]
    fn test_media_stream_cleanup_detection() {
        let code = "navigator.mediaDevices.getUserMedia({ video: true })";
        let rule = RequireMediaStreamCleanup;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireMediaStreamCleanup::NAME);
    }

    #[test]
    fn test_data_channel_buffering_detection() {
        let code = "const channel = pc.createDataChannel('data');";
        let rule = RequireDataChannelBuffering;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireDataChannelBuffering::NAME);
    }

    #[test]
    fn test_sensitive_data_unordered_detection() {
        let code = r#"const channel = pc.createDataChannel('auth', { ordered: false });
                      channel.send(JSON.stringify({ password: 'secret' }));"#;
        let rule = NoSensitiveDataUnordered;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoSensitiveDataUnordered::NAME);
    }

    #[test]
    fn test_websocket_heartbeat_detection() {
        let code = "const ws = new WebSocket('wss://example.com');";
        let rule = RequireWebsocketHeartbeat;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireWebsocketHeartbeat::NAME);
    }

    #[test]
    fn test_exponential_backoff_detection() {
        let code = r#"function reconnect() {
                        setTimeout(() => connect(), 1000);
                      }"#;
        let rule = RequireExponentialBackoff;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireExponentialBackoff::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = "const pc = new RTCPeerConnection();";
        let rule = RequirePeerConnectionCleanup;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].auto_fixable);
    }
}