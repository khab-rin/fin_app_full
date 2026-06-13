<script lang="ts">
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";

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
    <p class="auth-text-error">
        {currAuthStep.currentText}
    </p>

    <section class="navi-buttons">
        <div class="buttons-grid-row">
            <button type="button" onclick={handleGoBack} class="nav-btn-item">
                <span class="nav-btn-text">Назад</span>
            </button>

            <button type="button" onclick={handleGoToRegister} class="nav-btn-item">
                <span class="nav-btn-text">Регистрация</span>
            </button>

            <button type="button" onclick={handleGoNext} class="nav-btn-item">
                <span class="nav-btn-text">Вперед</span>
            </button>
        </div>

    </section>

</div>