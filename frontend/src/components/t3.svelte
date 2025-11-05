<script lang="ts">
	import { Sparkles } from "lucide-svelte";
	import { scale } from "svelte/transition";

	const url = "https://www.t3.chat/new?q=";
	let query = $state("");
	let inputFocused = $state(false);
	let inputEl: HTMLInputElement | null = null;
</script>

<div
	class="w-full h-full flex flex-col justify-center items-center sm:px-16 md:px-8 lg:px-16 xl:px-32 gap-2"
>
	<p class="text-sm text-neutral-400 tracking-normal">
		Ask <a
			href="https://www.t3.chat"
			target="_blank"
			class="hover:text-white transition-color duration-150"
			rel="noopener noreferrer">t3.chat</a
		>
	</p>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<form
		class="flex w-full items-center gap-2 text-base relative rounded-full border-[0.5px] focus:text-white tracking-normal px-2 transition duration-150 cursor-text {inputFocused
			? 'border-pink-500 text-white'
			: 'hover:border-neutral-400 border-neutral-600 text-neutral-400 hover:text-neutral-400'}"
		method="GET"
		target="_blank"
		onclick={() => ((inputFocused = true), inputEl?.focus())}
		onsubmit={(event) => {
			event?.preventDefault();
			if (inputEl) {
				// Trim the query to remove leading/trailing whitespace
				const trimmedQuery = query.trim();
				// If the trimmed query is not empty, set the input value to the trimmed query
				if (trimmedQuery !== "") {
					inputEl.value = trimmedQuery;
				}

				// open in new tab
				const fullUrl = url + encodeURIComponent(trimmedQuery);
				window.open(fullUrl, "_blank");
			}
		}}
	>
		<Sparkles
			size={16}
			class={inputFocused ? "text-pink-500 fill-pink-500" : "text-neutral-400"}
		/>
		<input
			type="text"
			class="w-full h-full py-2 focus:outline-none transition-all duration-150"
			required
			bind:value={query}
			onfocus={() => (inputFocused = true)}
			onblur={() => (inputFocused = false)}
			bind:this={inputEl}
			placeholder="Ask..."
		/>
		<button
			type="submit"
			in:scale={{ start: 0.8, duration: 150 }}
			out:scale={{ start: 1, duration: 150 }}
			class=" right-2 px-2 py-1 rounded-full w-max bg-neutral-800 hover:bg-neutral-700 text-xs text-white/75 hover:text-white transition-all cursor-pointer border border-neutral-600 hover:border-neutral-400
            {query?.trim() !== ''
				? 'opacity-100 visible'
				: 'opacity-0 invisible'}
          ">Ask</button
		>
	</form>
</div>
