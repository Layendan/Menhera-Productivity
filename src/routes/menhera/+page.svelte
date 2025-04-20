<script lang="ts">
	import { State } from '$lib';
	import { listen } from '@tauri-apps/api/event';
	import { CheckMenuItem, Menu, MenuItem, PredefinedMenuItem } from '@tauri-apps/api/menu';

	let stateValue: string = $state(State.Unknown);
	let derived: string = $derived(
		stateValue === State.Idle || stateValue === State.Distracted1
			? `/menhera_imgs/${stateValue}.png`
			: `/menhera_imgs/${stateValue}.gif`
	);

	let hideFocused: boolean = $state(false);
	let hideIdle = $state(false);

	listen('menhera_state', (event) => {
		const payload = event.payload as { state: State };
		stateValue = payload.state;
	});

	$inspect(stateValue);
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
					checked: hideFocused,
					action: () => (hideFocused = !hideFocused),
				}),
				await CheckMenuItem.new({
					id: 'hideIdle',
					text: 'Hide on Idle',
					checked: hideIdle,
					action: () => (hideIdle = !hideIdle),
				}),
				await PredefinedMenuItem.new({ item: 'Separator' }),
				await PredefinedMenuItem.new({ item: 'Quit' }),
			],
		});
		await menu.popup();
	}}
>
	{#if stateValue && !(hideFocused && stateValue.startsWith('Focused')) && !(hideIdle && stateValue === State.Idle)}
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
