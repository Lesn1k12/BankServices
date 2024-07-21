import React from 'react'
import {getOrder} from '@/app/api/apiOrders'

function GetOrder({id}) {
    const fetchOrder = async () => {
        await getOrder(id);
    }
  return (
    <div className="">
        <button onClick={fetchOrder}>
            get order
        </button>
    </div>
  )
}

export default GetOrder