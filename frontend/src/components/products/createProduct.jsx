"use client";

import React, { useState } from "react";
import { createProduct } from "@/app/api/apiProducts";
import { Button, Card, CardBody, Input, Select, SelectItem, Label  } from "@nextui-org/react";

function CreateProduct() {
  const [product, setProduct] = useState({
    name: "",
    price: 0,
    category: "",
    storage_country: "",
    storage_region: "",
    storage_street: "",
    storage_quantity: 0,
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    createProduct(product);
  };

  const handleSelectChange = (key, value) => {
    setProduct(prev => ({ ...prev, [key]: value }));
  };

  const categories = [
    "category1", "category2", "category3", "category4", "category5"
  ];

  const countries = [
    "country1", "country2", "country3", "country4", "country5"
  ];

  const regions = [
    "region1", "region2", "region3", "region4", "region5"
  ];

  const streets = [
    "street1", "street2", "street3", "street4", "street5"
  ];

  return (
    <Card className="flex flex-col max-w-sm mx-auto text-center">
      <h1 className="px-3 py-2 ">Add product:</h1>
      <CardBody >
        <form onSubmit={handleSubmit} className="flex flex-col gap-1">
          <Input
            type="text"
            value={product.name}
            placeholder="Name"
            onChange={(e) => setProduct({ ...product, name: e.target.value })}
          />
          <Input
            type="number"
            value={product.price}
            placeholder="Price"
            onChange={(e) => setProduct({ ...product, price: e.target.value })}
          />
          <Select placeholder="Select a category" value={product.category} onChange={e => handleSelectChange('category', e)}>
            {categories.map((category, index) => (
              <SelectItem key={index} value={category}>{category}</SelectItem>
            ))}
          </Select>
          <Select placeholder="Select a coutry" value={product.storage_country} onChange={e => handleSelectChange('storage_country', e)}>
            {countries.map((country, index) => (
              <SelectItem key={index} value={country}>{country}</SelectItem>
            ))}
          </Select>
          <Select placeholder="Select a region" value={product.storage_region} onChange={e => handleSelectChange('storage_region', e)}>
            {regions.map((region, index) => (
              <SelectItem key={index} value={region}>{region}</SelectItem>
            ))}
          </Select>
          <Select placeholder="Select a street"  value={product.storage_street} onChange={e => handleSelectChange('storage_street', e)}>
            {streets.map((street, index) => (
              <SelectItem key={index} value={street}>{street}</SelectItem>
            ))}
          </Select>
          <Input
            type="number"
            value={product.storage_quantity}
            placeholder="Quantity"
            onChange={(e) => setProduct({ ...product, storage_quantity: e.target.value })}
          />
        </form>
        <Button color="primary" type="submit" style = {{ marginTop: '20px' }} onClick={handleSubmit}>Submit</Button>
      </CardBody>
    </Card>
  );
}

export default CreateProduct;

