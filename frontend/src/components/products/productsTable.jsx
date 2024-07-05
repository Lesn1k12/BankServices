'use client'

import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { getAllProducts } from '@/app/api/apiProducts';

const API_URL = 'http://localhost:8083';

const ProductsTable = () => {
  const [allProducts, setAllProducts] = useState([]);
  const [data, setData] = useState({
    name: 'name3',
    highest_to_lowest: null,
    lowest_to_highest: null,
    country: null,
    region: null,
  });
  const [error, setError] = useState(null);

  useEffect(() => {
    getAllProducts(data)
      .then((response) => {
        setAllProducts(response);
      })
      .catch((error) => {
        setError(error);
      });
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  if (!Array.isArray(allProducts)) {
    console.error('Expected allProducts to be an array, got:', allProducts);
    return null;
  }

  return (
    <table aria-label="Example static collection table">
      <thead>
        <tr>
          <th>ID</th>
          <th>NAME</th>
          <th>PRICE</th>
          <th>CATEGORY</th>
          <th>STORAGE COUNTRY</th>
          <th>STORAGE REGION</th>
          <th>STORAGE STREET</th>
          <th>STORAGE QUANTITY</th>
        </tr>
      </thead>
      <tbody>
        {allProducts.map((product) => (
          <tr key={product.id}>
            <td>{product.id}</td>
            <td>{product.name}</td>
            <td>{product.price}</td>
            <td>{product.category}</td>
            <td>{product.storage_country}</td>
            <td>{product.storage_region}</td>
            <td>{product.storage_street}</td>
            <td>{product.storage_quantity}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default ProductsTable;
