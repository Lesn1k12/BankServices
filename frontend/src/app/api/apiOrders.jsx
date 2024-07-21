import axios from 'axios';

const API_URL = 'localhost:8080/api/'; 

export const createOrder = async () => {
    try{
        const response = await axios.post(`${API_URL}/create_order`, data, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        console.log(response.data);
    }
    catch(error){
        console.error(error);
    }
}

export const delete_order = async (id) => {
    try{
        const response = await axios.delete(`${API_URL}/create_order/${id}`);
        console.log(response.data);
    }
    catch(error){
        console.error(error);
    }
}

export const read_order = async (id) => {
    try{
        const response = await axios.get(`${API_URL}/create_order/${id}`);
        console.log(response.data);
        return response.data;
    }
    catch(error){
        console.error(error);
    }
}

export const read_all_orders = async () => {
    try{
        const response = await axios.get(`${API_URL}/read_all_orders`);
        console.log(response.data);
        return response.data;
    }
    catch(error){
        console.error(error);
    }
}
