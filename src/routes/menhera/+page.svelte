<script lang="ts">
	import { State } from '$lib';
	import { listen } from '@tauri-apps/api/event';
	import { CheckMenuItem, Menu, MenuItem, PredefinedMenuItem } from '@tauri-apps/api/menu';
	import { onMount } from 'svelte';

	let state: string = $state(State.Unknown);
	let derived: string = $derived(
		state === State.Idle || state === State.Distracted1
			? `/menhera_imgs/${state}.png`
			: `/menhera_imgs/${state}.gif`
	);

	listen('menhera_state', (event) => {
		const payload = event.payload as { state: State };
		state = payload.state;
	});

	$inspect(state);
</script>

<main
	class="main"
	data-tauri-drag-region
	oncontextmenu={async (e) => {
		e.preventDefault();
		const menu = await Menu.new({
			items: [
				await MenuItem.new({
					id: 'menhera',
					text: 'Menhera',
					enabled: false,
				}),
				await PredefinedMenuItem.new({ item: 'Separator' }),
				await CheckMenuItem.new({
					id: 'hideFocused',
					text: 'Hide on Focused',
				}),
				await CheckMenuItem.new({
					id: 'hideIdle',
					text: 'Hide on Idle',
				}),
				await PredefinedMenuItem.new({ item: 'Separator' }),
				await PredefinedMenuItem.new({ item: 'Quit' }),
			],
		});
		await menu.popup();
	}}
>
	{#if state}
		<img src={derived} alt="" />
	{/if}
</main>

<style>
	:root {
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 24px;
		font-weight: 400;
		overflow: hidden;
		overscroll-behavior: none;
	}

	.main {
		width: 100vw;
		height: 100vh;
		display: grid;
		place-items: center;
	}

	img {
		pointer-events: none;
		user-select: none;
		object-fit: cover;
		width: 100vw;
		height: 100vh;
	}
</style>
