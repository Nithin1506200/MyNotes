import App from "app.js";
import "dotenv/config";
import PostController from "resources/post/post.controller.js";
// import validateEnv from "@/utils/validateEnv.js";
const app = new App([new PostController()], Number(process.env.PORT));
app.listen();
