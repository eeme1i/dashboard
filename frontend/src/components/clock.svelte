<script lang="ts">
	let date = $state(new Date());
	const interval = setInterval(() => {
		date = new Date();
	}, 1000);

	let formattedDate = $derived(formatDate(date));

	function formatDate(d: Date) {
		const dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
		const monthNames = [
			"Jan",
			"Feb",
			"Mar",
			"Apr",
			"May",
			"Jun",
			"Jul",
			"Aug",
			"Sep",
			"Oct",
			"Nov",
			"Dec",
		];

		const day = dayNames[d.getDay()];
		const date = d.getDate();
		const month = monthNames[d.getMonth()];
		const time = d.toLocaleTimeString("en-US", {
			hour12: false,
			hour: "2-digit",
			minute: "2-digit",
			second: "2-digit",
		});

		return {
			day,
			date,
			month,
			time,
		};
	}
</script>

<div>
	<div class="flex flex-col items-start justify-end h-full">
		<div class="text-neutral-400">
			{formattedDate.day},
			{formattedDate.date}
			{formattedDate.month}
		</div>
		<div
			class="text-6xl md:text-7xl lg:text-8xl overflow-hidden wrap-break-word"
		>
			{formattedDate.time}
		</div>
	</div>
</div>
