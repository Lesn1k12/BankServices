'use client'

import React, { useState } from 'react'
import { createProduct } from "@/app/api/apiProducts"

import { Button } from '@nextui-org/react'

function CreateProduct() {
  const [product, setProduct] = useState({
    name: 'name2',
    price: 123.123,
    category: 'category',
    storage_country: 'USA',
    storage_region: 'California',
    storage_street: 'street',
    storage_quantity: 5
  })  
    const handleSubmit = async (e) => {
    e.preventDefault()
    createProduct(product)
  }
  return (
    <Button onClick={handleSubmit}>
      Create Product
    </Button>
  )
}

export default CreateProduct
