<script lang="ts">
    import VirtualList from "svelte-tiny-virtual-list";

    import Write from "./Write.svelte";
    import Post from "./Post.svelte";
    import { getPosts } from "$lib";
    import type { Post as PostType } from "../lib/types";

    let posts: PostType[] = $state([]);

    $effect(() => {
        getPosts().then((data) => {
            posts = data;
        });
    });

    $effect(() => {
        const eventSource = new EventSource(
            "http://127.0.0.1:3000/event_posts",
        );

        eventSource.onmessage = (event) => {
            const data = JSON.parse(event.data);
            posts.push(data);
        };
    });
</script>

<div id="feed" class="feed">
    <Write />
    <h2>feed</h2>

    <VirtualList
        width="100%"
        height="100%"
        itemCount={posts.length}
        itemSize={100}
    >
        <div slot="item" let:index let:style {style}>
            <Post
                username={posts[index].username}
                content={posts[index].content}
                created={posts[index].created}
            />
        </div>
    </VirtualList>
</div>

<style>
    .feed {
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 600px;
        gap: 16px;
    }

    .posts {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }
</style>
