<script lang="ts">
    let isExpanded = $state(false);

    let selectedTax = $state('Упрощенка 6%');
    let selectedYear = $state('2024');
    let selectedPeriod = $state('Весь год');

    const years = ['2023', '2024', '2025'];
    const periods = ['1 квартал', '2 квартал', '3 квартал', 'Весь год'];

    function toggleExpand() {
        isExpanded = !isExpanded;
    }
</script>

<div class="tax-page">
    <header class="tax-selector">
        <button 
            class="tax-type" 
            class:active={selectedTax === 'Упрощенка 6%'}
            onclick={() => selectedTax = 'Упрощенка 6%'}
        >
            Упрощенка 6%
        </button>
        <button 
            class="tax-type" 
            class:active={selectedTax === 'Упрощенка 15%'}
            onclick={() => selectedTax = 'Упрощенка 15%'}
        >
            Упрощенка 15%
        </button>
    </header>

    <div class="report-card">
        <div class="report-header" onclick={toggleExpand} role="button" tabindex="0">
            <div class="report-info">
                <span class="report-title">Налоговый отчет</span>
                <span class="report-subtitle">{selectedYear}, {selectedPeriod}</span>
            </div>
            <span class="arrow" class:open={isExpanded}>▼</span>
        </div>

        <!-- Выпадающая панель (показывается по условию) -->
        {#if isExpanded}
            <div class="report-settings">
                <div class="setting-group">
                    <label>Год</label>
                    <div class="options">
                        {#each years as year (year)}
                            <button 
                                class="opt-button" 
                                class:selected={selectedYear === year}
                                onclick={() => selectedYear = year}
                            >
                                {year}
                            </button>
                        {/each}
                    </div>
                </div>

                <div class="setting-group">
                    <label>Период</label>
                    <div class="options">
                        {#each periods as period (period)}
                            <button 
                                class="opt-button" 
                                class:selected={selectedPeriod === period}
                                onclick={() => selectedPeriod = period}
                            >
                                {period}
                            </button>
                        {/each}
                    </div>
                </div>

                <button class="generate-button">Сформировать отчет</button>
            </div>
        {/if}
    </div>
</div>

<style>
    .tax-page {
        padding: 15px;
        font-family: sans-serif;
    }

    .tax-selector {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
        border-bottom: 1px solid #eee;
        padding-bottom: 10px;
    }

    .tax-type {
        background: none;
        border: none;
        padding: 8px 12px;
        cursor: pointer;
        color: #888;
        font-weight: 500;
        transition: color 0.2s;
    }

    .tax-type.active {
        color: #007aff;
        border-bottom: 2px solid #007aff;
    }

    /* Карточка отчета */
    .report-card {
        background: white;
        border-radius: 12px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.08);
        overflow: hidden;
    }

    .report-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 15px 20px;
        cursor: pointer;
        background: #fff;
    }

    .report-info {
        display: flex;
        flex-direction: column;
    }

    .report-title {
        font-weight: bold;
        font-size: 1.1rem;
    }

    .report-subtitle {
        font-size: 0.85rem;
        color: #666;
    }

    .arrow {
        transition: transform 0.3s;
        color: #ccc;
    }

    .arrow.open {
        transform: rotate(180deg);
    }

    /* Настройки внутри выпадающего списка */
    .report-settings {
        padding: 20px;
        background: #fafafa;
        border-top: 1px solid #eee;
    }

    .setting-group {
        margin-bottom: 15px;
    }

    .setting-group label {
        display: block;
        font-size: 0.8rem;
        color: #888;
        margin-bottom: 8px;
        text-transform: uppercase;
    }

    .options {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .opt-button {
        padding: 6px 12px;
        border: 1px solid #ddd;
        background: white;
        border-radius: 20px;
        font-size: 0.9rem;
        cursor: pointer;
    }

    .opt-button.selected {
        background: #007aff;
        color: white;
        border-color: #007aff;
    }

    .generate-button {
        width: 100%;
        padding: 12px;
        background: #28a745;
        color: white;
        border: none;
        border-radius: 8px;
        font-weight: bold;
        margin-top: 10px;
        cursor: pointer;
    }
</style>