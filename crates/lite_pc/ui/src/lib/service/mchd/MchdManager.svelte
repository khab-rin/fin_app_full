<script lang='ts'>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

    import type {MchdStep} from "$lib/models/rustModels/MchdStep";

	function goToLoading() {
		const nextStep: MchdStep = {Loading: {text: ""}};
        currentMchdStep.step = nextStep;
	}

    function closeMchd() {
        const nextStep: MchdStep = {Loading: {text: ""}};
        currentMchdStep.step = nextStep;
        pageManager.Page = null;
    }
</script>


<p class="text-small">{currentMchdStep.currentText}</p>
{#if currentMchdStep.getPage}
    <svelte:component this={currentMchdStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<section class="group-two">

	<button
		type="button"
		class="blue-button"
		onclick={goToLoading}
		>
		<span class="blue-button-span">
			Меню доверенностей
		</span>
	</button>

	<button
		type="button"
		class="blue-button"
		onclick={closeMchd}
		>
		<span class="blue-button-span">
			Основной экран
		</span>
	</button>

</section>

