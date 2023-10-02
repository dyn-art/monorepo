# `@dyn/config`

## Debugging ESLint Configuration

If you are encountering issues or unexpected behavior with ESLint, you can use the following command to output the final configuration ESLint is using for a specific file. This can be helpful for debugging configuration issues.
```bash
npx eslint --print-config ./some/file/to/test/on.ts
```

## 🔴 Issues

### TypeScript Configurations Location

TypeScript configurations are placed at the root to allow easy referencing from other packages in the monorepo using the `extends` field in `tsconfig.json`, like so:

```json
{
  "extends": "@dyn/config/base.json"
}
```

This setup bypasses limitations in TypeScript's module resolution mechanism when using the `extends` field.