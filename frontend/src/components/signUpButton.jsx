import React from 'react'
import { apiAuth } from '../api/apiAuth'
import { Link } from '@nextui-org/react'

function SignUpButton() {
    try{
        const token = getTokenFromLocalStorage();
        if (token) {
            return (
                <div>
                    <button>Sign Up</button>
                </div>
            )
        }
    }
    catch (error) {
        console.log(error)
    }
        return (
            <Link href="#">Login</Link>
        )
}

export default SignUpButton