'use client'

import React, {useState} from 'react'
import { getProduct } from '@/app/api/apiProducts'
import { Button } from '@nextui-org/react'

function GetProduct(id) {
  const [product, setProduct] = useState({
    name: 'name',
    price: '123.123',
    category: 'category',
    storage_country: 'country',
    storage_region: 'region',
    storage_street: 'street',
    storage_quantity: '5'
  })

  const handlerSubmit = async () => {
    const product = await getProduct(id)
    setProduct(product)
  }

  return (
    <>
      <Button onClick={() => handlerSubmit()}>get</Button>
      <div>{product}</div>
    </>
  )
}

export default GetProduct