<script lang='ts'>
    import {onMount} from 'svelte';
    import {currAuthStep} from '$lib/models/Auth/AuthStep.svelte';
    import {AuthStepType} from '$lib/models/Auth/AuthValues';

    let surName = "";
    let firstName = "";
    let midName = "";
    let persInn = "";
    let snils = "";
    let compInn = "";
    let kpp = "";
    let phone = "";
    let email = "";

    onMount(async() => {
        if (AuthStepType.RegisterStep1Duplicate in currAuthStep.step) {
            const data = currAuthStep.step.RegisterStep1Duplicate;
            surName = data.sur_name;
            firstName = data.first_name;
            midName = data.mid_name;
            persInn = data.pers_inn;
            snils = data.snils;
            compInn = data.comp_inn;
            kpp = data.kpp;
            phone = data.phone;
            email = data.email;
             
        } else {
            console.error("asdf");
        }
    });

</script>

<div class="info-section">
    <h3>Данные дубликата</h3>
    
    <dl class="info-list">
        <div class="info-row">
            <dt>Фамилия</dt>
            <dd>{surName || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Имя</dt>
            <dd>{firstName || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Отчество</dt>
            <dd>{midName || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>ИНН физлица</dt>
            <dd>{persInn || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>СНИЛС</dt>
            <dd>{snils || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>ИНН организации</dt>
            <dd>{compInn || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>КПП</dt>
            <dd>{kpp || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Телефон</dt>
            <dd>{phone || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Email</dt>
            <dd>{email || '—'}</dd>
        </div>
    </dl>
</div>