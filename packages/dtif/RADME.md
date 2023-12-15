# `@dyn/dtif`
`@dyn/dtif` offers a TypeScript implementation of the Design Tree Interchange Format (DTIF), distinct from its Rust counterpart. 

## Why separation
1. **Slightly Different Implementations**: Rust and TypeScript DTIFs differ due to language/environment-specific capabilities. TypeScript handles URLs for resources like fonts and images to minimize size and avoid data duplication, unlike the Rust version (in WASM environment) where web requests are limited.
2. Efficiency in Continous Integration: The standalone TypeScript DTIF avoids the need to rebuild the entire `@dyn/svg-composition` just to access the types.
3. Lightweight and Specialized: The package focuses solely on DTIF structures, making it ideal for the use of exporting designs to DTIF without the extra weight of the full `@dyn/svg-composition` package.