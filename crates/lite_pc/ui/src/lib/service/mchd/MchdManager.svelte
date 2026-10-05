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


<div class="w-100">
	<h4 class='w-g100 x-self-c x-ce t-fill'>
		{currentMchdStep.currentText}
	</h4>
</div>



{#if currentMchdStep.getPage}
    <svelte:component this={currentMchdStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<section class="w-40">

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

