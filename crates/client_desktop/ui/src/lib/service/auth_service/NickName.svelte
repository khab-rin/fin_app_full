<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type { AuthStep } from '$lib/models/AuthStep';
    import type {TextInfo} from "$lib/models/TextInfo";
	import AuthManager from "./AuthManager.svelte";

    let inputName = $state(currAuthStep.nick_names.nick_names[0] || '');
    let IsPushed = $state(false);

    let filteredNames = $derived(
        currAuthStep.nick_names.nick_names.filter(name => 
            name.toLowerCase().startsWith(inputName.toLowerCase())
        )
    );

    function handleBeforeInput(e: InputEvent) {
        const char = e.data || "";
        const target = e.target as HTMLInputElement;
        const start = target.selectionStart ?? 0;
        const end = target.selectionEnd ?? 0;

        const nextChar = target.value.slice(0, start) + char + target.value.slice(end);
        
        const isPossible = currAuthStep.nick_names.nick_names.some(name => 
            name.toLowerCase().startsWith(nextChar.toLowerCase())
        );

        if (!isPossible && char !== "") {
            e.preventDefault();
        }
    }

    async function call_nick_handle() {
        if (inputName.trim() === '' || IsPushed) return;

        IsPushed = true;
        
        try {
            let next_step = await invoke<AuthStep>('cmd_session_by_nick', { nickname: inputName });
            currAuthStep.add(next_step);
        } catch (err) {
            let next_step: AuthStep = { 
                TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"} 
            };
            console.error("tech_err =", err);
            IsPushed = false; 
            currAuthStep.add(next_step);
        }
    }

    function goToPassword() {
        let next_step: AuthStep = currAuthStep.step = { 
            NeedPassword: { text: "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию" } 
        };
    }

    function goToRegistration() {
        currAuthStep.step = { 
            NeedRegistration: { text: "Пользователь не найден, требуется пройти регистрацию" } 
        };
    }
</script>

<div>
    <p class="info-text">
        {currAuthStep.currentText}
    </p>

    <div class="drop-down-list">
        <label for="drop-down">Выберите пользователя</label>
        <input
            type="text"
            id="drop-down"
            name="NickName"
            placeholder="Начните вводить пользователя"
            autocomplete="on"
            spellcheck="false"
            disabled={IsPushed}
            bind:value={inputName}
            onbeforeinput={handleBeforeInput}
            class="input-text-field"
        />

        {#if !IsPushed && inputName.length > 0 && filteredNames.length > 0}
            <ul class="suggestions">
                {#each filteredNames as name (name)}
                    <li>
                        <button 
                            type="button" 
                            class="suggestion-item"
                            onclick={() => {
                                inputName = name;
                                call_nick_handle();
                            }}
                        >
                            {name}
                        </button>
                    </li>
                {/each}
            </ul>
        {/if}
    </div>

    <button 
        type="button" 
        class="btn-submit" 
        disabled={IsPushed || inputName.trim() === ''} 
        onclick={call_nick_handle}
    >
        {#if IsPushed}
            <span class="spinner"></span>
            <span>Проверка...</span>
        {:else}
            <span>Войти</span>
        {/if}
    </button>

    <div class="auth-navigation-links">
        <button 
            type="button" 
            class="btn-link" 
            disabled={IsPushed} 
            onclick={goToPassword}
        >
            Вход по паролю
        </button>
        
        <span class="divider">|</span>

        <button 
            type="button" 
            class="btn-link" 
            disabled={IsPushed} 
            onclick={goToRegistration}
        >
            Зарегистрироваться
        </button>
    </div>
</div>

<style>
    .btn-submit {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        width: 100%;
        padding: 0.75rem;
        background-color: #007aff;
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        margin-top: 1rem;
        transition: background-color 0.2s;
    }

    .btn-submit:disabled {
        background-color: #b3d7ff;
        cursor: not-allowed;
    }

    .spinner {
        width: 18px;
        height: 18px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-radius: 50%;
        border-top-color: white;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }

    /* Стили для ссылок перехода */
    .auth-navigation-links {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.75rem;
        margin-top: 1.5rem;
        font-size: 0.9rem;
    }

    .btn-link {
        background: none;
        border: none;
        color: #007aff;
        cursor: pointer;
        padding: 0;
        font-size: inherit;
        font-family: inherit;
    }

    .btn-link:hover:not(:disabled) {
        text-decoration: underline;
    }

    .btn-link:disabled {
        color: #aaa;
        cursor: not-allowed;
    }

    .divider {
        color: #ccc;
        user-select: none;
    }
</style>