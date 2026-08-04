<script lang='ts'>
    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
	import { currAuthStep } from '$lib/models/Auth/AuthStep.svelte';

    function closeOper() {
        const next_step: OperationStep = {Loading: {text: "Выберите функционал работы с проводками"}};
        operStep.add(next_step);
        pageManager.Page = null;
    }

    function handleGoBack() {
        operStep.back();
    }

    function handleGoNext() {
        currAuthStep.next();
    }

</script>

<p class="text-small">{operStep.currentText}</p>

{#if operStep.getPage}
    <svelte:component this={operStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}

<section class="navi-button-section">
    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoBack}>
            <span class="navi-button-text">Назад</span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoNext}>
            <span class="navi-button-text">Вперед</span>
        </button>
    </div>
    
</section>

<div class="main-button-group">
    <button
        type="button"
        class="main-button"
        onclick={closeOper}
        >
        <span class="main-button-span">
            Основной экран
        </span>
    </button>
</div>