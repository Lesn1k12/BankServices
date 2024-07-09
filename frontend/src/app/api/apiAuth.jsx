import axios from "axios";

const API_URL = 'http://localhost:8080/api';

export const getTokenFromLocalStorage = () => {
  const token = localStorage.getItem('token');
  return token;
};

export const login = async (username, password) => {
    console.log('Attempting to log in', { username, password });
    try {
      const response = await axios.post(`http://localhost:8081/auth/login`, {
        username,
        password,
      });
      console.log('Login successful', response.data);
      localStorage.setItem('token', response.data); // Assuming the token is in response.data.token
    } catch (error) {
      console.error('Login error:', error);
      throw error; // Re-throw the error to be caught in the component
    }
  }
  

export const register = async (username, password, role) => {
    try {
        const response = await axios.post(`${API_URL}/register`, {
            username,
            password,
            role,
        });
        console.log(response.data);
    } catch (error) {
        console.error(error);
    }
}