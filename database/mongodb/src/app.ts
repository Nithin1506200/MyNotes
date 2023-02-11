import express, { Application } from "express";
import mongoose from "mongoose";
import cors from "cors";
import morgan from "morgan";
import helmet from "helmet";
import compression from "compression";
import Controller from "@/utils/interfaces/controller.interfaces.js";
import ErrorMiddleware from "@/middleware/error.middleware.js";
import x from "@/utils/ts.js";
class App {
  public express: Application;
  public port: number;
  constructor(controllers: Controller[], port: number) {
    this.express = express();
    this.port = port;
    this.initialiseDatabaseConction();
    this.initialiseMiddleWare();
    this.initializeControllers(controllers);
    this.initializeErrorHandling();
  }

  private initializeErrorHandling() {
    this.express.use(ErrorMiddleware);
  }
  private initializeControllers(controllers: Controller[]) {
    controllers.forEach((controller: Controller) => {
      this.express.use("./api", controller.router);
    });
  }
  private initialiseMiddleWare() {
    this.express.use(helmet());
    this.express.use(cors());
    this.express.use(morgan("dev"));
    this.express.use(express.json());
    this.express.use(express.urlencoded({ extended: false }));
    this.express.use(compression());
  }
  private initialiseDatabaseConction() {
    const { MONGO_USER, MONGO_PASSWORD, MONGO_PATH } = process.env;
  }
  public listen(): void {
    this.express.listen(this.port, () => {
      console.log("App listening on port ", this.port);
    });
  }
}
export default App;
