import type { Post } from "./types";

type GetPostsResponse = {
  posts: Post[];
};

export const SERVER_URL = "http://192.168.1.211:3000";

export async function getPosts(): Promise<Post[]> {
  const response = await fetch(`${SERVER_URL}/get_posts`);
  const data: GetPostsResponse = await response.json();
  return data.posts;
}
