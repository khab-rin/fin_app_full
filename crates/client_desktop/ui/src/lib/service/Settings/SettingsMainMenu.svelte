<script>
    import { logOut } from "$lib/models/Auth/LogOut";
    import {appState} from "$lib/models/AppState/appState.svelte";
    
    async function press_logout() {
        appState.settingsOnOff = false;
        await logOut();
    }
</script>

<div class="popover">
    <div class="popover-header">
        <h3 class="popover-header h3">Настройки</h3>
    </div>

    <div class="popover-list">
        <button class="popover-button" onclick={() => alert("Тут будут настройки")}>
            <span class="popover-span">⚙️ Личные настройки</span>
        </button>

        <button class="popover-batton" onclick={press_logout}>
            <span class="popover-span">🚪 Выйти</span>
        </button>
    </div>
</div>