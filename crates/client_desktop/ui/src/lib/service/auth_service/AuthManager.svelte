<script>
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import {AuthStepType} from "$lib/service/auth_service/AuthValues";
    import Loading from "$lib/service/auth_service/Loading.svelte";
    import CallIn from '$lib/service/auth_service/CallIn.svelte';
    import NickName from '$lib/service/auth_service/NickName.svelte';
    import PassWord from '$lib/service/auth_service/PassWord.svelte';
    import Registration from '$lib/service/auth_service/Registration.svelte';
    import TryLater from "./TryLater.svelte";

</script>

{#if AuthStepType.Loading in currAuthStep.step}
    <Loading/>
{:else if AuthStepType.NickName in currAuthStep.step }
    <NickName/>
{:else if AuthStepType.NeedPassword in currAuthStep.step }
    <PassWord/>
{:else if AuthStepType.CallIn in currAuthStep.step }
    <CallIn/>
{:else if AuthStepType.NeedRegistration in currAuthStep.step }
    <Registration/>
{:else}
    <TryLater/>
{/if}

