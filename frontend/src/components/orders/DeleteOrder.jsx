import React from 'react'
import { delete_order } from '@/app/api/apiOrders'

function DeleteOrder({id}) {

    const handleDelete = async () => {
        await delete_order(id);
    }

  return (
    <div>
        <button onClick={handleDelete}>Delete Order</button>
    </div>
  )
}

export default DeleteOrder