# RUSTOML

A lightweight, memory-efficient TOML parser written in Rust that focuses on strict schema field enforcement and low overhead through an optimized abstract syntax tree structure. By wrapping internal field type definitions inside an algebraic data type layout, the parser maximizes space utilization and isolates field-specific criteria completely at the variant level.

## Features
- __Integers__: Full support for standard decimal integers, visual underscore separators, and explicit radix notations including binary, octal, and hexadecimal forms.

- __Floats & Floating-Point Specials__: Complete compliance for conventional floats, scientific notation exponents, and exact matching for lowercase special values like inf, +inf, -inf, nan, +nan, and -nan.

- __Strings__: Full support for basic strings, literal strings, and multi-line variants.

- __Structural Tables__: Standard tables and deeply nested tables initialized by individual brackets.

- __Arrays of Tables__: Nested collections of structural tables generated sequentially by double bracket declarations.

- __Inline Tables__: Inline structural scopes bounded within brace characters.

- __Encapsulated Data API__: Typesafe conversion functions that retrieve native primitives and arrays at specific positions directly from table handles while completely hiding internal structural enums.

- __Test Suites__: Target test suites covering string lexing operations, multi-radix parsing, and grammar validation blocks.

## Architecture
The workflow moves sequentially from a raw byte stream into a structured, validated configuration map:

1. __Lexical Analysis__
   The Lexer iterates through individual bytes to output clean, sequential tokens. Numeric evaluation checks for initial sign indicators and filters visual underscores entirely at the buffer layer before hitting radix parsing logic. Multi-character boundaries are guarded by explicit delimiter checks to prevent token pollution.
2. __Syntactic Analysis__
The Parser reads context-free tokens to compile an internal configuration hashmap. It resolves bracket ambiguities based on structural position:

   - Double open brackets at the document root trigger dedicated array table header handling logic.

   - A single open bracket at the start of a line initializes a static table header path, whereas an open bracket on the right side of an assignment sign initiates a regular recursive array block.

   - Open brace indicators instantly trigger single-line inline table evaluations.
3. __Data Extraction and Encapsulation__
   The internal mapping nodes are completely sealed behind a robust data boundary. Users query values exclusively through the parent table handle using contextual key and indexing getters. Type mismatches, out-of-bounds array operations, and missing configuration criteria trigger isolated runtime errors that automatically drop syntax-heavy file location parameters in favor of clean path string contexts.
4. __Schema Field Validation__
The configuration validation block extracts field schema descriptors via isolated type parsers. By mapping metadata traits out of top-level structural definitions and isolating them into explicit type configurations, memory fragmentation is minimized across the parsing tree, allowing early validation failures if invalid field attributes are encountered.

## Improvements

- __Date and Time Integration__: Introduce specialized tokenization routines to capture standardized temporal data strings.

- __Array Schema Enhancements__: Scale structural validation capabilities to support heterogeneous arrays, element constraint verification, and tuple bounds.

- __Comprehensive Test Coverage__: Expand validation testing matrices across highly nested inline components and complex radix configurations.

- __Refactoring and Optimization__: Isolate functional components into structured module subtrees and wrap tokens inside consistent span layout structures.
---
Made by [NirbhikTheNice](https://github.com/NirbhikKumawat)