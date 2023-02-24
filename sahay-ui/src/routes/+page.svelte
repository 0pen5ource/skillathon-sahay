<script>
	import gurukul from '$lib/images/gurukul.png';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Sahay" />
</svelte:head>

<section>
	<a href="/signup">Enroll</a>
	<h1>
		<span class="welcome">
			<picture>
<!--				<source srcset={welcome} type="image/webp" />-->
				<img src={gurukul} alt="Welcome" />
			</picture>
		</span>


	</h1>



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
