```sh
yarn add -D typescript ts-node-dev @types/node tsconfig-paths
npx tsc --init
```

# package

```json
{
  "dev": "tsnd --respawn --transpile-only -r tsconfig-paths/register src/index.ts",
  "build": "tsc"
}
```

# tsconfig

```json
{
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
    "outDir": "./dist"
    "baseUrl": "./" /* Specify the base directory to resolve non-relative module names. */,
  "paths": {
    "@src/*": ["src/*"]
  },
  "typeRoots": [
      "./env.d.ts",
      "./node_modules/@types"
    ]
}
```
