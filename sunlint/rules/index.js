/**
 * SunLint Heuristic Rules Registry
 * Central reconst securityRules = {
  S015: require('./security/S015_insecure_tls_certificate/analyzer'),
  S023: require('./security/S023_no_json_injection/analyzer'),stry for all heuristic rules organized by category
 */

const path = require('path');

/**
 * Load rule analyzer from category folder
 * @param {string} category - Rule category (common, security, typescript)
 * @param {string} ruleId - Rule ID (e.g., C006_function_naming)
 * @returns {Object} Rule analyzer module
 */
function loadRule(category, ruleId) {
    try {
        // Special case for C047: Use semantic analyzer by default
        if (ruleId === 'C047_no_duplicate_retry_logic') {
            const semanticPath = path.join(__dirname, category, ruleId, 'c047-semantic-rule.js');
            console.log(`🔬 Loading C047 semantic analyzer: ${semanticPath}`);
            return require(semanticPath);
        }
        
        const rulePath = path.join(__dirname, category, ruleId, 'analyzer.js');
        return require(rulePath);
    } catch (error) {
        console.warn(`Failed to load rule ${category}/${ruleId}:`, error.message);
        return null;
    }
}

/**
 * Load rule configuration
 * @param {string} category - Rule category
 * @param {string} ruleId - Rule ID
 * @returns {Object} Rule configuration
 */
function loadRuleConfig(category, ruleId) {
    try {
        const configPath = path.join(__dirname, category, ruleId, 'config.json');
        return require(configPath);
    } catch (error) {
        console.warn(`Failed to load config for ${category}/${ruleId}:`, error.message);
        return {};
    }
}

// 🔹 Common Rules (C-series) - General coding standards
const commonRules = {
    C006: loadRule('common', 'C006_function_naming'),
    C012: loadRule('common', 'C012_command_query_separation'),
    C013: loadRule('common', 'C013_no_dead_code'),
    C014: loadRule('common', 'C014_dependency_injection'),
    C018: loadRule('common', 'C018_no_throw_generic_error'),
    C019: loadRule('common', 'C019_log_level_usage'), 
    C030: loadRule('common', 'C030_use_custom_error_classes'),
    C023: loadRule('common', 'C023_no_duplicate_variable'),
    C024: loadRule('common', 'C024_no_scatter_hardcoded_constants'),
    C029: loadRule('common', 'C029_catch_block_logging'),
    C031: loadRule('common', 'C031_validation_separation'),
    C041: loadRule('common', 'C041_no_sensitive_hardcode'),
    C042: loadRule('common', 'C042_boolean_name_prefix'),
    C048: loadRule('common', 'C048_no_bypass_architectural_layers'),
    C052: loadRule('common', 'C052_parsing_or_data_transformation'),
    C047: loadRule('common', 'C047_no_duplicate_retry_logic'),
};

// 🔒 Security Rules (S-series) - Ready for migration
const securityRules = {
    S006: loadRule('security', 'S006_no_plaintext_recovery_codes'),
    S015: loadRule('security', 'S015_insecure_tls_certificate'),
    S023: loadRule('security', 'S023_no_json_injection'),
    S026: loadRule('security', 'S026_json_schema_validation'),
    S027: loadRule('security', 'S027_no_hardcoded_secrets'),
    S029: loadRule('security', 'S029_csrf_protection'),
    // S001: loadRule('security', 'S001_fail_securely'),
    // S003: loadRule('security', 'S003_no_unvalidated_redirect'),
    // S012: loadRule('security', 'S012_hardcode_secret'),
    // ... 46 more security rules ready for migration
};

// 📘 TypeScript Rules (T-series) - Ready for migration  
const typescriptRules = {
    // T002: loadRule('typescript', 'T002_interface_prefix_i'),
    // T003: loadRule('typescript', 'T003_ts_ignore_reason'),
    // T004: loadRule('typescript', 'T004_interface_public_only'),
    // ... 7 more TypeScript rules ready for migration
};

/**
 * Get all available rules by category
 * @returns {Object} Organized rules object
 */
function getAllRules() {
    return {
        common: commonRules,
        security: securityRules,
        typescript: typescriptRules
    };
}

/**
 * Get rule by ID (searches all categories)
 * @param {string} ruleId - Rule ID (e.g., 'C006', 'S001', 'T002')
 * @returns {Object|null} Rule analyzer or null if not found
 */
function getRuleById(ruleId) {
    // Check all categories for the rule
    if (commonRules[ruleId]) return commonRules[ruleId];
    if (securityRules[ruleId]) return securityRules[ruleId];
    if (typescriptRules[ruleId]) return typescriptRules[ruleId];
    
    return null;
}

/**
 * Get active rule count by category
 * @returns {Object} Rule counts
 */
function getRuleCounts() {
    const counts = {
        common: Object.keys(commonRules).filter(id => commonRules[id]).length,
        security: Object.keys(securityRules).filter(id => securityRules[id]).length,
        typescript: Object.keys(typescriptRules).filter(id => typescriptRules[id]).length,
    };
    
    counts.total = counts.common + counts.security + counts.typescript;
    return counts;
}

/**
 * List all available rules with metadata
 * @returns {Array} Array of rule information
 */
function listRules() {
    const rules = [];
    const allRules = getAllRules();
    
    for (const category in allRules) {
        for (const ruleId in allRules[category]) {
            if (allRules[category][ruleId]) {
                const config = loadRuleConfig(category, ruleId);
                rules.push({
                    id: ruleId,
                    category,
                    name: config.name || ruleId,
                    description: config.description || 'No description',
                    status: 'active'
                });
            }
        }
    }
    
    return rules;
}

module.exports = {
    // Main exports
    getAllRules,
    getRuleById,
    getRuleCounts,
    listRules,
    
    // Category exports
    common: commonRules,
    security: securityRules,
    typescript: typescriptRules
};
