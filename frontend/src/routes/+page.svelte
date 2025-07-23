<script>
    import Feed from "../components/Feed.svelte";
    import Login from "../components/Login.svelte";
    import Layout from "../layouts/Layout.svelte";
    import { getAppState } from "../state.svelte";

    const isLoggedIn = $derived(getAppState().token !== null);
</script>

<Layout>
    <h1 class="title">tempeh!</h1>
    {#if isLoggedIn}
        <Feed />
    {:else}
        <Login />
    {/if}
</Layout>

<style>
    .title {
        font-size: 2rem;
        font-weight: bold;
        margin: 16px;
    }
</style>
