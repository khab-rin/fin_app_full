<script lang='ts'>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {MchdStepType} from "$lib/models/Mchd/MchdValues";

    let guide = (MchdStepType.SuccessRegisterMchd in currentMchdStep.step) ? currentMchdStep.step.SuccessRegisterMchd.guide : "";
</script>


{#if guide}
    <div class="info-row">
        <dd>{guide || '—'}</dd>
    </div>
{/if}