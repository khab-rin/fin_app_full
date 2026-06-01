<script>
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import Loading from "$lib/service/auth_service/Loading.svelte";
    import CallIn from '$lib/service/auth_service/CallIn.svelte';
    import NickName from '$lib/service/auth_service/NickName.svelte';
    import PassWord from '$lib/service/auth_service/PassWord.svelte';
    import Registration from '$lib/service/auth_service/Registration.svelte'

</script>

{#if "Loading" in currAuthStep.step}
    <Loading/>
{:else if 'NickName' in currAuthStep.step }
    <NickName/>
{:else if 'NeedPassword' in currAuthStep.step }
    <PassWord/>
{:else if 'CallIn' in currAuthStep.step }
    <CallIn/>
{:else if 'NeedRegistration' in currAuthStep.step }
    <Registration/>
{:else}
    <p>
        На данный момент авторизация невозможна, попробуйте позже пожалуйста....
    </p>
{/if}

