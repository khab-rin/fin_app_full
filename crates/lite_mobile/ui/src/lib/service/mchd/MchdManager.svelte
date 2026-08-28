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


<section class="group-two">
    <div>
        <button
            class="purple-button"
            type="button"
            onclick={handleGoBack}
		>
            <span class="purple-button-span">Назад</span>
        </button>
    </div>

    <div>
        <button
            class="purple-button"
            type="button"
            onclick={handleGoNext}>
            <span class="purple-button-span">Вперед</span>
        </button>
    </div>
    
</section>

<div class="blue-button-group">
    <button
        type="button"
        class="blue-button"
        onclick={closeMchd}
        >
        <span class="blue-button-span">
            Основной экран
        </span>
    </button>
</div>