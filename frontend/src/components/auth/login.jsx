'use client';

import React, { useState } from "react";
import { Input, Button } from "@nextui-org/react";
import {login} from "@/app/api/apiAuth";  

function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [role, setRole] = useState('user');
  const [error, setError] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log('Form submitted', { username, password, role });
    try {
      await login(username, password, role);
      setError('');
    } catch (err) {
      console.error('Login failed', err);
      setError('Login failed. Please check your credentials.');
    }
  };

  return (
    <form className="flex flex-col gap-4 h-[300px]" onSubmit={handleSubmit}>
      <Input
        isRequired
        label="Name"
        placeholder="Enter your name"
        type="text"  
        onChange={(e) => setUsername(e.target.value)}
      />
      <Input
        isRequired
        label="Password"
        placeholder="Enter your password"
        type="password"
        onChange={(e) => setPassword(e.target.value)}
      />
      {error && <p className="text-center text-small text-red-500">{error}</p>}
      <div className="flex gap-2 justify-end">
        <Button fullWidth color="primary" type="submit">
          Sign up
        </Button>
      </div>
    </form>
  );
}

export default Login;