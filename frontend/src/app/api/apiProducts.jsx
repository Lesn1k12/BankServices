import axios from "axios";

const API_URL = 'http://localhost:8080/api';

export const createProduct = async (data) => {
    try {
        const response = await axios.post(`${API_URL}/create_product`, data, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        console.log(response.data);
    } catch (error) {
        console.error(error);
    }
};

export const getProduct = async (id) => {
    try {
        const response = await axios.get(`${API_URL}/read_product/${id}`);
        console.log(response.data);
        return response.data
    } catch (error) {
        console.error(error);
        return {
            name: 'name',
            price: '123.123',
            category: 'category',
            storage_country: 'country',
            storage_region: 'region',
            storage_street: 'street',
            storage_quantity: '5'
        }
    }
};

const API_URL_PROD = 'http://localhost:8083';

export const getAllProducts = async (data) => {
    try {
        const response = await axios.get(`${API_URL_PROD}/products/sort_products`);
        console.log(response.data);
        return response.data;
    } catch (error) {
        console.error('API Error:', error.response ? error.response.data : error.message);
        return ([
            {
                id: 1,
                name: 'name',
                price: '123.123',
                category: 'category',
                storage_country: 'country',
                storage_region: 'region',
                storage_street: 'street',
                storage_quantity: '5'
            },
            {
                id: 2,
                name: 'name2',
                price: '123.1232',
                category: 'category2',
                storage_country: 'country2',
                storage_region: 'region2',
                storage_street: 'street2',
                storage_quantity: '52'
            },
        ]);
    }
};


export const deleteProduct = async (id) => {
    try {
        const response = await axios.delete(`${API_URL}/remove_product/${id}`);
        console.log(response.data);
    } catch (error) {
        console.error(error);
    }
};

export const updateProduct = async (id, data) => {  // Додаємо data як параметр
    try {
        const response = await axios.put(`${API_URL}/update_product/${id}`, data, {
            headers: {
                'Content-Type': 'application/json',
            },
        });
        console.log(response.data);
    } catch (error) {
        console.error(error);
    }
};
