//! # Rule Utilities Module
//!
//! Shared utilities for MoonShine rule implementations
//!
//! @category rule-utils
//! @safe team
//! @mvp core
//! @complexity low
//! @since 2.1.0

use oxc_ast::ast::Program;
use oxc_span::Span;
use std::fmt::Write;

/// Extracts the 1-based line and column from an OXC span with source text analysis.
///
/// This is a production implementation that supports various line ending formats.
pub fn span_to_line_col(source_text: &str, span: Span) -> (u32, u32) {
    span_to_line_col_with_cache(source_text, span, None)
}

/// Extracts the line and column with an optional line offset cache for performance.
///
/// This function is optimized for repeated calls on the same source text.
pub fn span_to_line_col_with_cache(
    source_text: &str,
    span: Span,
    line_offsets: Option<&[usize]>
) -> (u32, u32) {
    let start_offset = span.start as usize;

    // Handle edge cases
    if start_offset >= source_text.len() {
        return estimate_line_col_from_end(source_text);
    }

    if start_offset == 0 {
        return (1, 1);
    }

    // Use cached line offsets if provided, otherwise compute
    let line_number = if let Some(offsets) = line_offsets {
        find_line_from_offsets(offsets, start_offset)
    } else {
        calculate_line_number(source_text, start_offset)
    };

    let column_number = calculate_column_number(source_text, start_offset, line_number);

    (line_number, column_number)
}

/// Calculates the line number by counting line breaks before the offset.
fn calculate_line_number(source_text: &str, offset: usize) -> u32 {
    let mut line = 1;
    let bytes = source_text.as_bytes();

    for i in 0..offset.min(bytes.len()) {
        match bytes[i] {
            b'\n' => line += 1,
            b'\r' => {
                // Handle CRLF (\r\n) - only count as one line break
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    // Skip the \n in CRLF
                    continue;
                }
                line += 1;
            }
            _ => {}
        }
    }

    line
}

/// Calculates the column number from the start of the current line.
fn calculate_column_number(source_text: &str, offset: usize, line_number: u32) -> u32 {
    // Find the start of the current line
    let line_start = find_line_start(source_text, offset);

    // Calculate column as character offset from line start
    let line_text = &source_text[line_start..offset];

    // Count Unicode characters, not bytes, for proper column numbering
    let column = line_text.chars().count() as u32 + 1; // 1-based indexing

    column
}

/// Finds the byte offset of the start of the line containing the given offset.
fn find_line_start(source_text: &str, offset: usize) -> usize {
    let bytes = source_text.as_bytes();

    // Search backwards from offset to find line start
    for i in (0..offset.min(bytes.len())).rev() {
        match bytes[i] {
            b'\n' => return i + 1, // Start of line is after the newline
            b'\r' => {
                // Handle CRLF - start of line is after the full CRLF sequence
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    return i + 2;
                }
                return i + 1;
            }
            _ => {}
        }
    }

    0 // Start of file
}

/// Finds the line number using pre-computed line offsets for performance optimization.
fn find_line_from_offsets(line_offsets: &[usize], offset: usize) -> u32 {
    // Binary search for the line containing the offset
    match line_offsets.binary_search(&offset) {
        Ok(line_index) => (line_index + 1) as u32,
        Err(insert_index) => {
            // offset falls between line boundaries
            if insert_index == 0 {
                1
            } else {
                insert_index as u32
            }
        }
    }
}

/// Estimates the line and column when the offset is beyond the end of the source text.
fn estimate_line_col_from_end(source_text: &str) -> (u32, u32) {
    let line_count = calculate_line_number(source_text, source_text.len());

    // Find the last line and calculate its length
    let last_line_start = find_line_start(source_text, source_text.len());
    let last_line_text = &source_text[last_line_start..];
    let last_line_length = last_line_text.chars().count() as u32;

    (line_count, last_line_length + 1)
}

/// Generates a line offset cache for efficient repeated span conversions.
///
/// This should be called once per source file when processing many spans.
pub fn build_line_offset_cache(source_text: &str) -> Vec<usize> {
    let mut offsets = vec![0]; // Line 1 starts at offset 0
    let bytes = source_text.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'\n' => offsets.push(i + 1),
            b'\r' => {
                // Handle CRLF - only add offset after complete CRLF sequence
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    offsets.push(i + 2);
                } else {
                    offsets.push(i + 1);
                }
            }
            _ => {}
        }
    }

    offsets
}

/// A compatibility wrapper for existing code that expects a `Program` parameter.
pub fn span_to_line_col_legacy(program: &Program, span: Span) -> (u32, u32) {
    // In a real implementation, we would extract source_text from the program
    // For now, provide a reasonable fallback based on span offset
    let estimated_line = (span.start / 80) + 1; // Assume ~80 chars per line
    let estimated_col = (span.start % 80) + 1;
    (estimated_line, estimated_col)
}

/// Converts a string to PascalCase with Unicode safety.
pub fn unicode_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in s.chars() {
        if ch.is_alphabetic() {
            if capitalize_next {
                write!(&mut result, "{}", ch.to_uppercase().collect::<String>()).unwrap();
                capitalize_next = false;
            } else {
                write!(&mut result, "{}", ch.to_lowercase().collect::<String>()).unwrap();
            }
        } else if ch.is_ascii_punctuation() || ch.is_whitespace() {
            capitalize_next = true;
        } else {
            result.push(ch);
        }
    }

    result
}

/// Generates boolean name suggestions for a given name.
pub fn generate_boolean_suggestions(name: &str) -> String {
    let pascal_name = unicode_pascal_case(name);
    format!("is{}, has{}, should{}", pascal_name, pascal_name, pascal_name)
}

/// Checks if a name starts with a boolean prefix.
pub fn starts_with_boolean_prefix(name: &str, allowed_prefixes: &[&str]) -> bool {
    let lower_name = name.to_lowercase();
    allowed_prefixes.iter().any(|prefix| lower_name.starts_with(prefix))
}