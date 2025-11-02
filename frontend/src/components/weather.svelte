<script lang="ts">
	import { Navigation } from "lucide-svelte";
	import { onMount } from "svelte";
	import { PUBLIC_BACKEND_URL } from "$env/static/public";

	const BACKEND_URL = PUBLIC_BACKEND_URL || "http://localhost:3000";

	let data = $state<{ summary: string | null; temperature: number | null }>({
		summary: "Loading...",
		temperature: 0,
	});

	onMount(async () => {
		savedLocation = loadLocationFromLocalStorage();
		newLocation = savedLocation;

		if (savedLocation) {
			await Promise.all([getWeatherSummary(), getCurrentTemperature()]);
		}
	});

	let newLocation = $state<string | null>(null);
	let savedLocation = $state<string | null>(null);

	type LoadingState = {
		summary: "loading" | "loaded" | "error";
		temperature: "loading" | "loaded" | "error";
	};

	let loadingState = $state<LoadingState>({
		summary: "loading",
		temperature: "loading",
	});

	let inputFocused = $state(false);

	function saveLocationToLocalStorage(loc: string | null) {
		if (loc) {
			localStorage.setItem("location", loc);
		} else {
			localStorage.removeItem("location");
		}
	}

	function loadLocationFromLocalStorage(): string | null {
		const storedLocation = localStorage.getItem("location");
		return storedLocation ? storedLocation : null;
	}

	async function getWeatherSummary() {
		// get user timezone
		const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		try {
			loadingState.summary = "loading";
			// add timezone as query param
			const response = await fetch(
				`${BACKEND_URL}/api/weather/${savedLocation}/summary?timezone=${userTimezone}`
			);
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			const weatherData = await response.json();
			data.summary = weatherData.summary;
			loadingState.summary = "loaded";
			return;
		} catch (error) {
			data.summary = "Error loading weather";
			loadingState.summary = "error";
			return;
		}
	}
	async function getCurrentTemperature() {
		console.log("Fetching temperature for location:", savedLocation);
		try {
			loadingState.temperature = "loading";
			const response = await fetch(
				`${BACKEND_URL}/api/weather/${savedLocation}/current_temperature`
			);
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			const weatherData = await response.json();
			data.temperature = weatherData.temperature_celsius;
			loadingState.temperature = "loaded";
			console.log("Fetched temperature:", data.temperature);
			return;
		} catch (error) {
			data.temperature = null;
			loadingState.temperature = "error";
			return;
		}
	}

	async function handleSave() {
		savedLocation = newLocation;
		saveLocationToLocalStorage(savedLocation);
		// fetch both at the same time
		await Promise.all([getWeatherSummary(), getCurrentTemperature()]);
	}
</script>

<div
	class="flex flex-col items-start justify-between h-full gap-2 overflow-hidden"
>
	<form
		class="flex gap-1 w-full items-center h-max"
		onsubmit={(e) => {
			e.preventDefault();
			handleSave();
		}}
	>
		<div
			class="rounded-full bg-neutral-800 border-[0.5px] w-full text-base focus:text-white tracking-normal transition-all flex gap-2 px-2 items-center {inputFocused
				? 'border-cyan-400 text-white'
				: 'hover:border-neutral-400 border-neutral-600 text-neutral-400 hover:text-neutral-400'}"
		>
			<Navigation size={12} class={inputFocused ? "text-cyan-400" : ""} />
			<input
				class="w-full h-full py-1 focus:outline-none"
				type="text"
				placeholder="Enter location"
				bind:value={newLocation}
				onfocus={() => (inputFocused = true)}
				onblur={() => (inputFocused = false)}
			/>
		</div>
		{#if newLocation !== savedLocation}
			<button
				class="px-2 rounded-full w-max h-full bg-neutral-700 hover:bg-neutral-600 text-xs text-white/75 hover:text-white transition-all cursor-pointer"
				type="submit"
			>
				Save
			</button>
		{/if}
	</form>
	<div class="flex flex-col flex-1 justify-end">
		<div class="text-neutral-400">
			{#if loadingState.summary === "loading"}
				Loading...
			{:else if loadingState.summary === "error"}
				Error loading weather
			{:else if loadingState.summary === "loaded" && data.summary}
				{data.summary}
			{:else}
				No data
			{/if}
		</div>
		<div class="text-6xl md:text-7xl xl:text-8xl wrap-anywhere">
			{#if loadingState.temperature === "loading"}
				Loading...
			{:else if loadingState.temperature === "error"}
				--
			{:else if loadingState.temperature === "loaded" && data.temperature !== null}
				{data.temperature}°C
			{:else}
				--
			{/if}
		</div>
	</div>
</div>
