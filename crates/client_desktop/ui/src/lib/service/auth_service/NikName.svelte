<script lang='ts'>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "./SvelteAuthStep.svelte";

    import type {NickData} from '$lib/models/NickData';
    import type {AuthStep} from '$lib/models/AuthStep'

    let inputName = $state('');
    let savedNames = $state<string[]>([]);
    let isLoading = $state(true);

    onMount(async() => {
        try {
            const data = await invoke<NickData>('cmd_get_nick_names');
            if (!data.nick_names || data.nick_names.length == 0) {
                isLoading = false;
                currAuthStep.step = { NeedPassword: {} }
            } else {
                savedNames = data.nick_names;
                isLoading = false;
                inputName = savedNames[0];
            }
        } catch (e) {
            currAuthStep.step = { TryLater: {status: "SystemErr"} };
            console.error("Error:", e);
            isLoading = false;
        }
    });

    let filteredNames = $derived(
        savedNames.filter(name => 
            name.toLowerCase().startsWith(inputName.toLowerCase())
        )
    );

    function handleBeforeInput(e: InputEvent) {

        const char = e.data || "";
        const target = e.target as HTMLInputElement;

        // Используем оператор ?? (nullish coalescing)
        // Если selectionStart равен null, подставится 0
        const start = target.selectionStart ?? 0;
        const end = target.selectionEnd ?? 0;

        const nextChar = target.value.slice(0, start) + 
            char +
            target.value.slice(end);
        
        const isPossible = savedNames.some(name => 
            name.toLowerCase().startsWith(nextChar.toLowerCase())
        );

        if (!isPossible && char !== "") {
            e.preventDefault();
        }
    }

    async function call_nick_handle() {
        if (inputName.trim() == '') return;
        try {
            currAuthStep.step = await invoke<AuthStep>('cmd_auth_restore_nick', {nickname: inputName});
        } catch (err) {
            currAuthStep.step = { TryLater: {status: "SystemErr"}}
            console.error(console.error("tech_err =", err, "local_err =", "SystemErr"));
        }
    }
</script>

<div>
    <h2>
        Вход в систему
    </h2>
    {#if isLoading}
        <p>Загрузка данных</p>
    {:else}
        <div class="drop-down-list">
            <label for="drop-down">Выберите пользователя</label>
            <input
                type="text"
                id="drop-down"
                name="NickName"
                placeholder="Начните вводить пользователя"
                autocomplete="on"
                spellcheck="false"
                bind:value={inputName}
                onbeforeinput={handleBeforeInput}
                class="input-text-field"
            />

            {#if inputName.length > 0 && filteredNames.length > 0}
                <ul class="suggestions">
                    <!-- Добавляем (name) как ключ для производительности Svelte -->
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
    {/if}
</div>