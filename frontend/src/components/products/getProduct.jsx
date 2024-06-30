import React from 'react'
import { getProduct } from '../../services/productService'

function GetProduct(id) {
  let product = getProduct(id)

  return (
    <div>{product}</div>
  )
}

export default GetProduct