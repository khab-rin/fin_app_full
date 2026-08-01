<script lang='ts'>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

    import type {MchdStep} from "$lib/models/rustModels/MchdStep";

    function closeMchd() {
        const next_step: MchdStep = {Loading: {text: ""}};
        currentMchdStep.add(next_step);
        pageManager.Page = null;
    }

    function handleGoBack() {
        currentMchdStep.back(); 
    }

    function handleGoNext() {
        currentMchdStep.next(); 
    }
</script>


<p class="text-small">{currentMchdStep.currentText}</p>
{#if currentMchdStep.getPage}
    <svelte:component this={currentMchdStep.getPage} />
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
        onclick={closeMchd}
        >
        <span class="main-button-span">
            Основной экран
        </span>
    </button>
</div>