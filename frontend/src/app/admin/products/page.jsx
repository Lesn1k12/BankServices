import React from 'react'
import { Button } from '@nextui-org/react'

import CreateProduct from '@/components/products/createProduct'
import ProductsTable from '@/components/products/productsTable'
import UpdateProduct from '@/components/products/updateProduct'

function products() {
  return (
    <div>
        <CreateProduct />
        <ProductsTable />
        <UpdateProduct />
    </div>
  )
}

export default products