# `@dyn/ui`

## TODO

### Resolve `exports` Configuration for Remix Compatibility

Currently experiencing issues with the Remix framework's module resolution system when using the `exports` field in `package.json`. Need to determine an effective configuration that allows for direct importing of assets (such as CSS files) using shorter paths.

Current `exports` Configuration:

```json
  "exports": {
    ".": {
      "import": "./dist/esm/index.js",
      "require": "./dist/cjs/index.js",
      "types": "./dist/types/index.d.ts"
    },
    "./styles.css": "./dist/styles.css",
    "./tailwind.css": "./dist/tailwind.css",
    "./with-dyn-ui": "./with-dyn-ui.js"
  },
  "sideEffects": ["*.css"]
```

### Desired Improvement:

- Enable direct import of styles using syntax like `import '@dyn/ui/styles.css'`, avoiding the need to specify the `dist` directory in the import path.
- Investigate how Remix handles the `exports` field and identify why the current configuration leads to conflicts.
