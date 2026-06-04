<script lang="ts">
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type {AuthStep} from "$lib/models/AuthStep";

    function handleGoToRegister() {
        let next_step: AuthStep = {NeedRegistration: {text: ""}};
        currAuthStep.add(next_step);
    }

    function handleGoToNickName() {
        let next_step: AuthStep = {NickName: {text: ""}};
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
    <p>
        {currAuthStep.currentText}
    </p>
    <section class="grid-section">
        <div class="quick-links">
        

            <button 
                type="button" 
                onclick={handleGoToRegister} 
                class="grid-item"
                id="auth-register-btn"
            >
                <span class="btn-icon">📝</span>
                <span class="btn-label">Регистрация</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoToNickName} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Выбрать пользователя</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoBack} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Назад</span>
            </button>

            <button 
                type="button" 
                onclick={handleGoNext} 
                class="grid-item"
                id="auth-select-user-btn"
            >
                <span class="btn-icon">👤</span>
                <span class="btn-label">Вперед</span>
            </button>

        </div>
    </section>

</div>