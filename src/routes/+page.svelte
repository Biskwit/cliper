<script lang="ts">
	import { register, unregister } from '@tauri-apps/api/globalShortcut';
	import { Svroller } from 'svrollbar';
	import { emit, listen } from '@tauri-apps/api/event';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import { TEXT_CHANGED, IMAGE_CHANGED, listenText, listenImage, writeText, writeImage } from 'tauri-plugin-clipboard-api';
	import { appWindow, LogicalPosition } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';
    import { HighlightAuto, LineNumbers } from "svelte-highlight";
    import "svelte-highlight/styles/atom-one-dark.css";
	let data: any = [];

	let tauriTextUnlisten: UnlistenFn;
	let tauriImageUnlisten: UnlistenFn;
	let textUnlisten: () => void;
	let imageUnlisten: () => void;

	export async function startListening() {
		tauriTextUnlisten = await listen(TEXT_CHANGED, (event) => {
			data = [{ value: (event.payload as any).value, type: 'text' }, ...data];
			if (data.length > 30) {
				data = data.slice(0, 30);
			}
		});
		tauriImageUnlisten = await listen(IMAGE_CHANGED, (event) => {
            if (!data.find((item: any) => item.value === (event.payload as any).value)) {
                data = [{ value: (event.payload as any).value, type: 'image' }, ...data];
            }
			if (data.length > 30) {
				data = data.slice(0, 30);
			}
		});
		imageUnlisten = listenImage();
		textUnlisten = listenText();
	}

	function stopListening() {
		imageUnlisten();
		textUnlisten();
		tauriTextUnlisten();
		tauriImageUnlisten();
	}

	onMount(async () => {
        appWindow.hide();
		startListening();
		await listen('shortcutPressed', async (event) => {
			const payload = event.payload as any;
			await appWindow.setPosition(new LogicalPosition(payload.x, payload.y));
            appWindow.show();
            await appWindow.setFocus();
		});
		appWindow.listen('tauri://blur', ({ event, payload }) => {
            appWindow.hide();
		});
	});

	onDestroy(() => {
		unregister('Control+Shift+V');
		stopListening();
	});

    const copy = async (text: string) => {
        await writeText(text);
        appWindow.hide();
    };

    const copyImg = async (text: string) => {
        await writeImage(text);
        appWindow.hide();
    };
</script>
<div
	class="container bg-[#17191F]/80 w-full h-[350px] overflow-hidden p-4 text-xs rounded-lg backdrop-blur-lg"
	data-tauri-drag-region
>
	<ul class="w-full">
		<Svroller width="100%" height="318px">
			{#each data as item}
				{#if item.type === 'text'}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<li on:click={() => copy(item.value)}
						class="hover:opacity-80 transition-all duration-200 rounded-lg cursor-pointer my-2"
					>
                    <HighlightAuto let:highlighted langtag="{true}" code={`${item.value.length > 24 ? item.value.slice(0, 24) + ' ...' : item.value}`}>
						<div class="-ml-8">
							<LineNumbers {highlighted}
							--line-number-color="transparent"
							wrapLines hideBorder />
						</div>
                    </HighlightAuto>
					</li>
				{/if}
				{#if item.type === 'image'}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<li on:click={() => copyImg(item.value)}
						class="hover:opacity-80 transition-all duration-200 rounded-lg cursor-pointer my-2"
					>
						<img class="rounded-lg" src={'data:image/png;base64, ' + item.value} height="50" alt="" srcset="" />
					</li>
				{/if}
			{/each}
		</Svroller>
	</ul>
</div>
<style>
    :global(td) {
		border-radius: 8px;
	}
</style>

