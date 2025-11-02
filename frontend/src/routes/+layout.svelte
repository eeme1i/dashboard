<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div
	class="h-dvh w-dvw bg-neutral-900 p-0 sm:p-2 flex items-center justify-center tracking-tighter font-extralight overflow-hidden"
>
	<main class=" flex-col flex-1 h-full rounded-lg bg-neutral-800 p-2">
		{@render children()}
	</main>
</div>
