import express, { NextFunction, Request, Response, response } from "express";
import * as dotenv from "dotenv";
dotenv.config();
// init the app
const app = express();
//middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app
  .route("/ping")
  .get((req: Request, res: Response) => {
    res.send("pong get");
  })
  .post((req: Request, res: Response) => {
    res.send("pong post");
  })
  .put((req: Request, res: Response) => {
    res.send("pong put");
  })
  .delete((req: Request, res: Response) => {
    res.send("pong delete");
  })
  .patch((req: Request, res: Response) => {
    res.send("pong patch");
  });

// custom middle ware example

const middleware =
  (name: string = "nithin") =>
  (req: Request, res: Response, next: NextFunction) => {
    res.locals.customname = name;
    return next();
  };
app.get("/middleware", middleware("dcd"), (req: Request, res: Response) => {
  return res.send({
    inputname: req.query.inputname,
    middleware__name: res.locals.customname,
  });
});
// listen to port

app.listen(process.env.port || 8000, () => {
  console.log(
    "------------------- app is listning",
    process.env.port,
    "----------------"
  );
});
