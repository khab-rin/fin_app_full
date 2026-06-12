<script>
    import { logOut } from "$lib/service/auth_service/LogOut";
    
    let { show = $bindable() } = $props();
    let dialog = $state();

    $effect(() => {
        if (show) dialog?.showModal();
        else dialog?.close();
    });
</script>

<dialog bind:this={dialog} class="google-dialog" onclose={() => show = false}>
    <div class="dialog-header">
        <h3>Настройки</h3>
    </div>

    <div class="account-list">
        <button class="account-item-btn" onclick={() => alert("Тут будут настройки")}>
            <span>⚙️ Личные настройки</span>
        </button>
        <button class="account-item-btn" onclick={logOut}>
            <span style="color: red;">🚪 Выйти</span>
        </button>
    </div>

    <div class="dialog-footer">
        <button class="btn-close-modal" onclick={() => show = false}>Закрыть</button>
    </div>
</dialog>