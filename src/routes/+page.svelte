<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import type { State } from '$lib';
	import { emitTo } from '@tauri-apps/api/event';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	// Don't touch this, only use setIntervalId
	let intervalId: number | null = $state(null);

	function setIntervalId(id: number) {
		if (intervalId !== null) {
			clearInterval(intervalId);
		}
		intervalId = id;
	}

	async function getDisplayMedia() {
		const stream = await navigator.mediaDevices.getDisplayMedia({
			video: true,
			audio: false,
		});
		const video = document.querySelector('video');
		const canvas = document.querySelector('canvas');

		if (!video || !canvas) {
			console.error('Video or canvas element not found');
			return;
		}

		video.srcObject = stream;

		await video.play();

		setIntervalId(
			setInterval(() => {
				canvas.width = video.videoWidth;
				canvas.height = video.videoHeight;
				const ctx = canvas.getContext('2d');

				if (!ctx) {
					console.error('Canvas context not found');
					return;
				}
				ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
				canvas.toBlob(
					async (blob) => {
						if (!blob) {
							console.error('Canvas toBlob failed');
							return;
						}

						const arrayBuffer = await blob.arrayBuffer();
						const uint8Array = new Uint8Array(arrayBuffer);

						const state = await invoke<State>('parse_image', {
							image: Array.from(uint8Array),
						});

						emitTo('menhera', 'menhera_state', {
							state,
						});
					},
					'image/png',
					1.0
				);
			}, 5000)
		);

		await invoke('open_menhera');
		await getCurrentWindow().hide();
	}
</script>

<main class="container">
	<button class="row" onclick={getDisplayMedia}> Get Display Media </button>

	<!-- svelte-ignore a11y_media_has_caption -->
	<video autoplay class="row"></video>

	<canvas class="row"></canvas>
</main>

<style>
	:root {
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 24px;
		font-weight: 400;

		/* overflow: hidden; */

		color: #0f0f0f;
		background-color: #f6f6f6;

		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;
	}

	.container {
		margin: 0;
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		text-align: center;
	}

	.row {
		display: flex;
		justify-content: center;
	}

	.hidden {
		display: none;
	}

	button {
		border-radius: 8px;
		border: 1px solid transparent;
		padding: 0.6em 1.2em;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #0f0f0f;
		background-color: #ffffff;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}

	button {
		cursor: pointer;
	}

	button:hover {
		border-color: #396cd8;
	}
	button:active {
		border-color: #396cd8;
		background-color: #e8e8e8;
	}

	button {
		outline: none;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			color: #f6f6f6;
			background-color: #2f2f2f;
		}

		button {
			color: #ffffff;
			background-color: #0f0f0f98;
		}
		button:active {
			background-color: #0f0f0f69;
		}
	}
</style>
