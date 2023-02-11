import { Router, Request, Response, NextFunction } from "express";
import Controller from "@/utils/interfaces/controller.interfaces.js";
import HttpException from "@/utils/exceptions/http.exception.js";
import validationMiddleware from "@/middleware/validation.middleware.js";
import validate from "./post.validation.js";
import PostService from "./post.service.js";
import Post from "./post.interface.js";

class PostController implements Controller {
  public path = "/posts";
  public router = Router();
  private PostService = new PostService();
  constructor() {
    this.initialiseRoutes();
  }
  private initialiseRoutes() {
    this.router.post(
      this.path,
      validationMiddleware(validate.create),
      this.create
    );
  }
  private create = async (
    req: Request<{}, {}, Post>,
    res: Response,
    next: NextFunction
  ): Promise<Response | void> => {
    try {
      const { title, body, email_id } = req.body;
      const post = await this.PostService.create({ title, body, email_id });
      res.status(201).json({ post });
    } catch (error: any) {
      next(new HttpException(400, error.message || "Cannot create user"));
    }
  };
}
export default PostController;
