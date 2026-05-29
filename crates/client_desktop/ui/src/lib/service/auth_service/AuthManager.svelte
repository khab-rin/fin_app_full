<script>
    import { currAuthStep } from "./SvelteAuthStep.svelte";
    import CallIn from '$lib/service/auth_service/CallIn.svelte';
    import NickName from '$lib/service/auth_service/NikName.svelte';
    import PassWord from '$lib/service/auth_service/PassWord.svelte';
    import Registration from '$lib/service/auth_service/Registration.svelte'

</script>

{#if 'Init' in currAuthStep.step }
    <NickName/>
{:else if 'NeedPassword' in currAuthStep.step }
    <PassWord/>
{:else if 'CallIn' in currAuthStep.step }
    <CallIn/>
{:else if 'NeedRegistrtion' in currAuthStep.step }
    <Registration/>
{:else}
    <p>
        На данный момент авторизация невозможна, попробуйте позже пожалуйста....
    </p>
{/if}

