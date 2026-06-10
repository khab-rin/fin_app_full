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

    function goToRegistration() {
        let next_step: AuthStep  = {NeedRegistration: { text: "" } };
        currAuthStep.add(next_step);
    }
</script>

<div class="auth-container">
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

    <div class="auth-navigation-links">
        <button type="button" class="btn-link" disabled={IsPushed} onclick={goToPassword}>
            Вход по паролю
        </button>
        <span class="divider">|</span>
        <button type="button" class="btn-link" disabled={IsPushed} onclick={goToRegistration}>
            Зарегистрироваться
        </button>
    </div>
</div>

<style>
    /* Стили главного триггера а-ля Google Аккаунты */
    .google-style-trigger {
        display: flex;
        align-items: center;
        width: 100%;
        padding: 0.75rem 1rem;
        background-color: #fff;
        border: 1px solid #dadce0;
        border-radius: 8px;
        cursor: pointer;
        text-align: left;
        transition: background-color 0.2s, border-color 0.2s;
    }

    .google-style-trigger:hover:not(:disabled) {
        background-color: #f8f9fa;
        border-color: #d2e3fc;
    }

    .user-avatar-stub {
        width: 40px;
        height: 40px;
        background-color: #e8f0fe;
        color: #1a73e8;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        font-size: 1.2rem;
        margin-right: 1rem;
    }

    .user-info-stub {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
    }

    .username-label {
        font-size: 1rem;
        font-weight: 500;
        color: #3c4043;
    }

    .sub-label {
        font-size: 0.8rem;
        color: #70757a;
    }

    .arrow-icon {
        color: #70757a;
        font-size: 0.9rem;
    }

    /* Стили нативного модального окна <dialog> */
    .google-dialog {
        border: none;
        border-radius: 8px;
        padding: 1.5rem;
        width: 90%;
        max-width: 400px;
        box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
        background: white;
    }

    /* Эффект размытия/затемнения заднего фона модалки */
    .google-dialog::backdrop {
        background-color: rgba(32, 33, 36, 0.6);
        backdrop-filter: blur(2px);
    }

    .dialog-header h3 {
        margin: 0 0 0.25rem 0;
        font-size: 1.4rem;
        font-weight: 500;
        color: #202124;
        text-align: center;
    }

    .dialog-header p {
        margin: 0 0 1.5rem 0;
        font-size: 0.9rem;
        color: #5f6368;
        text-align: center;
    }

    .account-list {
        list-style: none;
        padding: 0;
        margin: 0;
        max-height: 250px;
        overflow-y: auto;
        border-top: 1px solid #dadce0;
        border-bottom: 1px solid #dadce0;
    }

    .account-item-btn {
        display: flex;
        align-items: center;
        width: 100%;
        padding: 0.75rem 0.5rem;
        background: none;
        border: none;
        cursor: pointer;
        text-align: left;
        transition: background-color 0.15s;
    }

    .account-item-btn:hover {
        background-color: #f8f9fa;
    }

    .avatar-circle {
        width: 32px;
        height: 32px;
        background-color: #f1f3f4;
        color: #5f6368;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: 500;
        margin-right: 0.75rem;
    }

    .account-name {
        font-size: 1rem;
        font-weight: 400;
        color: #3c4043;
    }

    .dialog-footer {
        display: flex;
        justify-content: flex-end;
        margin-top: 1rem;
    }

    .btn-close-modal {
        background: none;
        border: 1px solid #dadce0;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        cursor: pointer;
        color: #5f6368;
        font-weight: 500;
    }

    .btn-close-modal:hover {
        background-color: #f8f9fa;
        color: #202124;
    }

    .no-accounts {
        text-align: center;
        color: #70757a;
        padding: 1rem 0;
    }

    /* Базовые кнопки и ссылки */
    .btn-submit {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        width: 100%;
        padding: 0.75rem;
        background-color: #1a73e8;
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        margin-top: 1rem;
    }
    .btn-submit:disabled { background-color: #b3d7ff; cursor: not-allowed; }
    .spinner { width: 18px; height: 18px; border: 2px solid rgba(255, 255, 255, 0.3); border-radius: 50%; border-top-color: white; animation: spin 0.8s linear infinite; }
    @keyframes spin { to { transform: rotate(360deg); } }
    .auth-navigation-links { display: flex; justify-content: center; align-items: center; gap: 0.75rem; margin-top: 1.5rem; font-size: 0.9rem; }
    .btn-link { background: none; border: none; color: #1a73e8; cursor: pointer; padding: 0; font-size: inherit; }
    .btn-link:hover:not(:disabled) { text-decoration: underline; }
    .btn-link:disabled { color: #aaa; }
    .divider { color: #ccc; user-select: none; }
</style>