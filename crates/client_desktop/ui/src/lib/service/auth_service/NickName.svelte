<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type { AuthStep } from '$lib/models/AuthStep';

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
        closeAccountsModal(); // Закрываем окно сразу при выборе
        currAuthStep.data.nick.value = selectedNick;

        console.log("ОТПРАВЛЯЕМ НИК НА БЭКЕНД:", selectedNick);
        
        try {
            let next_step = await invoke<AuthStep>('cmd_session_by_nick', { nick: selectedNick });
            console.log("БЭКЕНД УСПЕШНО ВЕРНУЛ ШАГ:", next_step);
            
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

    function goToPassword() {
        let next_step: AuthStep = { NeedPassword: { text: "" } };
        currAuthStep.add(next_step);
    }

    function handleGoBack() {
        currAuthStep.back(); 
    }

    function handleGoNext() {
        currAuthStep.next(); 
    }
</script>

<div class="auth-card">
    <div class="nick-name">
        <p class="info-text">
            {currAuthStep.currentText}
        </p>

        <div class="account-selector-wrapper">
            <button
                type="button"
                disabled={IsPushed}
                class="google-style-trigger"
                onclick={openAccountsModal}
            >
                <div class="user-avatar-stub">
                    {currAuthStep.data.nick.value ? currAuthStep.data.nick.value.charAt(0).toUpperCase() : "?"}
                </div>
                <div class="user-info-stub">
                    <span class="username-label">
                        {currAuthStep.data.nick.value || "Выбрать аккаунт на устройстве"}
                    </span>
                    <span class="sub-label">
                        {currAuthStep.data.nick.value ? "Текущий выбор" : "Нажмите для просмотра списка"}
                    </span>
                </div>
                <span class="arrow-icon">❯</span>
            </button>
        </div>

        <dialog 
            bind:this={dialogRef} 
            class="google-dialog"
            onclick={(e) => { if (e.target === dialogRef) closeAccountsModal(); }}
        >
            <div class="dialog-header">
                <h3>Выбор аккаунта</h3>
                <p>Выберите профиль для продолжения работы с XPinAT</p>
            </div>

            <div class="dialog-content">
                {#if currAuthStep.nick_names.nick_names.length > 0}
                    <ul class="account-list">
                        {#each currAuthStep.nick_names.nick_names as name (name)}
                            <li>
                                <button 
                                    type="button" 
                                    class="account-item-btn"
                                    onclick={() => call_nick_handle(name)}
                                >
                                    <div class="avatar-circle">{name.charAt(0).toUpperCase()}</div>
                                    <span class="account-name">{name}</span>
                                </button>
                            </li>
                        {/each}
                    </ul>
                {:else}
                    <p class="no-accounts">На этом устройстве еще нет сохраненных аккаунтов</p>
                {/if}
            </div>

            <div class="dialog-footer">
                <button type="button" class="btn-close-modal" onclick={closeAccountsModal}>Отмена</button>
            </div>
        </dialog>

        {#if currAuthStep.data.nick.value}
            <button 
                type="button" 
                class="btn-submit" 
                disabled={IsPushed}
                onclick={() => call_nick_handle(currAuthStep.data.nick.value)}
            >
                {#if IsPushed}
                    <span class="spinner"></span>
                    <span>Проверка...</span>
                {:else}
                    <span>Войти как {currAuthStep.data.nick.value}</span>
                {/if}
            </button>
        {/if}
    </div>

    <section class="navi-buttons">
        <div class="buttons-grid-row">
            <button type="button" onclick={handleGoBack} class="nav-btn-item">
                <span class="nav-btn-text">Назад</span>
            </button>

            <button type="button" onclick={goToPassword} class="nav-btn-item">
                <span class="nav-btn-text">Вход по паролю</span>
            </button>

            <button type="button" onclick={handleGoNext} class="nav-btn-item">
                <span class="nav-btn-text">Вперед</span>
            </button>
        </div>
    </section>
</div>