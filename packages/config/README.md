# `@dyn/config`

## 🔴 Issues

### TypeScript Configurations Location

TypeScript configurations are placed at the root to allow easy referencing from other packages in the monorepo using the `extends` field in `tsconfig.json`, like so:

```json
{
  "extends": "@dyn/config/base.json"
}
```

This setup bypasses limitations in TypeScript's module resolution mechanism when using the `extends` field.