<script lang='ts'>
    import {onMount} from 'svelte';
    import {currAuthStep} from '$lib/models/Auth/AuthStep.svelte';
    import {AuthStepType} from '$lib/models/Auth/AuthValues';

    let sur_name = "";
    let first_name = "";
    let mid_name = "";
    let pers_inn = "";
    let snils = "";
    let comp_inn = "";
    let kpp = "";
    let phone = "";
    let email = "";

    onMount(async() => {
        if (AuthStepType.RegisterStep1Duplicate in currAuthStep.step) {
            const data = currAuthStep.step.RegisterStep1Duplicate;
            sur_name = data.sur_name;
            first_name = data.first_name;
            mid_name = data.mid_name;
            pers_inn = data.pers_inn;
            snils = data.snils;
            comp_inn = data.comp_inn;
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
            <dd>{sur_name || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Имя</dt>
            <dd>{first_name || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>Отчество</dt>
            <dd>{mid_name || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>ИНН физлица</dt>
            <dd>{pers_inn || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>СНИЛС</dt>
            <dd>{snils || '—'}</dd>
        </div>

        <div class="info-row">
            <dt>ИНН организации</dt>
            <dd>{comp_inn || '—'}</dd>
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