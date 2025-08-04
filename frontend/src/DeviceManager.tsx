import React, { useEffect, useState } from 'react';

interface Device {
  id: number;
  name: string;
  description: string | null;
}

interface Props {
  token: string;
}

const DeviceManager: React.FC<Props> = ({ token }) => {
  const [devices, setDevices] = useState<Device[]>([]);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');

  const fetchDevices = async () => {
    const res = await fetch('http://localhost:3001/api/devices', {
      headers: {
        Authorization: `Bearer ${token}`
      }
    });
    if (res.ok) {
      const data = await res.json();
      setDevices(data);
    }
  };

  const addDevice = async () => {
    const res = await fetch('http://localhost:3001/api/devices', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`
      },
      body: JSON.stringify({ name, description })
    });

    if (res.ok) {
      setName('');
      setDescription('');
      fetchDevices();
    }
  };

  useEffect(() => {
    fetchDevices();
  }, []);

  return (
    <div>
      <h2>Devices</h2>
      <ul>
        {devices.map(device => (
          <li key={device.id}>
            <strong>{device.name}</strong>: {device.description}
          </li>
        ))}
      </ul>

      <h3>Add New Device</h3>
      <input
        placeholder="Name"
        value={name}
        onChange={e => setName(e.target.value)}
      />
      <input
        placeholder="Description"
        value={description}
        onChange={e => setDescription(e.target.value)}
      />
      <button onClick={addDevice}>Add Device</button>
    </div>
  );
};

export default DeviceManager;
