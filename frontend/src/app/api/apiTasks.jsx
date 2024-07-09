import axios from "axios";
import { getTokenFromLocalStorage } from "./apiAuth";

const API_Tasks = 'http://localhost:8080/api/tasks/'



export const createTask = async (data) => {
    try{
        const token = getTokenFromLocalStorage();
        const response = await axios.post(`${API_Tasks}/create`, 
            data,
            {
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            }
        })
        return response.data
        console.log("success", response.data)
    } 
    catch (error) {
        console.log(error)
    }  
}

export const getTasks = async () => {
    try{
        const token = getTokenFromLocalStorage();
        const response = await axios.get(`${API_Tasks}/getTasks`, 
            {
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            }
        })
        return response.data
        console.log("success", response.data)
    } catch (error) {
        console.log(error)
    }
}

export const getTask = async (id) => {
    try{
        const token = getTokenFromLocalStorage();
        const response = await axios.get(`${API_Tasks}/getTask/${id}`, 
            {
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            }
        })
        return response.data
        console.log("success", response.data)
    } catch (error) {
        console.log(error)
    }
}

export const updateTask = async (id, data) => {
    try{
        const token = getTokenFromLocalStorage();
        const response = await axios.put(`${API_Tasks}/update/${id}`, 
            data,
            {
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            }
        })
        return response.data
        console.log("success", response.data)
    } catch (error) {
        console.log(error)
    }
}

export const deleteTask = async (id) => {
    try{
        const token = getTokenFromLocalStorage();
        const response = await axios.delete(`${API_Tasks}/delete/${id}`, 
            {
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            }
        })
        return response.data
        console.log("success", response.data)
    } catch (error) {
        console.log(error)
    }
}