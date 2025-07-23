mod utils;

use crate::utils::{sse::sse_handler, token::generate_token};
use axum::{
    Router,
    extract::{Json, State},
    http::{Method, StatusCode},
    response::Json as ResponseJson,
    routing::{get, post},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct CreatePost {
    username: String,
    token: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserResponse {
    username: String,
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostsResponse {
    posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: String,
    username: String,
    content: String,
    created: String,
}

type UsersState = Arc<RwLock<HashMap<String, User>>>;
type PostsState = Arc<RwLock<HashMap<String, Post>>>;

#[derive(Clone)]
struct AppState {
    users: UsersState,
    posts: PostsState,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let users: HashMap<String, User> = HashMap::new();
    let shared_users = Arc::new(RwLock::new(users));

    let posts: HashMap<String, Post> = HashMap::new();
    let shared_posts = Arc::new(RwLock::new(posts));

    let (tx, _rx) = broadcast::channel(100);

    let app_state = AppState {
        users: shared_users,
        posts: shared_posts,
        tx,
    };

    let cors = CorsLayer::new()
        // Allow all origins
        .allow_origin(Any)
        // Allow specific methods
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // Allow specific headers
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/login", post(login))
        .route("/create_post", post(create_post))
        .route("/get_posts", get(get_posts))
        .route("/event_posts", get(sse_handler))
        .with_state(app_state)
        .layer(cors);

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    println!("POST to http://127.0.0.1:3000/users");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world(State(state): State<AppState>) -> &'static str {
    {
        let users = state.users.read().await;

        for (k, _v) in users.iter() {
            println!("#{}", k);
        }
    }

    "Hello, World!"
}

// POST endpoint handler
// @todo check if passwords match
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, ResponseJson<UserResponse>), StatusCode> {
    if payload.username.trim().is_empty() || payload.password.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let username = payload.username.trim().to_string();
    let password = payload.password.trim().to_string();

    let user = User {
        username: username.clone(),
        password: password.clone(),
        token: generate_token(64),
    };

    {
        let mut users = state.users.write().await;

        if users.contains_key(&username) {
            let user = users.get(&username).unwrap();

            if user.password.clone() == password {
                users.remove(&username);
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }

        println!("Adding new user");

        users.insert(username, user.clone());
    }

    Ok((
        StatusCode::CREATED,
        ResponseJson(UserResponse {
            username: user.username,
            token: user.token,
        }),
    ))
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<(StatusCode, ResponseJson<Post>), StatusCode> {
    if payload.token.trim().is_empty() || payload.content.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    {
        let users = state.users.read().await;

        if users.get(&payload.username).is_some() {
            let user = users.get(&payload.username).unwrap();
            if user.token != payload.token {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    let now_utc = Utc::now();

    let post = Post {
        id: generate_token(64),
        username: payload.username,
        content: payload.content,
        created: now_utc.to_rfc3339(),
    };

    {
        let mut posts = state.posts.write().await;
        println!("Adding new post");
        posts.insert(post.id.clone(), post.clone());
        let serialized_post = serde_json::to_string(&post).unwrap_or_default();
        let _ = state.tx.send(serialized_post);
    }

    Ok((StatusCode::CREATED, ResponseJson(post)))
}

async fn get_posts(
    State(state): State<AppState>,
) -> Result<(StatusCode, ResponseJson<PostsResponse>), StatusCode> {
    let posts = state.posts.read().await;

    let posts_response = PostsResponse {
        posts: posts.values().cloned().collect(),
    };

    Ok((StatusCode::OK, ResponseJson(posts_response)))
}
