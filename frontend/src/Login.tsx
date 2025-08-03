import React, { useState } from 'react';

interface Props {
  onLogin: (token: string) => void;
}

function Login({ onLogin }: Props) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState('');

  const login = async () => {
    try {
      const res = await fetch('http://localhost:3001/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });

      if (!res.ok) {
        setMessage('Login failed');
        return;
      }

      const data = await res.json();
      localStorage.setItem('token', data.token);
      onLogin(data.token);
    } catch (err) {
      setMessage('Error connecting to server');
    }
  };

  return (
    <div>
      <h2>Login</h2>
      <input placeholder="Username" value={username} onChange={e => setUsername(e.target.value)} />
      <input placeholder="Password" type="password" value={password} onChange={e => setPassword(e.target.value)} />
      <button onClick={login}>Login</button>
      <p>{message}</p>
    </div>
  );
}

export default Login;
