<script lang="ts">
	import Clock from "../components/clock.svelte";
	import Weather from "../components/weather.svelte";
</script>

<div
	class="grid md:grid-cols-2 h-full max-h-full *:bg-neutral-900 *:rounded-md *:p-2 *:h-full gap-2 text-base md:text-4xl"
>
	<Clock />
	<Weather />
</div>
