<script>
    let page = "register"
    let name = "";
    let email = "";
    let phone = "";
    let telegramHandle = "";
    let errorMessage = "";
    let otp = "";
    let sessionToken = "";
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
                telegram: telegramHandle
            })
        });
        if (response.ok) {
            alert('success');
            const respBody = await response.json();
            sessionToken = respBody.sessionToken;
            page = "verify"
        } else {
            throw new Error('Something went wrong');
        }
    } catch (error) {
        errorMessage = error.message;
    }
}
async function verifyOTP() {
    try{
        const response = await fetch('/api/verify', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                otp,
                sessionToken: sessionToken
            })
        });
        if (response.ok) {
            const body = await response.json()
            if (body.status === 'success') {
                alert('success');
                window.location.replace('/')
            } else {
                throw new Error('Something went wrong');
            }
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
{#if page === "register"}
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
{/if}
{#if page === "verify"}
    <p> Enter OTP below</p>
    <div>
    <input type="text" name="otp" id="otp"  bind:value={otp}>
    <button on:click={verifyOTP}>Verify</button>
    </div>
{/if}
