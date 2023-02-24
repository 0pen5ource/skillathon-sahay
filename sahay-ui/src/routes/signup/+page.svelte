<script>
    let name = "";
    let email = "";
    let phone = "";
    let telegramHandle = "";
    let errorMessage = "";
async function postForm() {
    try{
        const response = await fetch('/api/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name,
                email,
                phone,
                telegramHandle
            })
        });
        if (response.ok) {
            alert('success');
        } else {
            throw new Error('Something went wrong');
        }
    } catch (error) {
        errorMessage = error.message;
    }
}
</script>

<h1>Register</h1>
{#if errorMessage}
    <p style="color:chocolate">{errorMessage}</p>
{/if}
<table>
    <tr>
        <td><label for="name">Name</label></td>
        <td><input name="name" id="name" bind:value={name}></td></tr>
    <tr><td><label for="email">Email</label></td>
        <td><input name="email" id="email" bind:value={email}></td></tr>
    <tr><td><label for="telegram">Telegram</label></td>
    <td><input name="telegram" id="telegram" bind:value={telegramHandle}></td>
    </tr>
    <tr><td><label for="phone">Phone</label></td>
    <td><input name="phone" id="phone" bind:value={phone}></td></tr>
    <tr><td><button on:click|preventDefault={postForm}>Sign Up</button></td></tr>
</table>