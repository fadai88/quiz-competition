const io = require('socket.io-client');
const socket = io('http://localhost:5000'); // Ensure this matches your server URL

socket.on('connect', () => {
    console.log('Connected to server');
    socket.emit('register', { username: 'testUser', email: 'test@example.com', password: 'password123' });
});

socket.on('registrationSuccess', () => {
    console.log('Registration successful');
});

socket.on('registrationFailure', (message) => {
    console.log('Registration failed:', message);
});