<script>
    import { SERVER_URL } from "$lib";
    import { getAppState } from "../state.svelte";

    let content = $state("");

    const onPost = () => {
        if (!content) {
            alert("Please enter some content.");
            return;
        }

        const { username, token } = getAppState();

        fetch(`${SERVER_URL}/create_post`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                token,
                content,
            }),
        })
            .then((response) => response.json())
            .then((data) => {
                console.log(data);
                content = "";
            });
    };
</script>

<div class="write">
    <h2>new post</h2>
    <textarea id="write-content" maxlength="140" rows="5" bind:value={content}
    ></textarea>
    <button id="write-button" onclick={onPost}>post</button>
</div>

<style>
    .write {
        display: flex;
        flex-direction: column;
        gap: 8px;

        width: 100%;
    }

    h2 {
        margin: 0;
    }

    button {
        align-self: flex-end;
    }
</style>
