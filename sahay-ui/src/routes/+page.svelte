<script>
	import { onMount } from 'svelte';
	import store from '../store.js';
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
		result = {"message":{"catalog":{"providers":[]}}};
	};

	let result = {"message":{"catalog":{"providers":[]}}};

	let messages = [];
	let type = "";
	let certificateOsid = "";
	onMount(() => {
		store.subscribe(async currentMessage => {
			messages = [...messages, currentMessage];
			if (currentMessage.includes("context")) {
				const dsepResponse = JSON.parse(currentMessage)
				type = dsepResponse.context.action;
				if (dsepResponse.context.action === "on_search") {
					if (result.message.catalog.providers.length > 0) {
						result.message.catalog.providers.push(...dsepResponse.message.catalog.providers)
					} else {
						result = dsepResponse;
					}
				}
				if (dsepResponse.context.action === "on_select") {
					result = dsepResponse;
				}
				if (dsepResponse.context.action === "on_confirm") {
					result = dsepResponse;
				}
			}
			if (currentMessage.includes('sunbird-rc.registry.create')) {
				const registryResponse = JSON.parse(currentMessage)
				certificateOsid = registryResponse.result.ProofOfAssociation.osid
			}
			log += '\n received message: ' + JSON.stringify(currentMessage);
		})
	})

	const select = async (itemId, bppUri, messageId, transactionId) => {
		console.log(searchText);
		const resp = await fetch('/api/select', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				bppUri, transactionId, messageId, itemId
			})
		})
		const data = await resp.json()
		log += JSON.stringify(data, null, 2)
	}

	const apply = async (itemId, bppUri, messageId, transactionId, fullfillmentId, mentorshipTitle) => {
		console.log(searchText);
		let name = prompt("Please enter your name", "xxx");
		let emailId = prompt("Please enter your emailId", name+"@mail.com");
		let card = prompt("Please enter your card details", "123412341234");
		let resp = await fetch('/api/init', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				bppUri, transactionId, messageId, itemId, fullfillmentId, card, emailId, name, mentorshipTitle
			})
		})
		let data = await resp.json()
		log += JSON.stringify(data, null, 2)
		resp = await fetch('/api/confirm', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				bppUri, transactionId, messageId, itemId, fullfillmentId, card, emailId, name, mentorshipTitle
			})
		})
		data = await resp.json()
		console.log(data);
		log += JSON.stringify(data, null, 2)
	}

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Sahay" />
</svelte:head>

<section>
	{#if token}
		<label for="search"></label>
		<span>
		<input id="search" bind:value={searchText} placeholder="Mentorship program">
		<button on:click={search}>Search</button>
		</span>
		<br/>


		<table>
			{#if type === "on_search" && result.message.catalog.providers.length}
			<tr><td>Number of providers :{result.message.catalog.providers.length || ''}</td></tr>
			{#each result.message.catalog.providers as provider}
				<tr><td><h3>{provider.descriptor.name}</h3></td></tr>
				{#each provider.items as item}
					<tr><td>{item.descriptor.name}</td><td><button on:click={() =>
					select(item.id, result.context.bpp_uri, result.context.message_id, result.context.transaction_id )}>View Details</button></td></tr>
				{/each}
			{/each}
			{/if}
			{#if type === ""}
				<tr><td>Please search for mentorship programs </td></tr>
			{/if}
			{#if type === "on_select" && Object.keys(result?.message?.order?.provider || {}).length > 0}
				<tr><td><h3>{result?.message?.order?.provider.descriptor.name}</h3></td></tr>
				{#each result?.message?.order?.provider.items as item}
					<tr><td><b>TITLE:</b>{item.descriptor.name}</td><td>
						{#if false}
							<span>Applied</span>
						{:else}
							<button disabled='{new Date(result?.message?.order?.provider.fulfillments[0].time.range.end) - new Date() < 0}' on:click={() =>
					apply(item.id, result.context.bpp_uri, result.context.message_id, result.context.transaction_id, item.fulfillment_ids[0], item.descriptor.name )}>{
									new Date(result?.message?.order?.provider.fulfillments[0].time.range.end) - new Date() > 0 ? "Apply" : "Expired"
							}</button>
						{/if}

					</td></tr>
				{/each}
				<h4>Mentor Details</h4>
				{#each result?.message?.order?.provider.fulfillments as fulfillment}
					<tr><td><b>Name:</b>{fulfillment.agent.person.name}</td></tr>
					<tr><td><b>Start Date:</b>{fulfillment.time.range.start}</td></tr>
					<tr><td><b>End Date:</b>{fulfillment.time.range.end}</td></tr>
					<tr><td><b>Language:</b>{fulfillment.language}</td></tr>
					<tr><td><b>Type:</b>{fulfillment.type}</td></tr>
				{/each}
				<h4>More Details</h4>
				{#each result?.message?.order?.provider.items[0].tags as tag}
					<b>{tag.descriptor.name}: </b>
					<span>{tag.list?.[0].descriptor.name}</span>
					<br/>
				{/each}
			{/if}
			{#if type === "on_confirm"}
				<h4>Join the session here: <a target="_blank" href='{result.message.order.fulfillments[0].tags[0].list[0].descriptor.name}'>{result.message.order.fulfillments[0].tags[0].list[0].descriptor.name}</a></h4>
				<a target="_blank" href='https://sahaay.xiv.in/bap/pdf/{certificateOsid}'>View Certificate</a>
			{/if}
			</table>
<!--		<textarea bind:value={log} style="position: absolute; bottom: 0"/>-->
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
