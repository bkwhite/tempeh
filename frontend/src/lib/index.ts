import type { Post } from "./types";

type GetPostsResponse = {
  posts: Post[];
};

export async function getPosts(): Promise<Post[]> {
  const response = await fetch("http://127.0.0.1:3000/get_posts");
  const data: GetPostsResponse = await response.json();
  return data.posts;
}
