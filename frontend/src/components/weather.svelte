<script lang="ts">
	import { Navigation } from "lucide-svelte";
	import { onMount } from "svelte";
	import { PUBLIC_BACKEND_URL } from "$env/static/public";
	import { fade, scale } from "svelte/transition";

	const BACKEND_URL = PUBLIC_BACKEND_URL || "http://localhost:3000";

	let data = $state<{ summary: string | null; temperature: number | null }>({
		summary: "Loading...",
		temperature: 0,
	});

	onMount(async () => {
		savedLocation = loadLocationFromLocalStorage();
		newLocation = savedLocation;

		if (savedLocation) {
			await refreshWeather();
		}
	});

	async function refreshWeather() {
		lastFetchDate = new Date();
		timeSinceLastFetch = 0;

		await Promise.all([getWeatherSummary(), getCurrentTemperature()]);
	}

	let newLocation = $state<string | null>(null);
	let savedLocation = $state<string | null>(null);

	let lastFetchDate = $state<Date | null>(null);
	let timeSinceLastFetch: null | number = null;

	const interval = setInterval(() => {
		if (lastFetchDate) {
			const now = new Date();
			timeSinceLastFetch = now.getTime() - lastFetchDate.getTime();

			if (timeSinceLastFetch! > 10 * 60 * 1000) {
				// more than 10 minutes
				refreshWeather();
			}
		}
	}, 1000);

	type LoadingState = {
		summary: "loading" | "loaded" | "error" | null;
		temperature: "loading" | "loaded" | "error" | null;
	};

	let loadingState = $state<LoadingState>({
		summary: null,
		temperature: null,
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
				`${BACKEND_URL}/api/weather/${newLocation}/summary?timezone=${userTimezone}`
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
			// rethrow so callers (e.g. Promise.all in handleSave) can catch failures
			throw error;
		}
	}
	async function getCurrentTemperature() {
		console.log("Fetching temperature for location:", savedLocation);
		try {
			loadingState.temperature = "loading";
			const response = await fetch(
				`${BACKEND_URL}/api/weather/${newLocation}/current_temperature`
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
			// rethrow so callers (e.g. Promise.all in handleSave) can catch failures
			throw error;
		}
	}

	async function handleSave() {
		// normalize input early
		const trimmed = newLocation?.trim();
		console.log(
			"savedLocation:",
			savedLocation,
			"newLocation:",
			newLocation,
			"trimmed:",
			trimmed
		);
		if (
			!trimmed ||
			trimmed === savedLocation ||
			loadingState.summary === "loading" ||
			loadingState.temperature === "loading"
		)
			return;
		// use the trimmed location for fetching and saving
		newLocation = trimmed;
		try {
			await refreshWeather();
			savedLocation = trimmed;
			saveLocationToLocalStorage(savedLocation);
		} catch (error) {
			console.error("Error saving location:", error);
			console.log("savedLocation:", savedLocation, "newLocation:", newLocation);
		}
	}

	function fadeAndScale(
		node: HTMLElement,
		params: {
			delay?: number;
			duration?: number;
			easing?: (t: number) => number;
			start?: number;
			end?: number;
		}
	) {
		return {
			delay: params.delay || 0,
			duration: params.duration || 400,
			easing: params.easing || ((t) => t),
			css: (t: number) => {
				const scale =
					params.start !== undefined && params.end !== undefined
						? params.start + (params.end - params.start) * t
						: t;
				return `
          opacity: ${t};
          transform: scale(${scale});
        `;
			},
		};
	}
</script>

<div
	class="flex flex-col flex-1 h-full gap-2 overflow-y-auto justify-between items-start"
>
	<form
		class="flex gap-1 w-full justify-start"
		onsubmit={(e) => {
			e.preventDefault();
			handleSave();
		}}
	>
		<div
			class="relative rounded-full border-[0.5px] text-base focus:text-white tracking-normal flex gap-2 px-2 items-center transition duration-150 {inputFocused
				? 'border-cyan-400 text-white'
				: 'hover:border-neutral-400 border-neutral-600 text-neutral-400 hover:text-neutral-400'}"
		>
			<Navigation size={12} class={inputFocused ? "text-cyan-400" : ""} />
			<input
				class="w-max h-full py-2 focus:outline-none transition-all transform duration-150"
				type="text"
				placeholder="Enter location"
				bind:value={newLocation}
				onfocus={() => (inputFocused = true)}
				onblur={() => (inputFocused = false)}
			/>
			<button
				in:fadeAndScale={{ start: 0.8, end: 1, duration: 150 }}
				out:scale={{ start: 1, duration: 150 }}
				class="absolute right-2 px-2 py-1 rounded-full w-max bg-neutral-800 hover:bg-neutral-700 text-xs text-white/75 hover:text-white transition-all cursor-pointer border border-neutral-600 hover:border-neutral-400
            {newLocation !== savedLocation && newLocation?.trim() !== ''
					? 'opacity-100 visible'
					: 'opacity-0 invisible pointer-events-none'}
          "
				type="submit"
			>
				Go
			</button>
		</div>
	</form>
	<div class="flex flex-col items-start">
		<p class="text-neutral-400 text-left">
			{#if !savedLocation}
				Please enter a location.
			{:else if loadingState.summary === "loading"}
				Loading...
			{:else if loadingState.summary === "error"}
				Error loading weather
			{:else if loadingState.summary === "loaded" && data.summary}
				{data.summary}
			{:else}
				No data
			{/if}
		</p>

		<p class="text-6xl md:text-7xl xl:text-8xl wrap-anywhere">
			{#if !savedLocation}
				--
			{:else if loadingState.temperature === "loading"}
				Loading...
			{:else if loadingState.temperature === "error"}
				--
			{:else if loadingState.temperature === "loaded" && data.temperature !== null}
				{data.temperature}°C
			{:else}
				--
			{/if}
		</p>
	</div>
</div>

<style>
	.input-container {
		transition: width 300ms ease-in-out;
	}
</style>
