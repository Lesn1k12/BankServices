'use client'

import React, {useState} from "react";
import axios from "axios";  
import {Input, Button} from "@nextui-org/react";

function Register() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [role, setRole] = useState('user');

    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await axios.post('http://localhost:8080/api/register', {
                username,
                password,
                role,
            });
            console.log(response.data);
        } catch (error) {
            console.error(error);
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
      <div className="flex gap-2 justify-end">
        <Button fullWidth color="primary" type="submit">
          Sign up
        </Button>
      </div>
    </form>
  );
}

export default Register;

