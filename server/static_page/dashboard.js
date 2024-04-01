async function performAction(action) {
    const username = action === 'login' ?
        document.getElementById('login-username').value :
        document.getElementById('register-username').value;

    const password = action === 'login' ?
        document.getElementById('login-password').value :
        document.getElementById('register-password').value;

    const endpoint = action === 'login' ? 'login' : 'register';

    const response = await fetch(`http://127.0.0.1:8087/${endpoint}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    });

    if (response.ok) {
        if (action === 'login') {
            alert('Login successful');
        } else {
            alert('User registered successfully');
        }
    } else if (response.status === 409) {
        alert('Username already exists');
    } else {
        alert(action === 'login' ? 'Invalid username or password' : 'Registration failed');
    }
}