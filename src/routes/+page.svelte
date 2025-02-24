<script lang="ts">
	import { onMount } from 'svelte';
	import { createOscillator } from '$lib/processor/osc';

	let audioCtx: AudioContext;
	let oscNode: AudioWorkletNode | null = null;

	onMount(async () => {
		if (!audioCtx) {
			audioCtx = new AudioContext();
		}
		oscNode = await createOscillator(audioCtx, 440);
	})
</script>
