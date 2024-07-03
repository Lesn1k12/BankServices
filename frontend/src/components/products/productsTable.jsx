'use client';

import React, { useEffect, useState } from 'react';
import {
  Table,
  TableHeader,
  TableBody,
  TableColumn,
  TableRow,
  TableCell
} from "@nextui-org/table";
import { getAllProducts } from '@/app/api/apiProducts';

import GetProduct from './getProduct';
import DeleteProduct from './DeleteProduct';

const ProductsTable = () => {
  const [allProducts, setAllProducts] = useState();
  const [data, setData] = useState({
    name: 'name3',
    highest_to_lowest: false,
    lowest_to_highest: true,
    country: 'country',
    region: 'region',
  }); 
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const products = await getAllProducts(data);
        setAllProducts(products);
      } catch (error) {
        setError(error.message);
      }
    };

    fetchData();
  }, [data]);

  if (error) {
    return <div>Error: {error}</div>;
  }

  if (!Array.isArray(allProducts)) {
    console.error('Expected allProducts to be an array, got:', allProducts);
    return null;
  }

  return (
    <Table aria-label="Example static collection table">
      <TableHeader>
        <TableColumn>ID</TableColumn>
        <TableColumn>NAME</TableColumn>
        <TableColumn>PRICE</TableColumn>
        <TableColumn>CATEGORY</TableColumn>
        <TableColumn>STORAGE COUNTRY</TableColumn>
        <TableColumn>STORAGE REGION</TableColumn>
        <TableColumn>STORAGE STREET</TableColumn>
        <TableColumn>STORAGE QUANTITY</TableColumn>
        <TableColumn>get</TableColumn>
        <TableColumn>delete</TableColumn>
      </TableHeader>
      <TableBody>
        {allProducts.map((product) => (
          <TableRow key={product.id}>
            <TableCell>{product.id}</TableCell>
            <TableCell>{product.name}</TableCell>
            <TableCell>{product.price}</TableCell>
            <TableCell>{product.category}</TableCell>
            <TableCell>{product.storage_country}</TableCell>
            <TableCell>{product.storage_region}</TableCell>
            <TableCell>{product.storage_street}</TableCell>
            <TableCell>{product.storage_quantity}</TableCell>
            <TableCell><GetProduct id={product.id}/></TableCell>
            <TableCell><DeleteProduct id={product.id}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

export default ProductsTable;
