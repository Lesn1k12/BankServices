import React from 'react'
import { deleteProduct } from "@/app/api/apiProducts"
import { Button } from '@nextui-org/react'

function DeleteProduct({id}) {
  return (
    <button onClick={() => console.log('Delete product', id)}>Delete</button>
  )
}

export default DeleteProduct