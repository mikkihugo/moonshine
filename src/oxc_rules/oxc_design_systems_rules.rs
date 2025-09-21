//! # Design Systems & Component Library Rules
//!
//! Comprehensive rules for design systems, component libraries, design tokens,
//! and scalable UI architecture patterns.
//!
//! ## Rule Categories:
//! - **Design Tokens**: Color, typography, spacing, motion consistency
//! - **Component Architecture**: Reusable components, composition patterns, prop APIs
//! - **Accessibility**: WCAG compliance, keyboard navigation, screen reader support
//! - **Theming**: Dark/light modes, custom themes, CSS custom properties
//! - **Documentation**: Storybook patterns, usage examples, design guidelines
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Design token usage
//! const Button = styled.button`
//!   padding: ${tokens.spacing.md};
//!   color: ${tokens.colors.primary.text};
//!   font-family: ${tokens.typography.fontFamily.body};
//! `;
//!
//! // ❌ Bad: Hardcoded design values
//! const Button = styled.button`
//!   padding: 16px;
//!   color: #007bff;
//!   font-family: 'Helvetica Neue';
//! `;
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

/// Rule: require-design-tokens
/// Enforces the use of design tokens instead of hardcoded values
#[derive(Clone)]
pub struct RequireDesignTokens;

impl RequireDesignTokens {
    pub const NAME: &'static str = "require-design-tokens";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDesignTokens {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for hardcoded colors
        if code.contains("#") && (code.contains("color") || code.contains("background")) && !code.contains("tokens") && !code.contains("theme") {
            diagnostics.push(create_design_tokens_diagnostic(
                0, 0,
                "Use design tokens for colors instead of hardcoded hex values"
            ));
        }

        // Check for hardcoded spacing values
        if (code.contains("padding:") || code.contains("margin:")) && (code.contains("px") || code.contains("rem")) && !code.contains("tokens") {
            diagnostics.push(create_design_tokens_diagnostic(
                0, 0,
                "Use design tokens for spacing instead of hardcoded pixel or rem values"
            ));
        }

        // Check for hardcoded font families
        if code.contains("font-family") && code.contains("'") && !code.contains("tokens") && !code.contains("theme") {
            diagnostics.push(create_design_tokens_diagnostic(
                0, 0,
                "Use design tokens for typography instead of hardcoded font families"
            ));
        }

        // Check for hardcoded z-index values
        if code.contains("z-index") && (code.contains("999") || code.contains("1000")) && !code.contains("tokens") {
            diagnostics.push(create_design_tokens_diagnostic(
                0, 0,
                "Use design tokens for z-index values to maintain layering consistency"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDesignTokens {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.92,
            suggestion: "Use design tokens: `color: ${tokens.colors.primary.main}`, `padding: ${tokens.spacing.md}`, `font-family: ${tokens.typography.fontFamily.body}`".to_string(),
            fix_code: Some("// Design token structure\nconst tokens = {\n  colors: {\n    primary: {\n      50: '#f0f9ff',\n      500: '#0ea5e9',\n      900: '#0c4a6e'\n    },\n    semantic: {\n      success: '#10b981',\n      warning: '#f59e0b',\n      error: '#ef4444'\n    }\n  },\n  spacing: {\n    xs: '0.25rem',\n    sm: '0.5rem',\n    md: '1rem',\n    lg: '1.5rem',\n    xl: '2rem'\n  },\n  typography: {\n    fontFamily: {\n      body: 'Inter, system-ui, sans-serif',\n      heading: 'Cal Sans, Inter, sans-serif',\n      mono: 'JetBrains Mono, monospace'\n    },\n    fontSize: {\n      xs: '0.75rem',\n      sm: '0.875rem',\n      base: '1rem',\n      lg: '1.125rem',\n      xl: '1.25rem'\n    }\n  },\n  zIndex: {\n    dropdown: 1000,\n    sticky: 1020,\n    fixed: 1030,\n    modal: 1040,\n    popover: 1050,\n    tooltip: 1060\n  }\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-component-composition
/// Enforces component composition patterns over inheritance
#[derive(Clone)]
pub struct RequireComponentComposition;

impl RequireComponentComposition {
    pub const NAME: &'static str = "require-component-composition";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireComponentComposition {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for component inheritance patterns
        if code.contains("extends Component") && code.contains("class") && !code.contains("// Legacy component") {
            diagnostics.push(create_component_composition_diagnostic(
                0, 0,
                "Prefer functional components with hooks over class inheritance"
            ));
        }

        // Check for large components without composition
        if code.contains("return (") && code.contains("<div>").count() > 5 && !code.contains("children") {
            diagnostics.push(create_component_composition_diagnostic(
                0, 0,
                "Large components should be broken down using composition patterns"
            ));
        }

        // Check for missing render props or children patterns
        if code.contains("component") && code.contains("render") && !code.contains("children") && !code.contains("render:") {
            diagnostics.push(create_component_composition_diagnostic(
                0, 0,
                "Consider using render props or children patterns for flexible composition"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireComponentComposition {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Use composition patterns: `const Card = ({ children, header, footer }) => <div><Header>{header}</Header><Content>{children}</Content><Footer>{footer}</Footer></div>`".to_string(),
            fix_code: Some("// Composition with compound components\nconst Card = ({ children }) => {\n  return <div className=\"card\">{children}</div>;\n};\n\nCard.Header = ({ children }) => (\n  <div className=\"card-header\">{children}</div>\n);\n\nCard.Body = ({ children }) => (\n  <div className=\"card-body\">{children}</div>\n);\n\nCard.Footer = ({ children }) => (\n  <div className=\"card-footer\">{children}</div>\n);\n\n// Render props pattern\nconst DataProvider = ({ children, url }) => {\n  const [data, setData] = useState(null);\n  const [loading, setLoading] = useState(true);\n  \n  useEffect(() => {\n    fetch(url).then(res => res.json()).then(data => {\n      setData(data);\n      setLoading(false);\n    });\n  }, [url]);\n  \n  return children({ data, loading });\n};\n\n// Usage\n<Card>\n  <Card.Header>Title</Card.Header>\n  <Card.Body>\n    <DataProvider url=\"/api/data\">\n      {({ data, loading }) => \n        loading ? 'Loading...' : JSON.stringify(data)\n      }\n    </DataProvider>\n  </Card.Body>\n  <Card.Footer>Actions</Card.Footer>\n</Card>".to_string()),
        }).collect()
    }
}

/// Rule: require-accessibility-props
/// Enforces accessibility properties in component APIs
#[derive(Clone)]
pub struct RequireAccessibilityProps;

impl RequireAccessibilityProps {
    pub const NAME: &'static str = "require-accessibility-props";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireAccessibilityProps {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for button components without accessibility props
        if code.contains("<button") && !code.contains("aria-label") && !code.contains("aria-describedby") {
            diagnostics.push(create_accessibility_props_diagnostic(
                0, 0,
                "Button components should include aria-label or aria-describedby for accessibility"
            ));
        }

        // Check for input components without labels
        if code.contains("<input") && !code.contains("aria-label") && !code.contains("htmlFor") {
            diagnostics.push(create_accessibility_props_diagnostic(
                0, 0,
                "Input components should be associated with labels via htmlFor or aria-label"
            ));
        }

        // Check for modal/dialog components without focus management
        if code.contains("modal") && !code.contains("focus") && !code.contains("trap") {
            diagnostics.push(create_accessibility_props_diagnostic(
                0, 0,
                "Modal components should implement focus management and focus trapping"
            ));
        }

        // Check for custom components without ARIA support
        if code.contains("component") && code.contains("interactive") && !code.contains("aria") && !code.contains("role") {
            diagnostics.push(create_accessibility_props_diagnostic(
                0, 0,
                "Interactive components should include appropriate ARIA attributes and roles"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAccessibilityProps {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.94,
            suggestion: "Add accessibility props: `<Button aria-label=\"Close dialog\" aria-describedby=\"dialog-description\" />`, implement focus management with `useFocusTrap` hook".to_string(),
            fix_code: Some("// Accessible Button component\ninterface ButtonProps {\n  children: React.ReactNode;\n  'aria-label'?: string;\n  'aria-describedby'?: string;\n  disabled?: boolean;\n  variant?: 'primary' | 'secondary';\n}\n\nconst Button = ({ children, 'aria-label': ariaLabel, 'aria-describedby': ariaDescribedBy, disabled, variant = 'primary', ...props }: ButtonProps) => (\n  <button\n    aria-label={ariaLabel}\n    aria-describedby={ariaDescribedBy}\n    disabled={disabled}\n    className={`btn btn-${variant}`}\n    {...props}\n  >\n    {children}\n  </button>\n);\n\n// Accessible Modal with focus management\nconst Modal = ({ isOpen, onClose, children, title }) => {\n  const modalRef = useRef<HTMLDivElement>(null);\n  const previousFocusRef = useRef<HTMLElement | null>(null);\n  \n  useEffect(() => {\n    if (isOpen) {\n      previousFocusRef.current = document.activeElement as HTMLElement;\n      modalRef.current?.focus();\n    } else {\n      previousFocusRef.current?.focus();\n    }\n  }, [isOpen]);\n  \n  const handleKeyDown = (e: KeyboardEvent) => {\n    if (e.key === 'Escape') {\n      onClose();\n    }\n  };\n  \n  if (!isOpen) return null;\n  \n  return (\n    <div\n      ref={modalRef}\n      role=\"dialog\"\n      aria-modal=\"true\"\n      aria-labelledby=\"modal-title\"\n      tabIndex={-1}\n      onKeyDown={handleKeyDown}\n      className=\"modal\"\n    >\n      <h2 id=\"modal-title\">{title}</h2>\n      {children}\n      <Button onClick={onClose} aria-label=\"Close modal\">\n        ×\n      </Button>\n    </div>\n  );\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-theme-support
/// Enforces theme support and CSS custom property usage
#[derive(Clone)]
pub struct RequireThemeSupport;

impl RequireThemeSupport {
    pub const NAME: &'static str = "require-theme-support";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireThemeSupport {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for components without theme provider support
        if code.contains("styled") && !code.contains("theme") && !code.contains("ThemeProvider") {
            diagnostics.push(create_theme_support_diagnostic(
                0, 0,
                "Styled components should support theming via ThemeProvider"
            ));
        }

        // Check for hardcoded dark/light mode values
        if (code.contains("dark") || code.contains("light")) && code.contains("mode") && !code.contains("theme") {
            diagnostics.push(create_theme_support_diagnostic(
                0, 0,
                "Dark/light mode should be managed through theme context, not hardcoded values"
            ));
        }

        // Check for CSS without custom properties
        if code.contains("background-color") && !code.contains("var(--") && !code.contains("theme") {
            diagnostics.push(create_theme_support_diagnostic(
                0, 0,
                "Use CSS custom properties for themeable values"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireThemeSupport {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Implement theme support: `const Button = styled.button\\`color: \\${({ theme }) => theme.colors.primary}; background: var(--button-bg);\\`;`".to_string(),
            fix_code: Some("// Theme configuration\nconst lightTheme = {\n  colors: {\n    primary: '#007bff',\n    background: '#ffffff',\n    text: '#212529'\n  },\n  spacing: {\n    sm: '0.5rem',\n    md: '1rem',\n    lg: '1.5rem'\n  }\n};\n\nconst darkTheme = {\n  colors: {\n    primary: '#0ea5e9',\n    background: '#1a202c',\n    text: '#f7fafc'\n  },\n  spacing: lightTheme.spacing // Inherit non-color tokens\n};\n\n// Theme Provider setup\nconst ThemeProvider = ({ children, theme }) => {\n  useEffect(() => {\n    // Set CSS custom properties\n    Object.entries(theme.colors).forEach(([key, value]) => {\n      document.documentElement.style.setProperty(`--color-${key}`, value);\n    });\n    Object.entries(theme.spacing).forEach(([key, value]) => {\n      document.documentElement.style.setProperty(`--spacing-${key}`, value);\n    });\n  }, [theme]);\n  \n  return (\n    <ThemeContext.Provider value={theme}>\n      {children}\n    </ThemeContext.Provider>\n  );\n};\n\n// Themed component\nconst Button = styled.button`\n  background-color: var(--color-primary);\n  color: var(--color-background);\n  padding: var(--spacing-md);\n  border: none;\n  border-radius: 4px;\n  \n  &:hover {\n    opacity: 0.8;\n  }\n  \n  &:focus {\n    outline: 2px solid var(--color-primary);\n    outline-offset: 2px;\n  }\n`;\n\n// Theme hook\nconst useTheme = () => {\n  const theme = useContext(ThemeContext);\n  const [isDark, setIsDark] = useState(false);\n  \n  const toggleTheme = () => setIsDark(!isDark);\n  \n  return {\n    theme: isDark ? darkTheme : lightTheme,\n    isDark,\n    toggleTheme\n  };\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-responsive-design
/// Enforces responsive design patterns and mobile-first approach
#[derive(Clone)]
pub struct RequireResponsiveDesign;

impl RequireResponsiveDesign {
    pub const NAME: &'static str = "require-responsive-design";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireResponsiveDesign {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for fixed widths without responsive alternatives
        if code.contains("width:") && code.contains("px") && !code.contains("@media") && !code.contains("responsive") {
            diagnostics.push(create_responsive_design_diagnostic(
                0, 0,
                "Fixed pixel widths should include responsive alternatives using breakpoints"
            ));
        }

        // Check for layout components without responsive props
        if code.contains("Grid") || code.contains("Flex") && !code.contains("responsive") && !code.contains("breakpoint") {
            diagnostics.push(create_responsive_design_diagnostic(
                0, 0,
                "Layout components should support responsive behavior via breakpoint props"
            ));
        }

        // Check for text sizing without responsive scaling
        if code.contains("font-size") && code.contains("px") && !code.contains("clamp") && !code.contains("@media") {
            diagnostics.push(create_responsive_design_diagnostic(
                0, 0,
                "Font sizes should be responsive using clamp() or media queries"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireResponsiveDesign {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Add responsive design: `font-size: clamp(1rem, 2.5vw, 1.5rem);`, `width: 100%; max-width: 1200px;`, use breakpoint props like `<Grid xs={12} md={6} lg={4} />`".to_string(),
            fix_code: Some("// Responsive breakpoints system\nconst breakpoints = {\n  xs: '0px',\n  sm: '576px',\n  md: '768px',\n  lg: '992px',\n  xl: '1200px',\n  xxl: '1400px'\n};\n\n// Media query helper\nconst media = {\n  up: (breakpoint) => `@media (min-width: ${breakpoints[breakpoint]})`,\n  down: (breakpoint) => `@media (max-width: ${breakpoints[breakpoint]})`,\n  between: (min, max) => `@media (min-width: ${breakpoints[min]}) and (max-width: ${breakpoints[max]})`\n};\n\n// Responsive Grid component\nconst Grid = styled.div`\n  display: grid;\n  gap: var(--spacing-md);\n  \n  /* Mobile first approach */\n  grid-template-columns: 1fr;\n  \n  ${media.up('sm')} {\n    grid-template-columns: repeat(2, 1fr);\n  }\n  \n  ${media.up('md')} {\n    grid-template-columns: repeat(3, 1fr);\n  }\n  \n  ${media.up('lg')} {\n    grid-template-columns: repeat(4, 1fr);\n  }\n`;\n\n// Responsive Typography\nconst Heading = styled.h1`\n  /* Fluid typography */\n  font-size: clamp(1.5rem, 4vw, 3rem);\n  line-height: 1.2;\n  \n  /* Responsive spacing */\n  margin-bottom: clamp(1rem, 2vw, 2rem);\n`;\n\n// Container with responsive padding\nconst Container = styled.div`\n  width: 100%;\n  max-width: 1200px;\n  margin: 0 auto;\n  \n  /* Responsive padding */\n  padding: 0 var(--spacing-md);\n  \n  ${media.up('lg')} {\n    padding: 0 var(--spacing-xl);\n  }\n`;\n\n// Responsive utility hook\nconst useResponsive = () => {\n  const [screenSize, setScreenSize] = useState('xs');\n  \n  useEffect(() => {\n    const handleResize = () => {\n      const width = window.innerWidth;\n      if (width >= 1200) setScreenSize('xl');\n      else if (width >= 992) setScreenSize('lg');\n      else if (width >= 768) setScreenSize('md');\n      else if (width >= 576) setScreenSize('sm');\n      else setScreenSize('xs');\n    };\n    \n    handleResize();\n    window.addEventListener('resize', handleResize);\n    return () => window.removeEventListener('resize', handleResize);\n  }, []);\n  \n  return { screenSize, isMobile: screenSize === 'xs' || screenSize === 'sm' };\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-component-documentation
/// Enforces comprehensive component documentation and examples
#[derive(Clone)]
pub struct RequireComponentDocumentation;

impl RequireComponentDocumentation {
    pub const NAME: &'static str = "require-component-documentation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireComponentDocumentation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for exported components without JSDoc
        if code.contains("export") && code.contains("component") && !code.contains("/**") {
            diagnostics.push(create_component_documentation_diagnostic(
                0, 0,
                "Exported components should include JSDoc documentation with examples"
            ));
        }

        // Check for prop interfaces without documentation
        if code.contains("interface") && code.contains("Props") && !code.contains("@description") {
            diagnostics.push(create_component_documentation_diagnostic(
                0, 0,
                "Component prop interfaces should include @description annotations"
            ));
        }

        // Check for complex components without usage examples
        if code.contains("component") && code.contains("useState") && !code.contains("@example") {
            diagnostics.push(create_component_documentation_diagnostic(
                0, 0,
                "Stateful components should include usage examples in documentation"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireComponentDocumentation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Add component documentation: `/** @description A reusable button component @example <Button variant=\"primary\" onClick={handler}>Click me</Button> */`".to_string(),
            fix_code: Some("/**\n * A flexible Button component that supports multiple variants and sizes.\n * \n * @description The Button component is a foundational element of our design system,\n * providing consistent styling and behavior across the application.\n * \n * @example\n * ```tsx\n * // Primary button\n * <Button variant=\"primary\" size=\"md\" onClick={handleClick}>\n *   Save Changes\n * </Button>\n * \n * // Icon button\n * <Button variant=\"ghost\" size=\"sm\" icon={<PlusIcon />}>\n *   Add Item\n * </Button>\n * \n * // Loading state\n * <Button variant=\"primary\" loading disabled>\n *   Processing...\n * </Button>\n * ```\n */\ninterface ButtonProps {\n  /** @description The visual style variant of the button */\n  variant?: 'primary' | 'secondary' | 'ghost' | 'danger';\n  \n  /** @description The size of the button */\n  size?: 'sm' | 'md' | 'lg';\n  \n  /** @description Optional icon to display alongside text */\n  icon?: React.ReactNode;\n  \n  /** @description Whether the button is in a loading state */\n  loading?: boolean;\n  \n  /** @description Whether the button is disabled */\n  disabled?: boolean;\n  \n  /** @description Click event handler */\n  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;\n  \n  /** @description The content to display inside the button */\n  children: React.ReactNode;\n}\n\n/**\n * Button component implementation with comprehensive prop support\n */\nconst Button: React.FC<ButtonProps> = ({\n  variant = 'primary',\n  size = 'md',\n  icon,\n  loading = false,\n  disabled = false,\n  onClick,\n  children,\n  ...props\n}) => {\n  return (\n    <button\n      className={`btn btn-${variant} btn-${size}`}\n      disabled={disabled || loading}\n      onClick={onClick}\n      {...props}\n    >\n      {loading && <Spinner size=\"sm\" />}\n      {icon && <span className=\"btn-icon\">{icon}</span>}\n      <span className=\"btn-text\">{children}</span>\n    </button>\n  );\n};\n\nButton.displayName = 'Button';\n\nexport { Button, type ButtonProps };".to_string()),
        }).collect()
    }
}

/// Rule: require-design-system-tokens
/// Enforces consistent usage of design system token naming conventions
#[derive(Clone)]
pub struct RequireDesignSystemTokens;

impl RequireDesignSystemTokens {
    pub const NAME: &'static str = "require-design-system-tokens";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDesignSystemTokens {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for inconsistent token naming
        if code.contains("color-") && code.contains("colour-") {
            diagnostics.push(create_design_system_tokens_diagnostic(
                0, 0,
                "Use consistent token naming convention (color vs colour)"
            ));
        }

        // Check for missing semantic token categories
        if code.contains("tokens") && !code.contains("semantic") && code.contains("colors") {
            diagnostics.push(create_design_system_tokens_diagnostic(
                0, 0,
                "Design tokens should include semantic color categories (success, warning, error)"
            ));
        }

        // Check for incomplete spacing scale
        if code.contains("spacing") && !code.contains("xs") && code.contains("sm") {
            diagnostics.push(create_design_system_tokens_diagnostic(
                0, 0,
                "Spacing tokens should include complete scale from xs to xxl"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDesignSystemTokens {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Implement comprehensive token system: Include primitive tokens (colors.blue.500), semantic tokens (colors.primary.main), and component tokens (button.background.primary)".to_string(),
            fix_code: Some("// Comprehensive design token system\nconst designTokens = {\n  // Primitive tokens - raw values\n  primitive: {\n    colors: {\n      blue: {\n        50: '#f0f9ff',\n        100: '#e0f2fe',\n        200: '#bae6fd',\n        300: '#7dd3fc',\n        400: '#38bdf8',\n        500: '#0ea5e9',\n        600: '#0284c7',\n        700: '#0369a1',\n        800: '#075985',\n        900: '#0c4a6e'\n      },\n      gray: {\n        50: '#f9fafb',\n        100: '#f3f4f6',\n        500: '#6b7280',\n        900: '#111827'\n      }\n    },\n    spacing: {\n      0: '0px',\n      1: '0.25rem',\n      2: '0.5rem',\n      3: '0.75rem',\n      4: '1rem',\n      5: '1.25rem',\n      6: '1.5rem',\n      8: '2rem',\n      10: '2.5rem',\n      12: '3rem'\n    },\n    borderRadius: {\n      none: '0px',\n      sm: '0.125rem',\n      md: '0.375rem',\n      lg: '0.5rem',\n      full: '9999px'\n    }\n  },\n  \n  // Semantic tokens - purpose-based\n  semantic: {\n    colors: {\n      primary: {\n        main: '{primitive.colors.blue.500}',\n        light: '{primitive.colors.blue.100}',\n        dark: '{primitive.colors.blue.700}'\n      },\n      success: {\n        main: '#10b981',\n        light: '#d1fae5',\n        dark: '#047857'\n      },\n      warning: {\n        main: '#f59e0b',\n        light: '#fef3c7',\n        dark: '#d97706'\n      },\n      error: {\n        main: '#ef4444',\n        light: '#fee2e2',\n        dark: '#dc2626'\n      },\n      text: {\n        primary: '{primitive.colors.gray.900}',\n        secondary: '{primitive.colors.gray.500}',\n        inverse: '{primitive.colors.gray.50}'\n      },\n      background: {\n        primary: '{primitive.colors.gray.50}',\n        secondary: '{primitive.colors.gray.100}',\n        inverse: '{primitive.colors.gray.900}'\n      }\n    }\n  },\n  \n  // Component tokens - component-specific\n  component: {\n    button: {\n      primary: {\n        background: '{semantic.colors.primary.main}',\n        text: '{semantic.colors.text.inverse}',\n        border: 'transparent',\n        borderRadius: '{primitive.borderRadius.md}',\n        padding: '{primitive.spacing.3} {primitive.spacing.6}'\n      },\n      secondary: {\n        background: 'transparent',\n        text: '{semantic.colors.primary.main}',\n        border: '{semantic.colors.primary.main}',\n        borderRadius: '{primitive.borderRadius.md}',\n        padding: '{primitive.spacing.3} {primitive.spacing.6}'\n      }\n    },\n    card: {\n      background: '{semantic.colors.background.primary}',\n      border: '{primitive.colors.gray.200}',\n      borderRadius: '{primitive.borderRadius.lg}',\n      padding: '{primitive.spacing.6}',\n      shadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)'\n    }\n  }\n};".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_design_tokens_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDesignTokens::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_component_composition_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireComponentComposition::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_accessibility_props_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAccessibilityProps::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_theme_support_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireThemeSupport::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_responsive_design_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireResponsiveDesign::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_component_documentation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireComponentDocumentation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_design_system_tokens_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDesignSystemTokens::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_design_tokens_violation() {
        let code = r#"
        const Button = styled.button`
          background-color: #007bff;
          padding: 16px;
          font-family: 'Helvetica Neue';
        `;
        "#;

        let rule = RequireDesignTokens;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 3); // color, padding, font-family
        assert!(diagnostics.iter().any(|d| d.message.contains("design tokens")));
    }

    #[test]
    fn test_require_design_tokens_compliant() {
        let code = r#"
        const Button = styled.button`
          background-color: ${tokens.colors.primary.main};
          padding: ${tokens.spacing.md};
          font-family: ${tokens.typography.fontFamily.body};
        `;
        "#;

        let rule = RequireDesignTokens;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_accessibility_props_violation() {
        let code = r#"
        const MyButton = () => (
          <button onClick={handleClick}>
            Click me
          </button>
        );
        "#;

        let rule = RequireAccessibilityProps;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("aria-label"));
    }

    #[test]
    fn test_require_accessibility_props_compliant() {
        let code = r#"
        const MyButton = () => (
          <button
            onClick={handleClick}
            aria-label="Submit form"
            aria-describedby="form-help"
          >
            Click me
          </button>
        );
        "#;

        let rule = RequireAccessibilityProps;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_theme_support_violation() {
        let code = r#"
        const Card = styled.div`
          background-color: #ffffff;
          border: 1px solid #e0e0e0;
        `;
        "#;

        let rule = RequireThemeSupport;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("theme"));
    }

    #[test]
    fn test_ai_enhancement_design_tokens() {
        let rule = RequireDesignTokens;
        let diagnostics = vec![create_design_tokens_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("tokens"));
    }

    #[test]
    fn test_ai_enhancement_accessibility_props() {
        let rule = RequireAccessibilityProps;
        let diagnostics = vec![create_accessibility_props_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("aria-label"));
    }
}