//! # OXC Blockchain and Web3 Development Rules
//!
//! This module implements WASM-safe OXC rules for blockchain and Web3 development,
//! including smart contract integration, DeFi patterns, wallet security, and
//! decentralized application best practices.
//!
//! ## Rule Categories:
//! - **Smart Contract Integration**: ABI handling, contract interaction patterns, gas optimization
//! - **Wallet Security**: Private key management, transaction signing, secure storage
//! - **DeFi Patterns**: Token standards, DEX integration, liquidity pool interactions
//! - **Transaction Security**: Replay attack prevention, nonce management, fee estimation
//! - **Web3 Provider Management**: RPC endpoints, network switching, connection handling
//! - **NFT and Token Handling**: ERC standards, metadata validation, ownership verification
//! - **Gas Optimization**: Transaction batching, efficient contract calls, cost reduction
//! - **Decentralized Storage**: IPFS integration, content addressing, distributed data
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
// Smart Contract Integration and Security Rules
// ================================================================================================

/// Prevents hardcoded private keys in Web3 applications
pub struct NoHardcodedPrivateKeys;

impl NoHardcodedPrivateKeys {
    pub const NAME: &'static str = "no-hardcoded-private-keys";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoHardcodedPrivateKeys {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Detect common private key patterns
        if (code.contains("privateKey") || code.contains("private_key")) &&
           (code.contains("0x") || code.contains("\"") || code.contains("'")) &&
           !code.contains("process.env") && !code.contains("getEnv") {
            diagnostics.push(create_hardcoded_private_keys_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoHardcodedPrivateKeys {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Never hardcode private keys in source code. Use environment variables, secure key management services, or hardware wallets for production applications".to_string(),
            confidence: 0.99,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires proper gas limit estimation for transactions
pub struct RequireGasEstimation;

impl RequireGasEstimation {
    pub const NAME: &'static str = "require-gas-estimation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireGasEstimation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("sendTransaction") || code.contains("estimateGas")) &&
           code.contains("gasLimit") && !code.contains("estimateGas") {
            diagnostics.push(create_gas_estimation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireGasEstimation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use estimateGas() to calculate appropriate gas limits instead of hardcoded values to prevent transaction failures and optimize costs".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces proper error handling for contract calls
pub struct RequireContractErrorHandling;

impl RequireContractErrorHandling {
    pub const NAME: &'static str = "require-contract-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireContractErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("contract.") || code.contains("call()")) &&
           code.contains("await") && !code.contains("catch") && !code.contains("try") {
            diagnostics.push(create_contract_error_handling_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireContractErrorHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add try-catch blocks around contract calls to handle reverts, network failures, and insufficient gas gracefully".to_string(),
            confidence: 0.95,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// DeFi and Token Handling Rules
// ================================================================================================

/// Requires slippage protection for DeFi transactions
pub struct RequireSlippageProtection;

impl RequireSlippageProtection {
    pub const NAME: &'static str = "require-slippage-protection";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSlippageProtection {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("swap") || code.contains("exchange")) &&
           !code.contains("slippage") && !code.contains("amountOutMin") &&
           !code.contains("deadline") {
            diagnostics.push(create_slippage_protection_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSlippageProtection {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add slippage protection with amountOutMin and deadline parameters to prevent MEV attacks and price manipulation in DeFi swaps".to_string(),
            confidence: 0.96,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces proper token approval patterns
pub struct RequireTokenApprovalCheck;

impl RequireTokenApprovalCheck {
    pub const NAME: &'static str = "require-token-approval-check";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTokenApprovalCheck {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("approve") && code.contains("MAX_UINT256") &&
           !code.contains("allowance") && !code.contains("check") {
            diagnostics.push(create_token_approval_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTokenApprovalCheck {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Check current allowance before approving tokens and avoid infinite approvals for security. Use specific amounts when possible".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents reentrancy vulnerabilities in DeFi interactions
pub struct NoReentrancyVulnerability;

impl NoReentrancyVulnerability {
    pub const NAME: &'static str = "no-reentrancy-vulnerability";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoReentrancyVulnerability {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("external") && code.contains("call") &&
           code.contains("balance") && !code.contains("nonReentrant") &&
           !code.contains("mutex") && !code.contains("lock") {
            diagnostics.push(create_reentrancy_vulnerability_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoReentrancyVulnerability {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement reentrancy guards or use checks-effects-interactions pattern to prevent reentrancy attacks in DeFi protocols".to_string(),
            confidence: 0.97,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Web3 Provider and Network Management Rules
// ================================================================================================

/// Requires proper Web3 provider connection handling
pub struct RequireProviderConnectionHandling;

impl RequireProviderConnectionHandling {
    pub const NAME: &'static str = "require-provider-connection-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireProviderConnectionHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("Web3Provider") || code.contains("JsonRpcProvider")) &&
           !code.contains("on('disconnect')") && !code.contains("addEventListener") {
            diagnostics.push(create_provider_connection_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireProviderConnectionHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add event handlers for provider disconnect, network changes, and account changes to maintain consistent Web3 connection state".to_string(),
            confidence: 0.89,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces network validation for multi-chain applications
pub struct RequireNetworkValidation;

impl RequireNetworkValidation {
    pub const NAME: &'static str = "require-network-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireNetworkValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("chainId") || code.contains("network")) &&
           code.contains("contract") && !code.contains("validate") &&
           !code.contains("check") && !code.contains("verify") {
            diagnostics.push(create_network_validation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireNetworkValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Validate network/chainId before contract interactions to prevent cross-chain errors and ensure contracts exist on the current network".to_string(),
            confidence: 0.91,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// NFT and Metadata Handling Rules
// ================================================================================================

/// Requires proper NFT metadata validation
pub struct RequireNftMetadataValidation;

impl RequireNftMetadataValidation {
    pub const NAME: &'static str = "require-nft-metadata-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireNftMetadataValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("tokenURI") || code.contains("metadata")) &&
           code.contains("fetch") && !code.contains("validate") &&
           !code.contains("schema") && !code.contains("verify") {
            diagnostics.push(create_nft_metadata_validation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireNftMetadataValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Validate NFT metadata against expected schema and handle IPFS/HTTP failures gracefully when fetching tokenURI data".to_string(),
            confidence: 0.88,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents unauthorized NFT operations
pub struct RequireNftOwnershipVerification;

impl RequireNftOwnershipVerification {
    pub const NAME: &'static str = "require-nft-ownership-verification";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireNftOwnershipVerification {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("transfer") || code.contains("approve")) &&
           code.contains("tokenId") && !code.contains("ownerOf") &&
           !code.contains("balanceOf") && !code.contains("verify") {
            diagnostics.push(create_nft_ownership_verification_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireNftOwnershipVerification {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Verify NFT ownership using ownerOf() before allowing transfers or approvals to prevent unauthorized operations".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Gas Optimization and Transaction Efficiency Rules
// ================================================================================================

/// Requires transaction batching for multiple operations
pub struct RequireTransactionBatching;

impl RequireTransactionBatching {
    pub const NAME: &'static str = "require-transaction-batching";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTransactionBatching {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("for") && code.contains("sendTransaction") &&
           !code.contains("batch") && !code.contains("multicall") {
            diagnostics.push(create_transaction_batching_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTransactionBatching {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use transaction batching or multicall patterns to reduce gas costs and improve efficiency for multiple contract interactions".to_string(),
            confidence: 0.85,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents excessive gas limit settings
pub struct NoExcessiveGasLimits;

impl NoExcessiveGasLimits {
    pub const NAME: &'static str = "no-excessive-gas-limits";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoExcessiveGasLimits {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("gasLimit") &&
           (code.contains("10000000") || code.contains("21000000")) {
            diagnostics.push(create_excessive_gas_limits_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoExcessiveGasLimits {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use reasonable gas limits based on estimateGas() results plus buffer instead of excessive hardcoded values to avoid unnecessary costs".to_string(),
            confidence: 0.90,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_hardcoded_private_keys_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoHardcodedPrivateKeys::NAME.to_string(),
        message: "Private keys must never be hardcoded in source code".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use environment variables or secure key management".to_string()),
    }
}

fn create_gas_estimation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireGasEstimation::NAME.to_string(),
        message: "Use gas estimation instead of hardcoded gas limits".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Call estimateGas() before sending transactions".to_string()),
    }
}

fn create_contract_error_handling_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireContractErrorHandling::NAME.to_string(),
        message: "Contract calls must include proper error handling".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add try-catch blocks around contract interactions".to_string()),
    }
}

fn create_slippage_protection_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSlippageProtection::NAME.to_string(),
        message: "DeFi swaps must include slippage protection".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add amountOutMin and deadline parameters".to_string()),
    }
}

fn create_token_approval_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTokenApprovalCheck::NAME.to_string(),
        message: "Check current allowance before token approvals".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Verify existing allowance and use specific amounts".to_string()),
    }
}

fn create_reentrancy_vulnerability_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoReentrancyVulnerability::NAME.to_string(),
        message: "Potential reentrancy vulnerability in external call".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Implement reentrancy guards or checks-effects-interactions pattern".to_string()),
    }
}

fn create_provider_connection_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireProviderConnectionHandling::NAME.to_string(),
        message: "Web3 providers should handle connection events".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add disconnect and network change event handlers".to_string()),
    }
}

fn create_network_validation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireNetworkValidation::NAME.to_string(),
        message: "Validate network before contract interactions".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Check chainId matches expected network".to_string()),
    }
}

fn create_nft_metadata_validation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireNftMetadataValidation::NAME.to_string(),
        message: "NFT metadata should be validated against expected schema".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add metadata schema validation and error handling".to_string()),
    }
}

fn create_nft_ownership_verification_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireNftOwnershipVerification::NAME.to_string(),
        message: "Verify NFT ownership before operations".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Check ownerOf() before transfers or approvals".to_string()),
    }
}

fn create_transaction_batching_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTransactionBatching::NAME.to_string(),
        message: "Consider transaction batching for multiple operations".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use multicall or batch transactions for efficiency".to_string()),
    }
}

fn create_excessive_gas_limits_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoExcessiveGasLimits::NAME.to_string(),
        message: "Avoid excessive gas limits to prevent unnecessary costs".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use estimateGas() plus reasonable buffer".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardcoded_private_keys_detection() {
        let code = r#"const privateKey = "0x1234567890abcdef";"#;
        let rule = NoHardcodedPrivateKeys;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoHardcodedPrivateKeys::NAME);
    }

    #[test]
    fn test_contract_error_handling_detection() {
        let code = r#"const result = await contract.transfer(to, amount);"#;
        let rule = RequireContractErrorHandling;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireContractErrorHandling::NAME);
    }

    #[test]
    fn test_slippage_protection_detection() {
        let code = r#"router.swapExactTokensForTokens(amountIn, amountOut, path, to);"#;
        let rule = RequireSlippageProtection;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireSlippageProtection::NAME);
    }

    #[test]
    fn test_token_approval_detection() {
        let code = r#"await token.approve(spender, MAX_UINT256);"#;
        let rule = RequireTokenApprovalCheck;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireTokenApprovalCheck::NAME);
    }

    #[test]
    fn test_nft_ownership_verification_detection() {
        let code = r#"await nft.transfer(to, tokenId);"#;
        let rule = RequireNftOwnershipVerification;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireNftOwnershipVerification::NAME);
    }

    #[test]
    fn test_transaction_batching_detection() {
        let code = r#"for (const tx of transactions) { await sendTransaction(tx); }"#;
        let rule = RequireTransactionBatching;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireTransactionBatching::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"const privateKey = "0xabc123";"#;
        let rule = NoHardcodedPrivateKeys;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.95);
        assert!(!suggestions[0].auto_fixable); // Security issue requires manual review
    }
}