'use client';

import { useEffect } from 'react';

export default function Error({ error }) {

    useEffect(() => {
        console.error('Error:', error);
    }, [error]);

    return (
        <div>
            <h1>Error</h1>
            <p>{error.message}</p>
        </div>
    );
}