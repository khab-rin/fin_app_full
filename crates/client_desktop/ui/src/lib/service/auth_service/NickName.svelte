<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type { AuthStep } from '$lib/models/rustModels/AuthStep';

    let IsPushed = $state(false);
    let dialogRef = $state<HTMLDialogElement | null>(null);

    // Функция открытия модального окна
    function openAccountsModal() {
        if (dialogRef) dialogRef.showModal();
    }

    // Функция закрытия модального окна
    function closeAccountsModal() {
        if (dialogRef) dialogRef.close();
    }

    async function call_nick_handle(selectedNick: string) {
        if (IsPushed) return;
        
        IsPushed = true;
        closeAccountsModal();
        
        try {
            let next_step = await invoke<AuthStep>('cmd_session_by_nick', { nick: selectedNick });
            IsPushed = false;
            currAuthStep.add(next_step);
        } catch (err) {
            let next_step: AuthStep = { 
                TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"} 
            };
            console.error("ОШИБКА В call_nick_handle:", err);
            IsPushed = false; 
            currAuthStep.add(next_step);
        }
    }

</script>



<div class="selector-wrapper">
    <button
        type="button"
        class="wide-button"
        disabled={IsPushed}
        
        onclick={openAccountsModal}
    >

        <span class="wide-button-span">
             - {currAuthStep.data.nick.value ? "Текущий выбор" : "Нажмите для просмотра списка"}
        </span>

        <span class="wide-button-span">
            {currAuthStep.data.nick.value || "Выбрать аккаунт на устройстве"} >
        </span>
    </button>
</div>


<dialog 
    bind:this={dialogRef} 
    class="selector-dialog"
    onclick={(e) => { if (e.target === dialogRef) closeAccountsModal(); }}
>

    <h5>Выбор аккаунта</h5>


    <div class="selector-dialog-content">
        {#if currAuthStep.nick_names.length > 0}
            <ul class="dialog-list">
                {#each currAuthStep.nick_names as name (name)}
                    <li>
                        <div class="wide-button-grid">
                            <button 
                                type="button" 
                                class="wide-button"
                                onclick={() => {
                                    currAuthStep.data.nick.value = name;
                                    closeAccountsModal();}    
                                }
                            >
                                <span class="wide-button-span">{name}</span>
                            </button>
                        </div>
                    </li>
                {/each}
            </ul>

            <div class="wide-button-grid">
                <button class="wide-button"
                    type="button"
                    onclick={closeAccountsModal}
                    >
                    
                    <span class="wide-button-span">
                        Отмена
                    </span>


                </button>
            </div>

        {:else}
            <p>На этом устройстве еще нет сохраненных аккаунтов</p>
        {/if}
    </div>

</dialog>

{#if currAuthStep.data.nick.value}
    <div class="main-button-group">
        <button 
            type="button" 
            class="main-button" 
            disabled={IsPushed}
            onclick={() => call_nick_handle(currAuthStep.data.nick.value)}
        >
            {#if IsPushed}
                <span class="main-button-span"></span>
                <span class="main-button-span">Проверка...</span>
            {:else}
                <span class="main-button-span">Войти как {currAuthStep.data.nick.value}</span>
            {/if}
        </button>
    </div>
{/if}

