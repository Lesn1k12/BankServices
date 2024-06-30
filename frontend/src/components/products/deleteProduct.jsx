import React from 'react'
import { deleteProduct } from "@/app/api/apiProducts"
import { Button } from '@nextui-org/react'

function DeleteProduct(id) {
  return (
    <Button onClick={() => deleteProduct(id)}>delete</Button>
  )
}

export default DeleteProduct