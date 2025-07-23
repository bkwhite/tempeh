export type AppState = {
  username: string;
  token: string | null;
};

export type Post = {
  id: string;
  username: string;
  content: string;
  created: string;
};
