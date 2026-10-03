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

<div class='group-5'>
	<p class="text-base text-c text-fill text-box">{currentMchdStep.currentText}</p>
</div>




{#if currentMchdStep.getPage}
    <svelte:component this={currentMchdStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<section class="group-2">

	<button
		type="button"
		class="but-bl"
		onclick={goToLoading}
		>
		<span class="span-fill">
			Меню доверенностей
		</span>
	</button>
</section>

