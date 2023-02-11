import postModel from "./post.model.js";

import Post from "./post.interface.js";

class PostService {
  private post = postModel;

  public async create(data: Post): Promise<Post> {
    try {
      const post = await this.post.create({
        title: data.title,
        body: data.body,
        email_id: data.email_id,
      });
      return post;
    } catch (error) {
      throw new Error("Unable to create post");
    }
  }
}
export default PostService;
