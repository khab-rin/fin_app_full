<script lang="ts">
	import { page } from '$app/state';
    import '$lib/style/global.css'
    import favicon from '$lib/assets/favicon.svg';
	import { goTo } from '$lib/rules/navigation';
	import { onMount } from 'svelte';
	import { checkSessionInit } from '$lib/service/auth_service/check_state';
	import { currAuthStep } from '$lib/service/auth_service/SvelteAuthStep.svelte';
	import AuthManager from '$lib/service/auth_service/AuthManager.svelte';

	let { children } = $props<{ children: import('svelte').Snippet }>();


	onMount(async () => {
		if (await checkSessionInit()) {
			currAuthStep.step = { SuccessShort: {}} ;
		} else {
			currAuthStep.step = { Init: {} };
		}
		currAuthStep.isLoading = false;
	});


</script>


<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>


{#if currAuthStep.isLoading}
	<div class = "loading_screen">
		<p>Инициализация защищенного соединения...</p>
	</div>
{:else}
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
			
			{#if currAuthStep.isLoading}
				<div class="loading_screen_sub">
					<p>Инициализация защищенного соединения...</p>
				</div>
			{:else if 'SuccessShort' in currAuthStep.step }
				{@render children()}
			{:else}
				<AuthManager/>
			{/if}
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
{/if}

