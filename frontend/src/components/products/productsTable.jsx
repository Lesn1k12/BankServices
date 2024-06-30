'use client'

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

const ProductsTable = () => {
  const [allProducts, setAllProducts] = useState([]);

  useEffect(() => {
    const fetchData = async () => {
      const products = await getAllProducts();
      setAllProducts(products);
    };

    fetchData();
  }, []);

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
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

export default ProductsTable;
