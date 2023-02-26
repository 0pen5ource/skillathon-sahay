<script>
	import gurukul from '$lib/images/gurukul.png';
	let token = 'sahay';
	export const load = (async ({ cookies }) => {
		token = cookies.split(';').find(row => row.trim().startsWith('token='));
	});
	let searchText = ''
	let log = ''
	const search = async () => {
		console.log(searchText);
		const resp = await fetch('/api/search', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				sessionTitle: searchText
			})
		})
		const data = await resp.json()
		log += JSON.stringify(data, null, 2)
	};

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Sahay" />
</svelte:head>

<section>
	{#if token}
		<label for="search">Search</label>
		<input id="search" bind:value={searchText} placeholder="Language tutor">
		<button on:click={search}>Search</button>

		<br/>

		<textarea bind:value={log} />
	{:else}
		<a href="/signup">Enroll</a>
		<h1>
			<span class="welcome">
				<picture>
	<!--				<source srcset={welcome} type="image/webp" />-->
					<img src={gurukul} alt="Welcome" />
				</picture>
			</span>


		</h1>
	{/if}



</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		/*position: absolute;*/
		/*width: 100%;*/
		/*height: 100%;*/
		top: 0;
		/*display: block;*/
	}
</style>
