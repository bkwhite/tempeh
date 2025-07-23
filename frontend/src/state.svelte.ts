import type { AppState, Post } from "$lib/types";

let appState = $state<AppState>({ username: "", token: null });
let postsState = $state<Post[]>([]);

export function getAppState() {
  return appState;
}

export function login(username: string, token: string) {
  appState = { username, token };
}

export function fetchPosts() {
  postsState = [];
}
