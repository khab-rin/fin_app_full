<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type { AuthStep } from '$lib/models/rustModels/AuthStep';
    import type {PasswordDataClientShort} from "$lib/models/rustModels/PasswordDataClientShort"

    let isPushed = $state(false);

    async function handleAuthSubmit() {
        if (isPushed) return;
       
        if (
            !currAuthStep.data.nick.isValid || 
            !currAuthStep.data.persInn.isValid || 
            !currAuthStep.data.compInn.isValid ||
            !currAuthStep.data.kpp.isValid ||
            !currAuthStep.data.password.isValid
        ) return;

        const sendData: PasswordDataClientShort = {
            nick: currAuthStep.data.nick.value,
            password: currAuthStep.data.password.value,
            pers_inn: currAuthStep.data.persInn.value,
            comp_inn: currAuthStep.data.compInn.value,
            kpp: currAuthStep.data.kpp.value
        };

        isPushed = true;

       
        try {
            let next_step = await invoke<AuthStep>('cmd_session_by_password', {
                data: sendData
            });
            isPushed = false;
            currAuthStep.add(next_step);

        } catch (err) {
            console.error("Критическая ошибка cmd_auth_with_password:", err);
            let next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushed = false;
            currAuthStep.add(next_step);
        }
    }

    function handleGoToRegister() {
        let next_step: AuthStep = {NeedRegistration: {text: ""}};
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
    <p class="auth-text-step">
        {currAuthStep.currentText}
    </p>

    <div class="form-group">
        <label for="nick">Имя пользователя</label>
        <input 
            id="nick" 
            type="text" 
            bind:value={currAuthStep.data.nick.value} 
            disabled={isPushed}
            placeholder="Введите ваш никнейм"
            class="input-field" 
            class:input-error={!currAuthStep.data.nick.isValid}/>
        
        {#if !currAuthStep.data.nick.isValid}
            <span class="error-message">Некорректный никнейм</span>
        {/if}
    </div>

    <div class="form-group">
        <label for="persInn">ИНН физического лица</label>
        <input 
            id="persInn" 
            type="text" 
            bind:value={currAuthStep.data.persInn.value} 
            disabled={isPushed}
            placeholder="12 цифр ИНН ФЛ"
            class="input-field"
            class:input-error={!currAuthStep.data.persInn.isValid}
        />
        {#if !currAuthStep.data.persInn.isValid}
            <span class="error-message">Некорректный инн физического лица</span>
        {/if}
    </div>

    <div class="form-group">
        <label for="compInn">ИНН организации</label>
        <input 
            id="innOrg" 
            type="text" 
            bind:value={currAuthStep.data.compInn.value}
            disabled={isPushed}
            placeholder="10 цифр ИНН ЮЛ"
            class="input-field"
            class:input-error={!currAuthStep.data.compInn.isValid}/>
        {#if !currAuthStep.data.compInn.isValid}
            <span class="error-message">Некорректный инн юридического лица</span>
        {/if}
    </div>

    <div class="form-group">
        <label for="kpp">КПП организации</label>
        <input 
            id="kppOrg" 
            type="text" 
            bind:value={currAuthStep.data.kpp.value}
            disabled={isPushed} 
            placeholder="9 знаков КПП"
            class="input-field"
            class:input-error={!currAuthStep.data.kpp.isValid}/>
        {#if !currAuthStep.data.kpp.isValid}
            <span class="error-message">Некорректный кпп</span>
        {/if}
    </div>

    <div class="form-group">
        <label for="password">Пароль</label>
        <input 
            id="password" 
            type="password" 
            bind:value={currAuthStep.data.password.value}
            disabled={isPushed} 
            placeholder="Введите пароль"
            class="input-field"
            class:input-error={!currAuthStep.data.password.isValid}/>
        {#if !currAuthStep.data.password.isValid}
            <span class="error-message">Пароль некоректен в рамках прилжоения</span>
        {/if}
    </div>

    <section class="navi-buttons">
        <button 
            type="button" 
            onclick={handleAuthSubmit}
            disabled={
                isPushed || 
                !currAuthStep.data.nick.isValid || 
                !currAuthStep.data.persInn.isValid || 
                !currAuthStep.data.compInn.isValid || 
                !currAuthStep.data.kpp.isValid || 
                !currAuthStep.data.password.isValid
            }
            class="main-button"
            id="auth-submit-btn"
        >
            <span class="navi-buttons.btn-icon">
                {#if isPushed}⏳{:else}🔑{/if}
            </span>
            <span class="btn-label">
                {#if isPushed}Вход...{:else}Отправить{/if}
            </span>
        </button>

        <div class="buttons-grid-row">
            <button type="button" onclick={handleGoBack} class="nav-btn-item">
                <span class="nav-btn-text">Назад</span>
            </button>

            <button type="button" onclick={handleGoToRegister} disabled={isPushed} class="nav-btn-item">
                <span class="nav-btn-text">Регистрация</span>
            </button>

            <button type="button" onclick={handleGoNext} class="nav-btn-item">
                <span class="nav-btn-text">Вперед</span>
            </button>
        </div>
    </section>
</div>
