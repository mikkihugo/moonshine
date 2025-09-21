//! # AR/VR Development Rules
//!
//! Comprehensive rules for Augmented Reality, Virtual Reality, WebXR,
//! immersive experiences, and spatial computing applications.
//!
//! ## Rule Categories:
//! - **WebXR Standards**: Session management, reference spaces, input handling
//! - **Immersive UX**: Comfort guidelines, accessibility, motion sickness prevention
//! - **Spatial Computing**: 3D interactions, hand tracking, eye tracking
//! - **Performance Optimization**: Frame rate maintenance, LOD management, occlusion culling
//! - **Cross-Platform**: Device compatibility, fallback mechanisms, progressive enhancement
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Proper WebXR session lifecycle
//! async function startXRSession() {
//!   const session = await navigator.xr.requestSession('immersive-vr');
//!   session.addEventListener('end', cleanup);
//!   return session;
//! }
//!
//! // ❌ Bad: Missing comfort settings
//! function enableLocomotion() {
//!   // No comfort options, could cause motion sickness
//!   player.position.set(x, y, z);
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};

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
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}

/// Rule: require-xr-session-lifecycle
/// Enforces proper WebXR session creation, management, and cleanup
#[derive(Clone)]
pub struct RequireXRSessionLifecycle;

impl RequireXRSessionLifecycle {
    pub const NAME: &'static str = "require-xr-session-lifecycle";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireXRSessionLifecycle {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for XR session creation without proper cleanup
        if code.contains("requestSession") && !code.contains("session.end") {
            diagnostics.push(create_xr_session_lifecycle_diagnostic(
                0, 0,
                "WebXR sessions must be properly ended to release resources"
            ));
        }

        // Check for missing session event handlers
        if code.contains("requestSession") && !code.contains("addEventListener('end'") {
            diagnostics.push(create_xr_session_lifecycle_diagnostic(
                0, 0,
                "WebXR sessions should handle 'end' events for proper cleanup"
            ));
        }

        // Check for session usage without feature detection
        if code.contains("navigator.xr") && !code.contains("isSessionSupported") {
            diagnostics.push(create_xr_session_lifecycle_diagnostic(
                0, 0,
                "WebXR usage should include session support detection"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireXRSessionLifecycle {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.94,
            suggestion: "Implement proper XR session lifecycle: `session.addEventListener('end', () => cleanup()); window.addEventListener('beforeunload', () => session.end());`".to_string(),
            fix_code: Some("class XRSessionManager {\n  async createSession(mode) {\n    if (!navigator.xr) throw new Error('WebXR not supported');\n    \n    const supported = await navigator.xr.isSessionSupported(mode);\n    if (!supported) throw new Error(`${mode} not supported`);\n    \n    const session = await navigator.xr.requestSession(mode);\n    \n    session.addEventListener('end', () => {\n      this.cleanup();\n    });\n    \n    window.addEventListener('beforeunload', () => {\n      session.end();\n    });\n    \n    return session;\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-comfort-settings
/// Enforces comfort and accessibility settings to prevent motion sickness
#[derive(Clone)]
pub struct RequireComfortSettings;

impl RequireComfortSettings {
    pub const NAME: &'static str = "require-comfort-settings";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireComfortSettings {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for locomotion without comfort options
        if code.contains("locomotion") && !code.contains("comfort") && !code.contains("teleport") {
            diagnostics.push(create_comfort_settings_diagnostic(
                0, 0,
                "Locomotion systems should provide comfort options like teleportation"
            ));
        }

        // Check for camera movement without snap turning
        if code.contains("camera.rotation") && !code.contains("snap") && !code.contains("comfort") {
            diagnostics.push(create_comfort_settings_diagnostic(
                0, 0,
                "Camera rotation should offer snap turning for comfort"
            ));
        }

        // Check for missing FOV reduction during motion
        if code.contains("velocity") && !code.contains("vignette") && !code.contains("fov") {
            diagnostics.push(create_comfort_settings_diagnostic(
                0, 0,
                "Fast movement should include FOV reduction or vignetting for comfort"
            ));
        }

        // Check for acceleration without comfort mitigation
        if code.contains("acceleration") && !code.contains("comfort") && !code.contains("fade") {
            diagnostics.push(create_comfort_settings_diagnostic(
                0, 0,
                "Acceleration should include comfort mitigation techniques"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireComfortSettings {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Add comfort settings: `const comfortSettings = { snapTurn: true, teleportation: true, vignetteOnMotion: true, reducedMotion: false };`".to_string(),
            fix_code: Some("class ComfortManager {\n  constructor() {\n    this.settings = {\n      snapTurnAngle: 30,\n      teleportationEnabled: true,\n      vignetteOnMotion: true,\n      motionSicknessReduction: 0.8,\n      comfortSpeed: 3.0\n    };\n  }\n  \n  applyLocomotion(direction, speed) {\n    if (this.settings.teleportationEnabled && speed > this.settings.comfortSpeed) {\n      this.teleport(direction);\n    } else {\n      this.smoothMove(direction, Math.min(speed, this.settings.comfortSpeed));\n    }\n  }\n  \n  applyRotation(angle) {\n    if (this.settings.snapTurnAngle > 0) {\n      const snapAngle = Math.round(angle / this.settings.snapTurnAngle) * this.settings.snapTurnAngle;\n      this.snapTurn(snapAngle);\n    } else {\n      this.smoothTurn(angle);\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-frame-rate-maintenance
/// Enforces consistent frame rate maintenance for VR comfort
#[derive(Clone)]
pub struct RequireFrameRateMaintenance;

impl RequireFrameRateMaintenance {
    pub const NAME: &'static str = "require-frame-rate-maintenance";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireFrameRateMaintenance {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for VR rendering without frame rate monitoring
        if code.contains("requestAnimationFrame") && code.contains("vr") && !code.contains("framerate") {
            diagnostics.push(create_frame_rate_maintenance_diagnostic(
                0, 0,
                "VR applications must monitor frame rate to maintain 90fps for comfort"
            ));
        }

        // Check for expensive operations without LOD
        if code.contains("drawElements") && !code.contains("lod") && !code.contains("level") {
            diagnostics.push(create_frame_rate_maintenance_diagnostic(
                0, 0,
                "Complex rendering should implement Level of Detail (LOD) for performance"
            ));
        }

        // Check for missing occlusion culling
        if code.contains("mesh") && code.contains("visible") && !code.contains("frustum") {
            diagnostics.push(create_frame_rate_maintenance_diagnostic(
                0, 0,
                "VR rendering should implement frustum and occlusion culling"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireFrameRateMaintenance {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Implement frame rate maintenance: `const targetFPS = 90; if (currentFPS < targetFPS * 0.9) { reduceLOD(); } else if (currentFPS > targetFPS * 1.1) { increaseLOD(); }`".to_string(),
            fix_code: Some("class VRPerformanceManager {\n  constructor() {\n    this.targetFPS = 90;\n    this.frameTimeHistory = [];\n    this.lodLevel = 1.0;\n  }\n  \n  updatePerformance() {\n    const currentFPS = this.calculateFPS();\n    \n    if (currentFPS < this.targetFPS * 0.9) {\n      this.reduceLOD();\n    } else if (currentFPS > this.targetFPS * 1.1 && this.lodLevel < 1.0) {\n      this.increaseLOD();\n    }\n  }\n  \n  reduceLOD() {\n    this.lodLevel = Math.max(0.1, this.lodLevel - 0.1);\n    this.applyLODToScene();\n  }\n  \n  increaseLOD() {\n    this.lodLevel = Math.min(1.0, this.lodLevel + 0.1);\n    this.applyLODToScene();\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-spatial-audio
/// Enforces proper spatial audio implementation for immersive experiences
#[derive(Clone)]
pub struct RequireSpatialAudio;

impl RequireSpatialAudio {
    pub const NAME: &'static str = "require-spatial-audio";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSpatialAudio {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for audio without spatial positioning
        if code.contains("Audio") && code.contains("3D") && !code.contains("position") {
            diagnostics.push(create_spatial_audio_diagnostic(
                0, 0,
                "3D audio sources should have spatial positioning for immersion"
            ));
        }

        // Check for missing HRTF or binaural audio
        if code.contains("AudioContext") && code.contains("vr") && !code.contains("HRTF") {
            diagnostics.push(create_spatial_audio_diagnostic(
                0, 0,
                "VR applications should use HRTF for realistic spatial audio"
            ));
        }

        // Check for audio without distance attenuation
        if code.contains("3D") && code.contains("sound") && !code.contains("distance") {
            diagnostics.push(create_spatial_audio_diagnostic(
                0, 0,
                "Spatial audio should implement distance-based attenuation"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSpatialAudio {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Implement spatial audio: `const panner = audioContext.createPanner(); panner.positionX.value = x; panner.positionY.value = y; panner.positionZ.value = z;`".to_string(),
            fix_code: Some("class SpatialAudioManager {\n  constructor(audioContext) {\n    this.audioContext = audioContext;\n    this.listener = audioContext.listener;\n    this.sources = new Map();\n  }\n  \n  createSpatialSource(id, audioBuffer, position) {\n    const source = this.audioContext.createBufferSource();\n    const panner = this.audioContext.createPanner();\n    \n    // Configure 3D audio\n    panner.panningModel = 'HRTF';\n    panner.distanceModel = 'inverse';\n    panner.refDistance = 1;\n    panner.maxDistance = 100;\n    panner.rolloffFactor = 1;\n    \n    // Set position\n    panner.positionX.value = position.x;\n    panner.positionY.value = position.y;\n    panner.positionZ.value = position.z;\n    \n    source.buffer = audioBuffer;\n    source.connect(panner).connect(this.audioContext.destination);\n    \n    this.sources.set(id, { source, panner });\n    return { source, panner };\n  }\n  \n  updateListenerPosition(position, orientation) {\n    this.listener.positionX.value = position.x;\n    this.listener.positionY.value = position.y;\n    this.listener.positionZ.value = position.z;\n    \n    this.listener.forwardX.value = orientation.forward.x;\n    this.listener.forwardY.value = orientation.forward.y;\n    this.listener.forwardZ.value = orientation.forward.z;\n    \n    this.listener.upX.value = orientation.up.x;\n    this.listener.upY.value = orientation.up.y;\n    this.listener.upZ.value = orientation.up.z;\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-hand-tracking-validation
/// Enforces proper hand tracking input validation and error handling
#[derive(Clone)]
pub struct RequireHandTrackingValidation;

impl RequireHandTrackingValidation {
    pub const NAME: &'static str = "require-hand-tracking-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireHandTrackingValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for hand tracking without confidence validation
        if code.contains("hand") && code.contains("joints") && !code.contains("confidence") {
            diagnostics.push(create_hand_tracking_validation_diagnostic(
                0, 0,
                "Hand tracking data should validate joint confidence levels"
            ));
        }

        // Check for gesture recognition without temporal validation
        if code.contains("gesture") && !code.contains("temporal") && !code.contains("history") {
            diagnostics.push(create_hand_tracking_validation_diagnostic(
                0, 0,
                "Gesture recognition should use temporal validation to reduce false positives"
            ));
        }

        // Check for hand input without fallback to controllers
        if code.contains("handTracking") && !code.contains("controller") && !code.contains("fallback") {
            diagnostics.push(create_hand_tracking_validation_diagnostic(
                0, 0,
                "Hand tracking should provide fallback to controller input"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireHandTrackingValidation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Add hand tracking validation: `if (joint.confidence > 0.8 && joint.position.isValid()) { processGesture(joint); } else { fallbackToController(); }`".to_string(),
            fix_code: Some("class HandTrackingValidator {\n  constructor() {\n    this.confidenceThreshold = 0.8;\n    this.gestureHistory = [];\n    this.temporalWindow = 5;\n  }\n  \n  validateJoint(joint) {\n    return joint.confidence > this.confidenceThreshold &&\n           joint.position &&\n           !isNaN(joint.position.x) &&\n           !isNaN(joint.position.y) &&\n           !isNaN(joint.position.z);\n  }\n  \n  validateGesture(gesture) {\n    this.gestureHistory.push(gesture);\n    \n    if (this.gestureHistory.length > this.temporalWindow) {\n      this.gestureHistory.shift();\n    }\n    \n    // Require consistent gesture over time\n    const consistentGestures = this.gestureHistory.filter(g => g.type === gesture.type);\n    return consistentGestures.length >= Math.ceil(this.temporalWindow / 2);\n  }\n  \n  processHandInput(handData) {\n    if (!handData || !this.validateHandTracking(handData)) {\n      return this.fallbackToControllers();\n    }\n    \n    return this.processValidatedHands(handData);\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-accessibility-features
/// Enforces accessibility features for users with disabilities in VR/AR
#[derive(Clone)]
pub struct RequireAccessibilityFeatures;

impl RequireAccessibilityFeatures {
    pub const NAME: &'static str = "require-accessibility-features";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAccessibilityFeatures {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for interactions without voice alternatives
        if code.contains("interaction") && !code.contains("voice") && !code.contains("audio") {
            diagnostics.push(create_accessibility_features_diagnostic(
                0, 0,
                "Interactive elements should provide voice or audio alternatives"
            ));
        }

        // Check for visual content without audio descriptions
        if code.contains("visual") && code.contains("important") && !code.contains("describe") {
            diagnostics.push(create_accessibility_features_diagnostic(
                0, 0,
                "Important visual content should have audio descriptions"
            ));
        }

        // Check for missing text-to-speech for UI elements
        if code.contains("menu") && !code.contains("tts") && !code.contains("speak") {
            diagnostics.push(create_accessibility_features_diagnostic(
                0, 0,
                "Menu items should support text-to-speech for visually impaired users"
            ));
        }

        // Check for controls without alternative input methods
        if code.contains("button") && !code.contains("alternative") && !code.contains("dwell") {
            diagnostics.push(create_accessibility_features_diagnostic(
                0, 0,
                "Controls should provide alternative input methods like dwell selection"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAccessibilityFeatures {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Add accessibility features: `class AccessibilityManager { enableVoiceCommands() { /* voice control */ } enableDwellSelection() { /* gaze-based selection */ } enableAudioDescriptions() { /* spatial audio descriptions */ } }`".to_string(),
            fix_code: Some("class VRAccessibilityManager {\n  constructor() {\n    this.speechSynthesis = window.speechSynthesis;\n    this.voiceRecognition = new (window.SpeechRecognition || window.webkitSpeechRecognition)();\n    this.dwellTime = 2000; // 2 seconds\n  }\n  \n  enableVoiceNavigation() {\n    this.voiceRecognition.onresult = (event) => {\n      const command = event.results[0][0].transcript.toLowerCase();\n      this.processVoiceCommand(command);\n    };\n    this.voiceRecognition.start();\n  }\n  \n  enableDwellSelection() {\n    this.gazeTarget = null;\n    this.gazeStartTime = null;\n    \n    // Monitor gaze direction\n    this.onGazeUpdate = (target) => {\n      if (target !== this.gazeTarget) {\n        this.gazeTarget = target;\n        this.gazeStartTime = Date.now();\n      } else if (this.gazeStartTime && Date.now() - this.gazeStartTime > this.dwellTime) {\n        this.selectTarget(target);\n        this.gazeTarget = null;\n      }\n    };\n  }\n  \n  announceElement(element) {\n    const utterance = new SpeechSynthesisUtterance(element.getAttribute('aria-label') || element.textContent);\n    this.speechSynthesis.speak(utterance);\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-cross-platform-compatibility
/// Enforces cross-platform VR/AR compatibility and feature detection
#[derive(Clone)]
pub struct RequireCrossPlatformCompatibility;

impl RequireCrossPlatformCompatibility {
    pub const NAME: &'static str = "require-cross-platform-compatibility";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCrossPlatformCompatibility {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for device-specific code without feature detection
        if code.contains("oculus") || code.contains("vive") || code.contains("quest") {
            if !code.contains("detect") && !code.contains("supports") {
                diagnostics.push(create_cross_platform_compatibility_diagnostic(
                    0, 0,
                    "Device-specific code should include feature detection for compatibility"
                ));
            }
        }

        // Check for missing progressive enhancement
        if code.contains("immersive-vr") && !code.contains("inline") && !code.contains("fallback") {
            diagnostics.push(create_cross_platform_compatibility_diagnostic(
                0, 0,
                "VR experiences should provide 2D fallback for unsupported devices"
            ));
        }

        // Check for hardcoded controller mappings
        if code.contains("gamepad") && code.contains("button[0]") && !code.contains("mapping") {
            diagnostics.push(create_cross_platform_compatibility_diagnostic(
                0, 0,
                "Controller input should use device-agnostic button mappings"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCrossPlatformCompatibility {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Implement cross-platform compatibility: `const deviceCapabilities = await detectDeviceCapabilities(); if (deviceCapabilities.supportsVR) { startVR(); } else { start2DMode(); }`".to_string(),
            fix_code: Some("class XRCompatibilityManager {\n  async initialize() {\n    this.capabilities = await this.detectCapabilities();\n    this.inputMappings = await this.loadInputMappings();\n    return this.selectBestMode();\n  }\n  \n  async detectCapabilities() {\n    const capabilities = {\n      supportsVR: false,\n      supportsAR: false,\n      supportsHandTracking: false,\n      supportedReferenceSpaces: []\n    };\n    \n    if (navigator.xr) {\n      try {\n        capabilities.supportsVR = await navigator.xr.isSessionSupported('immersive-vr');\n        capabilities.supportsAR = await navigator.xr.isSessionSupported('immersive-ar');\n      } catch (e) {\n        console.warn('XR capability detection failed:', e);\n      }\n    }\n    \n    return capabilities;\n  }\n  \n  selectBestMode() {\n    if (this.capabilities.supportsVR) {\n      return 'immersive-vr';\n    } else if (this.capabilities.supportsAR) {\n      return 'immersive-ar';\n    } else {\n      return 'inline';\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-eye-tracking-privacy
/// Enforces privacy protection for eye tracking data
#[derive(Clone)]
pub struct RequireEyeTrackingPrivacy;

impl RequireEyeTrackingPrivacy {
    pub const NAME: &'static str = "require-eye-tracking-privacy";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireEyeTrackingPrivacy {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for eye tracking without explicit consent
        if code.contains("eyeTracking") && !code.contains("consent") && !code.contains("permission") {
            diagnostics.push(create_eye_tracking_privacy_diagnostic(
                0, 0,
                "Eye tracking requires explicit user consent due to privacy implications"
            ));
        }

        // Check for raw gaze data storage without anonymization
        if code.contains("gazeData") && code.contains("store") && !code.contains("anonymize") {
            diagnostics.push(create_eye_tracking_privacy_diagnostic(
                0, 0,
                "Gaze data should be anonymized before storage to protect user privacy"
            ));
        }

        // Check for eye tracking data transmission without encryption
        if code.contains("eyeData") && code.contains("send") && !code.contains("encrypt") {
            diagnostics.push(create_eye_tracking_privacy_diagnostic(
                0, 0,
                "Eye tracking data transmission must be encrypted for privacy protection"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireEyeTrackingPrivacy {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.95,
            suggestion: "Implement eye tracking privacy: `async function requestEyeTrackingPermission() { const consent = await showConsentDialog('Eye tracking will be used for interaction. Data will not be stored.'); return consent; }`".to_string(),
            fix_code: Some("class EyeTrackingPrivacyManager {\n  async requestPermission() {\n    const consent = await this.showConsentDialog({\n      title: 'Eye Tracking Permission',\n      message: 'This application would like to use eye tracking for enhanced interaction. Your gaze data will be processed locally and not stored or transmitted.',\n      options: ['Allow', 'Deny']\n    });\n    \n    if (consent) {\n      this.logConsentGiven();\n      return true;\n    }\n    \n    return false;\n  }\n  \n  processGazeData(rawGazeData) {\n    // Remove personally identifiable patterns\n    const anonymizedData = this.anonymizeGazeData(rawGazeData);\n    \n    // Process only for immediate interaction\n    const interactionData = this.extractInteractionIntent(anonymizedData);\n    \n    // Clear raw data immediately\n    rawGazeData = null;\n    \n    return interactionData;\n  }\n  \n  anonymizeGazeData(data) {\n    // Remove timing precision that could identify individuals\n    // Aggregate data to remove unique patterns\n    return {\n      region: this.quantizeRegion(data.gazePoint),\n      duration: Math.round(data.duration / 100) * 100 // Round to 100ms\n    };\n  }\n}".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_xr_session_lifecycle_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireXRSessionLifecycle::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_comfort_settings_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireComfortSettings::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_frame_rate_maintenance_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireFrameRateMaintenance::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_spatial_audio_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSpatialAudio::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_hand_tracking_validation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireHandTrackingValidation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_accessibility_features_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAccessibilityFeatures::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_cross_platform_compatibility_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCrossPlatformCompatibility::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_eye_tracking_privacy_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireEyeTrackingPrivacy::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Security".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_xr_session_lifecycle_violation() {
        let code = r#"
        async function startVR() {
            const session = await navigator.xr.requestSession('immersive-vr');
            setupRenderer(session);
        }
        "#;

        let rule = RequireXRSessionLifecycle;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("ended"));
    }

    #[test]
    fn test_require_xr_session_lifecycle_compliant() {
        let code = r#"
        async function startVR() {
            const session = await navigator.xr.requestSession('immersive-vr');
            session.addEventListener('end', cleanup);
            setupRenderer(session);

            window.addEventListener('beforeunload', () => {
                session.end();
            });
        }
        "#;

        let rule = RequireXRSessionLifecycle;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_comfort_settings_violation() {
        let code = r#"
        function movePlayer(direction) {
            player.position.add(direction.multiplyScalar(speed));
            camera.rotation.y += turnSpeed;
        }
        "#;

        let rule = RequireComfortSettings;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("comfort"));
    }

    #[test]
    fn test_require_frame_rate_maintenance_violation() {
        let code = r#"
        function render() {
            scene.children.forEach(mesh => {
                mesh.render();
            });
            requestAnimationFrame(render);
        }
        "#;

        let rule = RequireFrameRateMaintenance;
        let diagnostics = rule.run(code);

        // This should not trigger since it doesn't contain "vr"
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_hand_tracking_validation_violation() {
        let code = r#"
        function processHandInput() {
            const joints = hand.getJoints();
            joints.forEach(joint => {
                processGesture(joint);
            });
        }
        "#;

        let rule = RequireHandTrackingValidation;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("confidence"));
    }

    #[test]
    fn test_ai_enhancement_xr_session_lifecycle() {
        let rule = RequireXRSessionLifecycle;
        let diagnostics = vec![create_xr_session_lifecycle_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("addEventListener"));
    }
}