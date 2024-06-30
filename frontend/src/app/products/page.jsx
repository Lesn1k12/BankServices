import React from 'react'
import { Button } from '@nextui-org/react'
import {ProductProvider} from '@/app/api/apiProducts'

import CreateProduct from '@/components/products/createProduct'
import ProductsTable from '@/components/products/productsTable'

function products() {
  return (
    <div>
        <CreateProduct />
        <ProductsTable />
    </div>
  )
}

export default products