<script lang='ts'>
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import {AuthStepType} from "$lib/models/Auth/AuthValues";

    import type { AuthStep } from "$lib/models/rustModels/AuthStep";

    import Loading from "$lib/service/auth_service/Loading.svelte";
    import CallIn from '$lib/service/auth_service/CallIn.svelte';
    import NickName from '$lib/service/auth_service/NickName.svelte';
    import PassWord from '$lib/service/auth_service/PassWord.svelte';
    import Registration from '$lib/service/auth_service/Registration.svelte';
    import TryLater from "./TryLater.svelte";

    let isPassword = $derived(AuthStepType.NeedPassword in currAuthStep.step);
    let isRegistration = $derived(AuthStepType.NeedRegistration in currAuthStep.step);

    function goToPassword() {
        let next_step: AuthStep = { NeedPassword: { text: "" } };
        currAuthStep.add(next_step);
    }

    function goToRegistration() {
        let next_step: AuthStep = { NeedRegistration: { text: "" } };
        currAuthStep.add(next_step);
    }

    function handleGoBack() {
        currAuthStep.back(); 
    }

    function handleGoNext() {
        currAuthStep.next(); 
    }
</script>

<p class="text-small">
    {currAuthStep.currentText}
</p>

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


<section class="navi-button-section">

    <h6> Навигация по авторизации </h6>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoBack}
        >
            <span class="medium-button-span">
                Шаг назад
            </span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            hidden={isPassword}
            onclick={goToPassword}
        >
            <span class="medium-button-span">
                Вход по паролю
            </span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            hidden={isRegistration}
            onclick={goToRegistration}
        >
            <span class="medium-button-span">
                Регистрация
            </span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoNext}
        >
            <span class="medium-button-span">
                Шаг вперед
            </span>
        </button>
    </div>

</section>
