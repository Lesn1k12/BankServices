'use client'

import React, {useState} from 'react'
import { Button } from '@nextui-org/react'
import { updateProduct } from "@/app/api/apiProducts"

function UpdateProduct() {
  const [product, setProduct] = useState({
    name: 'name1',
    price: '1234.1234',
    category: 'category1',
    storage_country: 'country1',
    storage_region: 'region1',
    storage_street: 'street1',
    storage_quantity: '6'
  })  
    const handleSubmit = async (e) => {
    e.preventDefault()
    updateProduct(product)
  }
  return (
    <Button onClick={handleSubmit}>
      update Product
    </Button>
  )
}

export default UpdateProduct