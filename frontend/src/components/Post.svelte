<script lang="ts">
    interface Props {
        username: string;
        content: string;
        created: string;
    }

    const { username, content, created }: Props = $props();
</script>

<div class="post">
    <h3>@{username} - 1h</h3>
    <p>{content}</p>
</div>

<style>
    .post {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
</style>
