import Joi from "joi";
import Post from "./post.interface.js";

const create = Joi.object<Post>({
  title: Joi.string().required(),
  body: Joi.string().required(),
  email_id: Joi.string().required(),
});
const update = Joi.object<Post>({
  title: Joi.string().required(),
  body: Joi.string().required(),
  email_id: Joi.string().required(),
});

export default { create, update };
