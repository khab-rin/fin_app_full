<script>
    import { logOut } from "$lib/models/Auth/LogOut";
    import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
    import { PageType } from "$lib/models/MainManager/PageValues";

    
    async function press_logout() {
        pageManager.settingsOnOff = false;
        await logOut();
    }

    function gotoMchd() {
        pageManager.settingsOnOff = false;
        pageManager.Page = PageType.Mchd
    }

</script>

<div class="popover">
    <div class="popover-header">
        <h3 class="popover-header h3">Настройки</h3>
    </div>

    <div class="popover-list">
        <button 
            class="popover-button" 
            onclick={() => alert("Тут будут настройки")}
            disabled={pageManager.totalOff}>
            <span class="popover-span">⚙️ Личные настройки</span>
        </button>

        <button 
            class="popover-button"
            disabled = {pageManager.totalOff}
            onclick={gotoMchd}>
            <span class="popover-span">МЧД ДОВЕРЕННОСТИ</span>
        </button>

        <button class="popover-batton" onclick={press_logout}>
            <span class="popover-span">🚪 Выйти</span>
        </button>
    </div>
</div>