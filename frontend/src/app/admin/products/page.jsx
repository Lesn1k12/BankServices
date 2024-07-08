import React from 'react'
import { Button } from '@nextui-org/react'

import CreateProduct from '@/components/products/createProduct'
import ProductsTable from '@/components/products/productsTable'
import UpdateProduct from '@/components/products/updateProduct'

export const displayData = (id, component) => {
  switch (component) {
    case 'create':
      return <CreateProduct />
    case 'update':
      return <UpdateProduct id={id} />
    default:
      return null
  }
}

function products() {
 

  return (
    <div className='flex flex-row gap-3'>
        <CreateProduct />
        <ProductsTable />
    </div>
  )
}

export default products