<script>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
</script>


<p class="text-small">{currentMchdStep.currentText}</p>
{#if currentMchdStep.getPage}
    <svelte:component this={currentMchdStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}