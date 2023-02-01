- [Expressjs](#expressjs)
  - [setup](#setup)
    - [init node](#init-node)
    - [typescript](#typescript)
    - [express js](#express-js)
    - [dotenv](#dotenv)
  - [express app int](#express-app-int)
  - [requrests](#requrests)

# Expressjs

## setup

- Nodejs
- Typescript
- expressjs

### init node

```sh
yarn init

```

```sh
npx tsc --init
```

```sh
mkdir src
touch src/app.ts
```

### typescript

```sh
yarn add typescript -D
yarn tsc --init
yarn add ts-node-dev -D
```

```json
{
    scripts : {
        dev : "ts-node-dev --respawn --transpile-only src/app.ts
    }
}
```

### express js

```sh
yarn add express
yarn add @types/node @types/express
```

### dotenv

```sh
yarn add dotenv
touch .env
```

## express app int

## requrests

```javascript
app.route("/").get((req: Request, res: Response) => {
  res.send("true");
}).post;
```
