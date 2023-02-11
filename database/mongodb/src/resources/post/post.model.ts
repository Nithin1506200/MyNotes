import { Schema, model } from "mongoose";
import Post from "./post.interface.js";
import { Document } from "mongoose";
const PostSchema = new Schema<Post>(
  {
    title: {
      type: String,
      require: true,
    },
    body: {
      type: String,
      require: true,
    },
    email_id: {
      type: String,
    },
  },
  { timestamps: true }
);
export default model<Post>("Post", PostSchema);
