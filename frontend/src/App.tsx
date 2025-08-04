import React, { useEffect, useState } from 'react';
import { BrowserRouter, Routes, Route, Navigate, useNavigate } from 'react-router-dom';
import Register from './Register';
import Login from './Login';
import DeviceManager from './DeviceManager';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<AuthPage />} />
        <Route path="/devices" element={<DevicePage />} />
      </Routes>
    </BrowserRouter>
  );
}

function AuthPage() {
  const navigate = useNavigate();
  const [token, setToken] = useState<string | null>(null);

  useEffect(() => {
    const stored = localStorage.getItem('token');
    if (stored) {
      navigate('/devices');
    }
  }, []);

  const handleLogin = (token: string) => {
    localStorage.setItem('token', token);
    setToken(token);
    navigate('/devices');
  };

  return (
    <div style={{ padding: '2rem' }}>
      <h1>Login or Register</h1>
      <Register />
      <hr />
      <Login onLogin={handleLogin} />
    </div>
  );
}

function DevicePage() {
  const navigate = useNavigate();
  const token = localStorage.getItem('token');

  useEffect(() => {
    if (!token) {
      navigate('/');
    }
  }, [token]);

  const handleLogout = () => {
    localStorage.removeItem('token');
    navigate('/');
  };

  return (
    <div style={{ padding: '2rem' }}>
      <h1>Device Management</h1>
      <button onClick={handleLogout}>Logout</button>
      <hr />
      {token && <DeviceManager token={token} />}
    </div>
  );
}

export default App;
