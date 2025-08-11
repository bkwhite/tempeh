<script lang="ts">
    interface Props {
        username: string;
        content: string;
        created: string;
    }

    const { username, content, created }: Props = $props();

    // format the date like 1h, 1m ago
    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        const now = new Date();
        const diff = now.getTime() - date.getTime();
        const minutes = Math.floor(diff / (1000 * 60));
        const hours = Math.floor(minutes / 60);
        const days = Math.floor(hours / 24);
        const months = Math.floor(days / 30);
        const years = Math.floor(months / 12);

        if (years > 0) return `${years}y`;
        if (months > 0) return `${months}mo`;
        if (days > 0) return `${days}d`;
        if (hours > 0) return `${hours}h`;
        if (minutes > 0) return `${minutes}m`;
        return "now";
    }

    function wrapLinksWithA(text: string): string {
        return text.replace(
            /(https?:\/\/[^\s]+)/g,
            '<a href="$1" target="_blank">$1</a>',
        );
    }

    let createdLabel = formatDate(created);

    // update date every minute
    setInterval(() => {
        $: createdLabel = formatDate(created);
    }, 60000);
</script>

<div class="post">
    <h3>@{username} - {createdLabel}</h3>
    <p>{@html wrapLinksWithA(content)}</p>
</div>

<style>
    .post {
        display: flex;
        flex-direction: column;
        gap: 8px;

        border: 1px solid #ccc;
        padding: 16px;
        border-radius: 8px;
    }
</style>
