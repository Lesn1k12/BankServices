'use client'

import React, { useEffect, useState, useMemo } from 'react';
import {Table, TableHeader, TableColumn, TableBody, TableRow, TableCell, Pagination, getKeyValue} from "@nextui-org/react";
import { getAllProducts } from '@/app/api/apiProducts';
import DeleteProduct from './deleteProduct';
import { displayData } from '@/app/admin/products/page';

const ProductsTable = () => {
  const [allProducts, setAllProducts] = useState([]);
  const [page, setPage] = useState(1);
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

    const rowsPerPage = 4;

    const pages = Math.ceil(allProducts.length / rowsPerPage);

    const items = useMemo(() => {
      const start = (page - 1) * rowsPerPage;
      const end = start + rowsPerPage;

      return allProducts.slice(start, end);
    }, [page, allProducts]);

  return (
    <Table aria-label="Example static collection table" bottomContent={
      <div className="flex w-full justify-center">
        <Pagination
          isCompact
          showControls
          showShadow
          color="secondary"
          page={page}
          total={pages}
          onChange={(page) => setPage(page)}
        />
      </div>
    }
    classNames={{
      wrapper: "min-h-[222px]",
    }}>
      <TableHeader>
        <TableColumn className='text-center'>ID</TableColumn>
        <TableColumn className='text-center'>NAME</TableColumn>
        <TableColumn className='text-center'>PRICE</TableColumn>
        <TableColumn className='text-center'>CATEGORY</TableColumn>
        <TableColumn className='text-center'>STORAGE COUNTRY</TableColumn>
        <TableColumn className='text-center'>STORAGE REGION</TableColumn>
        <TableColumn className='text-center'>STORAGE STREET</TableColumn>
        <TableColumn className='text-center'>STORAGE QUANTITY</TableColumn>
        <TableColumn className='text-center'>REDACT</TableColumn>
        <TableColumn className='text-center'>DELETE</TableColumn>
      </TableHeader>
      <TableBody>
        {allProducts.map((product) => (
          <TableRow key={product.id}>
            <TableCell className='text-center'>{product.id}</TableCell>
            <TableCell className='text-center'>{product.name}</TableCell>
            <TableCell className='text-center'>{product.price}</TableCell>
            <TableCell className='text-center'>{product.category}</TableCell>
            <TableCell className='text-center'>{product.storage_country}</TableCell>
            <TableCell className='text-center'>{product.storage_region}</TableCell>
            <TableCell className='text-center'>{product.storage_street}</TableCell>
            <TableCell className='text-center'>{product.storage_quantity}</TableCell>
            <TableCell className='text-center'>Update</TableCell>
            <TableCell className='text-center'><DeleteProduct id={product.id}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

export default ProductsTable;
