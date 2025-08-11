<script lang="ts">
    import Write from "./Write.svelte";
    import Post from "./Post.svelte";
    import { getPosts, SERVER_URL } from "$lib";
    import type { Post as PostType } from "../lib/types";

    let posts: PostType[] = $state([]);

    // Load initial posts
    $effect(() => {
        getPosts().then((data) => {
            // @ts-ignore
            posts = data.sort((a, b) => b.created - a.created);
        });
    });

    // Set up Server-Sent Events for real-time updates
    $effect(() => {
        const eventSource = new EventSource(`${SERVER_URL}/event_posts`);

        eventSource.onmessage = (event) => {
            const newPost = JSON.parse(event.data);
            // Create new sorted array with the new post
            posts = [newPost, ...posts];
        };

        eventSource.onerror = (error) => {
            console.error("EventSource failed:", error);
        };

        // Cleanup function to close the EventSource when component unmounts
        return () => {
            eventSource.close();
        };
    });
</script>

<div id="feed" class="feed">
    <Write />
    <h2>feed</h2>
    <div class="posts">
        {#each posts as post (post.id || `${post.username}-${post.created}`)}
            <Post
                username={post.username}
                content={post.content}
                created={post.created}
            />
        {/each}
    </div>
</div>

<style>
    .feed {
        display: flex;
        flex-direction: column;
        width: 600px;
        gap: 16px;
    }

    .posts {
        display: flex;
        flex-direction: column;
        gap: 16px;
        overflow-y: auto;
        max-height: 70vh; /* Prevent infinite growth */
        padding-bottom: 32px;
    }

    h2 {
        margin: 0;
        color: #333;
        font-size: 1.5rem;
        font-weight: 600;
    }
</style>
