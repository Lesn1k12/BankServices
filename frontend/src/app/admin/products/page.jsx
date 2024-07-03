import React from 'react'
import { Button } from '@nextui-org/react'
import {ProductProvider} from '@/app/api/apiProducts'

import CreateProduct from '@/components/products/createProduct'
import ProductsTable from '@/components/products/productsTable'
import GetProduct from '@/components/products/getProduct'
import DeleteProduct from '@/components/products/deleteProduct'
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