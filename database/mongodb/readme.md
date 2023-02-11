- [link](#link)
- [init](#init)
- [tsconfig](#tsconfig)
- [package.json](#packagejson)

## link

<https://www.youtube.com/watch?v=1o9YOHeKhNQ>

## init

```sh
yarn init
yarn add typescript ts-node-dev -D
yarn add dotenv express joi cors compression morgan helmet
yarn add @types/node @types/express -D
yarn add @types/cors @types/morgan @types/compression -D
```

## tsconfig

```sh
yarn tsc --init
```

```json
    "baseUrl":"./src",
    "out":"dist",
    "paths": {
      "@/resources/*": ["resourses/*"],
      "@/utils/*": ["utils/*"],
      "@/middleware/*": ["middleware/*"]
    }
```

## package.json

```json
"type":"module",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1",
    "dev": "ts-node-dev --respawn --transpile-only src/app.ts",
     "build": "yarn tsc"
  },
```
