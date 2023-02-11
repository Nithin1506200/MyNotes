import { Document } from "mongoose";
export default interface Post {
  title: string;
  body: string;
  email_id: string;
}
