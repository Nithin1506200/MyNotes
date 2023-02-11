import { Request, Response, NextFunction } from "express";

import HttpException from "@/utils/exceptions/http.exception.js";

function handleJwtError(error: Error, req: Request, res: Response) {
  res.send(400).json({
    message: "given jwt is not valid",
  });
}

function errorMiddleware(
  error: HttpException,
  req: Request,
  res: Response,
  next: NextFunction
) {
  const status: number = error.status || 500;
  const message = error.message || "something went wrong";
  if (error.name === "jwterror") {
    handleJwtError(error, req, res);
  }

  res.status(status).send({
    status,
    message,
  });
}
export default errorMiddleware;
