<script lang="ts">
    import { login } from "../state.svelte";

    type UserResponse = {
        username: string;
        token: string;
    };

    let username = $state("");
    let password = $state("");

    const onLogin = () => {
        fetch("http://127.0.0.1:3000/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                password,
            }),
        })
            .then((response) => response.json())
            .then((data: UserResponse) => {
                login(data.username, data.token);
            });
    };
</script>

<div id="login" class="login">
    <form class="login-form">
        <div class="form-group">
            <label for="username">username</label>
            <input
                type="text"
                id="username"
                name="username"
                bind:value={username}
                required
            />
        </div>

        <div class="form-group">
            <label for="password">password</label>
            <input
                type="password"
                id="password"
                name="password"
                bind:value={password}
                required
            />
        </div>

        <button type="button" id="login-button" onclick={onLogin}>login</button>
    </form>
</div>

<style>
    .login {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
    }

    .login-form {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;

        width: 300px;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
</style>
