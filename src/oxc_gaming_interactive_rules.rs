//! # Gaming & Interactive Media Rules
//!
//! Comprehensive rules for gaming, interactive media, WebGL, WebXR, and real-time
//! graphics applications in modern JavaScript/TypeScript development.
//!
//! ## Rule Categories:
//! - **WebGL Optimization**: Shader management, buffer optimization, rendering performance
//! - **WebXR Development**: VR/AR patterns, immersive experiences, spatial computing
//! - **Game Engine Patterns**: Entity-component systems, game loops, asset management
//! - **Real-time Graphics**: Animation optimization, frame rate management, GPU utilization
//! - **Interactive Media**: Canvas optimization, audio processing, input handling
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Efficient WebGL resource management
//! const texture = gl.createTexture();
//! gl.bindTexture(gl.TEXTURE_2D, texture);
//! // ... use texture
//! gl.deleteTexture(texture); // Clean up
//!
//! // ❌ Bad: Memory leak in WebGL
//! function createTexture() {
//!   return gl.createTexture(); // Never cleaned up
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

/// Rule: require-webgl-resource-cleanup
/// Enforces proper cleanup of WebGL resources to prevent memory leaks
#[derive(Clone)]
pub struct RequireWebGLResourceCleanup;

impl RequireWebGLResourceCleanup {
    pub const NAME: &'static str = "require-webgl-resource-cleanup";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebGLResourceCleanup {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for WebGL resource creation without cleanup
        if code.contains("createTexture()") && !code.contains("deleteTexture") {
            diagnostics.push(create_webgl_resource_cleanup_diagnostic(
                0, 0,
                "WebGL textures should be properly deleted to prevent memory leaks"
            ));
        }

        if code.contains("createBuffer()") && !code.contains("deleteBuffer") {
            diagnostics.push(create_webgl_resource_cleanup_diagnostic(
                0, 0,
                "WebGL buffers should be properly deleted to prevent memory leaks"
            ));
        }

        if code.contains("createShader()") && !code.contains("deleteShader") {
            diagnostics.push(create_webgl_resource_cleanup_diagnostic(
                0, 0,
                "WebGL shaders should be properly deleted after program linking"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebGLResourceCleanup {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.92,
            suggestion: "Implement resource cleanup: `gl.deleteTexture(texture); gl.deleteBuffer(buffer);` in disposal methods".to_string(),
            fix_code: Some("class WebGLResource { dispose() { if (this.texture) { gl.deleteTexture(this.texture); this.texture = null; } } }".to_string()),
        }).collect()
    }
}

/// Rule: require-frame-rate-monitoring
/// Enforces frame rate monitoring and performance optimization in games
#[derive(Clone)]
pub struct RequireFrameRateMonitoring;

impl RequireFrameRateMonitoring {
    pub const NAME: &'static str = "require-frame-rate-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireFrameRateMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for game loops without frame rate monitoring
        if code.contains("requestAnimationFrame") && !code.contains("fps") && !code.contains("frameTime") {
            diagnostics.push(create_frame_rate_monitoring_diagnostic(
                0, 0,
                "Game loops should monitor frame rate for performance optimization"
            ));
        }

        // Check for render loops without performance tracking
        if code.contains("render()") && !code.contains("performance.now") {
            diagnostics.push(create_frame_rate_monitoring_diagnostic(
                0, 0,
                "Render functions should track performance metrics"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireFrameRateMonitoring {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Add frame rate monitoring: `const frameTime = performance.now(); const fps = 1000 / deltaTime;`".to_string(),
            fix_code: Some("let lastFrameTime = performance.now(); function gameLoop() { const now = performance.now(); const deltaTime = now - lastFrameTime; const fps = 1000 / deltaTime; lastFrameTime = now; }".to_string()),
        }).collect()
    }
}

/// Rule: require-webxr-session-management
/// Enforces proper WebXR session lifecycle management
#[derive(Clone)]
pub struct RequireWebXRSessionManagement;

impl RequireWebXRSessionManagement {
    pub const NAME: &'static str = "require-webxr-session-management";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebXRSessionManagement {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for WebXR session creation without proper cleanup
        if code.contains("requestSession") && !code.contains("session.end") {
            diagnostics.push(create_webxr_session_management_diagnostic(
                0, 0,
                "WebXR sessions should be properly ended to free resources"
            ));
        }

        // Check for WebXR reference spaces without error handling
        if code.contains("requestReferenceSpace") && !code.contains("catch") {
            diagnostics.push(create_webxr_session_management_diagnostic(
                0, 0,
                "WebXR reference space requests should include error handling"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebXRSessionManagement {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Add session lifecycle management: `session.addEventListener('end', cleanup); await session.end();`".to_string(),
            fix_code: Some("class XRSessionManager { async endSession() { if (this.session) { await this.session.end(); this.session = null; } } }".to_string()),
        }).collect()
    }
}

/// Rule: require-shader-error-handling
/// Enforces error handling in shader compilation and linking
#[derive(Clone)]
pub struct RequireShaderErrorHandling;

impl RequireShaderErrorHandling {
    pub const NAME: &'static str = "require-shader-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireShaderErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for shader compilation without error checking
        if code.contains("compileShader") && !code.contains("getShaderParameter") {
            diagnostics.push(create_shader_error_handling_diagnostic(
                0, 0,
                "Shader compilation should check for compilation errors"
            ));
        }

        // Check for program linking without error checking
        if code.contains("linkProgram") && !code.contains("getProgramParameter") {
            diagnostics.push(create_shader_error_handling_diagnostic(
                0, 0,
                "Program linking should check for linking errors"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireShaderErrorHandling {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.94,
            suggestion: "Add shader error checking: `if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) { console.error(gl.getShaderInfoLog(shader)); }`".to_string(),
            fix_code: Some("function compileShaderWithErrorCheck(gl, source, type) { const shader = gl.createShader(type); gl.shaderSource(shader, source); gl.compileShader(shader); if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) { throw new Error(gl.getShaderInfoLog(shader)); } return shader; }".to_string()),
        }).collect()
    }
}

/// Rule: require-audio-context-resumption
/// Enforces proper Web Audio API context resumption for user interaction
#[derive(Clone)]
pub struct RequireAudioContextResumption;

impl RequireAudioContextResumption {
    pub const NAME: &'static str = "require-audio-context-resumption";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAudioContextResumption {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for AudioContext usage without resumption handling
        if code.contains("AudioContext") && !code.contains("resume()") && !code.contains("state") {
            diagnostics.push(create_audio_context_resumption_diagnostic(
                0, 0,
                "AudioContext should handle suspended state and resumption"
            ));
        }

        // Check for audio playback without user gesture handling
        if code.contains("play()") && !code.contains("click") && !code.contains("touch") {
            diagnostics.push(create_audio_context_resumption_diagnostic(
                0, 0,
                "Audio playback should be initiated by user gesture"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAudioContextResumption {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Add audio context resumption: `if (audioContext.state === 'suspended') { await audioContext.resume(); }`".to_string(),
            fix_code: Some("async function ensureAudioContextRunning(audioContext) { if (audioContext.state === 'suspended') { await audioContext.resume(); } }".to_string()),
        }).collect()
    }
}

/// Rule: require-canvas-performance-optimization
/// Enforces canvas performance optimization techniques
#[derive(Clone)]
pub struct RequireCanvasPerformanceOptimization;

impl RequireCanvasPerformanceOptimization {
    pub const NAME: &'static str = "require-canvas-performance-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCanvasPerformanceOptimization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for inefficient canvas clearing
        if code.contains("clearRect(0, 0,") && code.contains("canvas.width") {
            diagnostics.push(create_canvas_performance_optimization_diagnostic(
                0, 0,
                "Use canvas.width = canvas.width for faster clearing instead of clearRect"
            ));
        }

        // Check for missing offscreen canvas usage
        if code.contains("getContext('2d')") && !code.contains("OffscreenCanvas") && code.contains("complex") {
            diagnostics.push(create_canvas_performance_optimization_diagnostic(
                0, 0,
                "Consider using OffscreenCanvas for complex rendering operations"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireCanvasPerformanceOptimization {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.85,
            suggestion: "Optimize canvas performance: Use `canvas.width = canvas.width` for clearing, implement dirty rectangle tracking".to_string(),
            fix_code: Some("// Fast clear: canvas.width = canvas.width; // Dirty rectangles: const dirtyRegions = []; ctx.clearRect(x, y, width, height);".to_string()),
        }).collect()
    }
}

/// Rule: require-entity-component-system
/// Enforces Entity-Component-System architecture patterns in games
#[derive(Clone)]
pub struct RequireEntityComponentSystem;

impl RequireEntityComponentSystem {
    pub const NAME: &'static str = "require-entity-component-system";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireEntityComponentSystem {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for game objects without component separation
        if code.contains("class GameObject") && !code.contains("Component") {
            diagnostics.push(create_entity_component_system_diagnostic(
                0, 0,
                "Game objects should use Entity-Component-System architecture"
            ));
        }

        // Check for tightly coupled game logic
        if code.contains("update()") && code.contains("render()") && code.contains("physics") {
            diagnostics.push(create_entity_component_system_diagnostic(
                0, 0,
                "Game systems should be decoupled using ECS patterns"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireEntityComponentSystem {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.82,
            suggestion: "Implement ECS architecture: `class Entity { components: Map<string, Component> } class System { update(entities: Entity[]) }`".to_string(),
            fix_code: Some("abstract class Component {} class TransformComponent extends Component { x = 0; y = 0; } class RenderSystem { update(entities: Entity[]) { entities.filter(e => e.has(TransformComponent)).forEach(render); } }".to_string()),
        }).collect()
    }
}

/// Rule: require-asset-preloading
/// Enforces asset preloading strategies for smooth gameplay
#[derive(Clone)]
pub struct RequireAssetPreloading;

impl RequireAssetPreloading {
    pub const NAME: &'static str = "require-asset-preloading";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAssetPreloading {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for image loading without preloading
        if code.contains("new Image()") && !code.contains("preload") {
            diagnostics.push(create_asset_preloading_diagnostic(
                0, 0,
                "Images should be preloaded to prevent gameplay stuttering"
            ));
        }

        // Check for audio loading without buffering
        if code.contains("Audio()") && !code.contains("canplaythrough") {
            diagnostics.push(create_asset_preloading_diagnostic(
                0, 0,
                "Audio assets should be preloaded and buffered"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAssetPreloading {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Implement asset preloading: `await Promise.all(assets.map(url => preloadAsset(url)));`".to_string(),
            fix_code: Some("class AssetLoader { async preloadImage(url) { return new Promise((resolve, reject) => { const img = new Image(); img.onload = () => resolve(img); img.onerror = reject; img.src = url; }); } }".to_string()),
        }).collect()
    }
}

/// Rule: require-input-buffering
/// Enforces input buffering for responsive game controls
#[derive(Clone)]
pub struct RequireInputBuffering;

impl RequireInputBuffering {
    pub const NAME: &'static str = "require-input-buffering";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireInputBuffering {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct input handling without buffering
        if code.contains("keydown") && !code.contains("inputBuffer") && !code.contains("queue") {
            diagnostics.push(create_input_buffering_diagnostic(
                0, 0,
                "Input events should be buffered for consistent game response"
            ));
        }

        // Check for touch events without gesture recognition
        if code.contains("touchstart") && !code.contains("gesture") && !code.contains("buffer") {
            diagnostics.push(create_input_buffering_diagnostic(
                0, 0,
                "Touch input should implement gesture buffering and recognition"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireInputBuffering {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Implement input buffering: `class InputBuffer { buffer: InputEvent[]; processInput() { /* consume buffered inputs */ } }`".to_string(),
            fix_code: Some("class InputManager { private inputBuffer: KeyboardEvent[] = []; bufferInput(event: KeyboardEvent) { this.inputBuffer.push(event); } processBufferedInputs() { this.inputBuffer.forEach(this.handleInput); this.inputBuffer.length = 0; } }".to_string()),
        }).collect()
    }
}

/// Rule: require-webgl-state-management
/// Enforces proper WebGL state management to prevent rendering issues
#[derive(Clone)]
pub struct RequireWebGLStateManagement;

impl RequireWebGLStateManagement {
    pub const NAME: &'static str = "require-webgl-state-management";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebGLStateManagement {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for WebGL calls without state management
        if code.contains("useProgram") && !code.contains("currentProgram") {
            diagnostics.push(create_webgl_state_management_diagnostic(
                0, 0,
                "WebGL program usage should track current state to avoid redundant calls"
            ));
        }

        // Check for texture binding without state tracking
        if code.contains("bindTexture") && !code.contains("boundTexture") {
            diagnostics.push(create_webgl_state_management_diagnostic(
                0, 0,
                "WebGL texture binding should track state to prevent redundant operations"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebGLStateManagement {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Implement WebGL state management: `class WebGLStateManager { currentProgram: WebGLProgram; useProgram(program) { if (this.currentProgram !== program) { gl.useProgram(program); this.currentProgram = program; } } }`".to_string(),
            fix_code: Some("class WebGLContext { private state = { currentProgram: null, boundTextures: new Map() }; useProgram(program) { if (this.state.currentProgram !== program) { this.gl.useProgram(program); this.state.currentProgram = program; } } }".to_string()),
        }).collect()
    }
}

/// Rule: require-webxr-feature-detection
/// Enforces proper WebXR feature detection and fallbacks
#[derive(Clone)]
pub struct RequireWebXRFeatureDetection;

impl RequireWebXRFeatureDetection {
    pub const NAME: &'static str = "require-webxr-feature-detection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireWebXRFeatureDetection {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for WebXR usage without feature detection
        if code.contains("navigator.xr") && !code.contains("isSessionSupported") {
            diagnostics.push(create_webxr_feature_detection_diagnostic(
                0, 0,
                "WebXR usage should include feature detection and graceful fallbacks"
            ));
        }

        // Check for required features without availability checking
        if code.contains("requiredFeatures") && !code.contains("isSessionSupported") {
            diagnostics.push(create_webxr_feature_detection_diagnostic(
                0, 0,
                "WebXR required features should be checked for availability"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebXRFeatureDetection {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.93,
            suggestion: "Add WebXR feature detection: `const supported = await navigator.xr.isSessionSupported('immersive-vr');`".to_string(),
            fix_code: Some("async function checkWebXRSupport() { if (!navigator.xr) return false; try { return await navigator.xr.isSessionSupported('immersive-vr'); } catch { return false; } }".to_string()),
        }).collect()
    }
}

/// Rule: require-performance-profiling
/// Enforces performance profiling in graphics-intensive applications
#[derive(Clone)]
pub struct RequirePerformanceProfiling;

impl RequirePerformanceProfiling {
    pub const NAME: &'static str = "require-performance-profiling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePerformanceProfiling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for graphics operations without profiling
        if code.contains("drawArrays") && !code.contains("performance.mark") {
            diagnostics.push(create_performance_profiling_diagnostic(
                0, 0,
                "Graphics rendering operations should include performance profiling"
            ));
        }

        // Check for animation loops without timing analysis
        if code.contains("requestAnimationFrame") && !code.contains("timing") {
            diagnostics.push(create_performance_profiling_diagnostic(
                0, 0,
                "Animation loops should track frame timing for optimization"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePerformanceProfiling {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Add performance profiling: `performance.mark('render-start'); /* rendering */ performance.measure('render-time', 'render-start');`".to_string(),
            fix_code: Some("class PerformanceProfiler { startTimer(name: string) { performance.mark(`${name}-start`); } endTimer(name: string) { performance.mark(`${name}-end`); performance.measure(name, `${name}-start`, `${name}-end`); } }".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_webgl_resource_cleanup_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebGLResourceCleanup::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_frame_rate_monitoring_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireFrameRateMonitoring::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_webxr_session_management_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebXRSessionManagement::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_shader_error_handling_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireShaderErrorHandling::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_audio_context_resumption_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAudioContextResumption::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_canvas_performance_optimization_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireCanvasPerformanceOptimization::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_entity_component_system_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireEntityComponentSystem::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_asset_preloading_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAssetPreloading::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_input_buffering_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireInputBuffering::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_webgl_state_management_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebGLStateManagement::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_webxr_feature_detection_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebXRFeatureDetection::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_performance_profiling_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePerformanceProfiling::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_webgl_resource_cleanup_violation() {
        let code = r#"
        function createTexture() {
            const texture = gl.createTexture();
            gl.bindTexture(gl.TEXTURE_2D, texture);
            return texture;
        }
        "#;

        let rule = RequireWebGLResourceCleanup;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("deleted"));
    }

    #[test]
    fn test_require_webgl_resource_cleanup_compliant() {
        let code = r#"
        class TextureManager {
            createTexture() {
                const texture = gl.createTexture();
                gl.bindTexture(gl.TEXTURE_2D, texture);
                return texture;
            }

            dispose(texture) {
                gl.deleteTexture(texture);
            }
        }
        "#;

        let rule = RequireWebGLResourceCleanup;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_frame_rate_monitoring_violation() {
        let code = r#"
        function gameLoop() {
            update();
            render();
            requestAnimationFrame(gameLoop);
        }
        "#;

        let rule = RequireFrameRateMonitoring;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("frame rate"));
    }

    #[test]
    fn test_require_frame_rate_monitoring_compliant() {
        let code = r#"
        let lastFrameTime = performance.now();
        function gameLoop() {
            const now = performance.now();
            const deltaTime = now - lastFrameTime;
            const fps = 1000 / deltaTime;

            update(deltaTime);
            render();
            lastFrameTime = now;
            requestAnimationFrame(gameLoop);
        }
        "#;

        let rule = RequireFrameRateMonitoring;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_webxr_session_management_violation() {
        let code = r#"
        async function startXR() {
            const session = await navigator.xr.requestSession('immersive-vr');
            setupRenderer(session);
        }
        "#;

        let rule = RequireWebXRSessionManagement;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("ended"));
    }

    #[test]
    fn test_require_webxr_session_management_compliant() {
        let code = r#"
        async function startXR() {
            const session = await navigator.xr.requestSession('immersive-vr');
            session.addEventListener('end', cleanup);
            setupRenderer(session);

            // Later: await session.end();
        }
        "#;

        let rule = RequireWebXRSessionManagement;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_ai_enhancement_webgl_resource_cleanup() {
        let rule = RequireWebGLResourceCleanup;
        let diagnostics = vec![create_webgl_resource_cleanup_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("deleteTexture"));
    }

    #[test]
    fn test_ai_enhancement_frame_rate_monitoring() {
        let rule = RequireFrameRateMonitoring;
        let diagnostics = vec![create_frame_rate_monitoring_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].suggestion.contains("performance.now"));
    }
}