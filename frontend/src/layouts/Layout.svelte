<div class="app">
    <slot />
</div>

<style>
    :global {
        html,
        body {
            margin: 0;
            width: 100%;
            height: 100%;
            font-family: Arial, Helvetica, sans-serif;
        }

        button {
            background-color: black;
            color: white;
            padding: 8px 16px;
            border: none;
            border-radius: 8px;
            cursor: pointer;
        }

        h1,
        h2,
        h3,
        h4,
        h5,
        p {
            margin: 0;
            padding: 0;
        }

        .app {
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100vh;
            padding-bottom: 32px;
        }
    }
</style>
