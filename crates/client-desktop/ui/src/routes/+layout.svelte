<script lang="ts">
	import { page } from '$app/state';
    import '$lib/style/global.css'
    import favicon from '$lib/assets/favicon.svg';

    let { children } = $props();
	import { goTo } from '$lib/rules/navigation';
</script>


<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>

<div class="app-container">
    <header class="top-bar">
		<div class="user-avatar">
		<span class="avatar-icon">👤</span>
		</div>
		<input type="text" class="search-input" placeholder="Поиск...">
		<button class="param-btn">⚙️</button>
	</header>

	<section class="user-identity">
		<h1>ИП Иванов Иван Иванович</h1>
	</section>

    <main class="main-content">
        {@render children()}
    </main>

	<footer class="bottom-nav">

		<button
			class="nav-item"
			class:active={page.url.pathname === '/' }
			onclick={() => goTo('/')}
		>
			<span class="nav-icon">🏠</span>
			<span class="nav-label">Главная</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">📊</span>
			<span class="nav-label">Статистика</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">📅</span>
			<span class="nav-label">Календарь</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">❓</span>
			<span class="nav-label">Справка</span>
		</button>
	</footer>



</div>


