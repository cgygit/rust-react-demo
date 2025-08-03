import React, { useEffect, useState } from 'react';
import Register from './Register';
import Login from './Login';

function App() {
  const [token, setToken] = useState<string | null>(null);

  useEffect(() => {
    const stored = localStorage.getItem('token');
    if (stored) setToken(stored);
  }, []);

  return (
    <div style={{ padding: '2rem' }}>
      <h1>Auth Demo</h1>

      {token ? (
        <>
          <p style={{ color: 'green' }}>✅ Logged in</p>
          <button onClick={() => {
            localStorage.removeItem('token');
            setToken(null);
          }}>Logout</button>
        </>
      ) : (
        <>
          <Register />
          <hr />
          <Login onLogin={setToken} />
        </>
      )}
    </div>
  );
}

export default App;
