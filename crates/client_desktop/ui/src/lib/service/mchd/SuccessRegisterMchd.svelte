<script lang='ts'>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {MchdStepType} from "$lib/models/Mchd/MchdValues";

    let guide = (MchdStepType.SuccessRegisterMchd in currentMchdStep.step) ? currentMchdStep.step.SuccessRegisterMchd.guid : "";
</script>


<p>{guide}</p>