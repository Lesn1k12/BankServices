'use client'
import React, {useEffect} from 'react'
import { get_all_orders } from '@/app/api/apiOrders'
import GetProduct from './GetProduct'
import DeleteOrder from './DeleteOrder'

function GetAllOrders() {
    const fetchAllOrders = async () => {
        await get_all_orders();
    }
    useEffect(() => {
        const data = fetchAllOrders();
    }, [])

  return (
    <div>{data}</div>
  )
}

export default GetAllOrders