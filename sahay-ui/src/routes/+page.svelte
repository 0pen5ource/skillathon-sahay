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
		result = {  "context": {    "domain": "dsep:mentoring",    "action": "on_search",    "bap_id": "https://sahaay.xiv.in/bap",    "bap_uri": "https://sahaay.xiv.in/bap",    "bpp_id": "dev.elevate-apis.shikshalokam.org/bpp",    "bpp_uri": "https://dev.elevate-apis.shikshalokam.org/bpp",    "timestamp": "2023-02-26T04:39:58.316Z",    "ttl": "PT10M",    "version": "1.0.0",    "message_id": "d3a075b2-a5f6-4d01-9e09-2fb356e1a12c",    "transaction_id": "ebb98d27-1b73-4d20-acdc-dd8bef91de24"  },  "message": {    "catalog": {      "providers": [        {          "id": "63d103e62d52ec96cf85efa5",          "categories": [            {              "id": "4229d975-43ae-e560-3ed5-439283800560",              "descriptor": {                "code": "4229d975-43ae-e560-3ed5-439283800560",                "name": "Master of science ",                "short_desc": null,                "long_desc": null,                "images": null              }            }          ],          "descriptor": {            "code": "org1",            "name": "ShikshalokamOne",            "short_desc": "ShikshalokamOrgOne",            "long_desc": null,            "images": null          },          "items": [            {              "quantity": {                "available": {                  "count": 5                },                "allocated": {                  "count": 10                }              },              "price": {                "value": "0"              },              "id": "63fa069b23df0828569386ea",              "category_ids": [                "4229d975-43ae-e560-3ed5-439283800560"              ],              "descriptor": {                "code": "mentorship for MS",                "name": "mentorship for MS",                "short_desc": "mentorship for MS",                "long_desc": "mentorship for MS",                "images": [                  {                    "url": "https://loremflickr.com/640/480/abstract?random=yroxtztln5"                  }                ]              },              "fulfillment_ids": [                "90f0d8db-5b5b-4d25-847f-5383ab627dcb"              ],              "tags": [                {                  "display": true,                  "descriptor": {                    "code": "recommended_for",                    "name": "recommended_for",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "1902e455-5282-f636-7f7d-bd123c16f26d",                        "name": "Master graduation ",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "about_mentor",                    "name": "About Mentor",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "about_mentor",                        "name": "This mentor is a true pioneer in their field, having helped to shape the industry in countless ways over the course of their career. They are highly respected for their expertise and leadership, and are always eager to share their insights with others. With a deep understanding of the industry's history and a keen eye for future trends, this mentor is a powerful ally for anyone looking to make their mark in this field.",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "professional_experience",                    "name": "Professional Experience",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "professional_experience",                        "name": "Principal Engineer | Chief Innovation Officer - ThinkTank Solutions",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "qualification",                    "name": "Qualification",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "qualification",                        "name": "Ph.D from NID Ahmedabad",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "experience",                    "name": "Experience",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "experience",                        "name": "13 Years",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "total_meetings",                    "name": "Total Meetings",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "total_meetings",                        "name": "88+",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": true,                  "descriptor": {                    "code": "specialist_in",                    "name": "Specialist In",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "specialist_in",                        "name": "UI/UX Design",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                }              ]            }          ],          "fulfillments": [            {              "language": [                "English"              ],              "id": "90f0d8db-5b5b-4d25-847f-5383ab627dcb",              "time": {                "range": {                  "start": "2023-02-25T18:59:00",                  "end": "2023-02-26T17:59:00"                },                "label": "Session Timing"              },              "type": "ONLINE",              "tags": [                {                  "display": null,                  "descriptor": {                    "code": "status",                    "name": "status",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "live",                        "name": "Live",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                },                {                  "display": null,                  "descriptor": {                    "code": "timeZone",                    "name": "timeZone",                    "short_desc": null,                    "long_desc": null,                    "images": null                  },                  "code": null,                  "name": null,                  "list": [                    {                      "descriptor": {                        "code": "Asia/Calcutta",                        "name": "Asia/Calcutta",                        "short_desc": null,                        "long_desc": null,                        "images": null                      },                      "code": null,                      "name": null                    }                  ]                }              ],              "agent": {                "person": {                  "name": "Tejash",                  "id": "63fa05f362820fd9e6beb82a"                }              }            }          ]        }      ]    },    "intent": null,    "order": null  }};
	};

	let result = {"message":{"catalog":{"providers":[]}}};
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Sahay" />
</svelte:head>

<section>
	{#if token}
		<label for="search"></label>
		<span>
		<input id="search" bind:value={searchText} placeholder="Language tutor">
		<button on:click={search}>Search Mentors</button>
		</span>
		<br/>


		<table>
			{#if result.message.catalog.providers.length}
			<tr><td>Number of providers :{result.message.catalog.providers.length || ''}</td></tr>
			{#each result.message.catalog.providers as provider}
				<tr><td><h3>{provider.descriptor.name}</h3></td></tr>
				{#each provider.items as item}
					<tr><td>{item.descriptor.name}</td><td><button>Apply</button></td></tr>
				{/each}
			{/each}
			{:else}
				<tr><td>Please search for mentors with keywords</td></tr>
			{/if}
		</table>
		<textarea bind:value={log} style="position: absolute; bottom: 0"/>
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
