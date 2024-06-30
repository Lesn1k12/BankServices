"use client"
import Image from "next/image";
import Link from "next/link";
import { Button } from "@nextui-org/react";
import axios from "axios";


export default function Home() {
    const index = async () => {
        try {
            const response = await axios.get('http://localhost:8080/api/index');
            console.log(response.data);
        } catch (error) {
            console.error(error);
        }
    };

    return (
      <>
        <Button>
          <Link href="/auth">
            Login
          </Link>
        </Button>
        <Button>
          <Link href="/products">
            Products
          </Link>
        </Button>
        <Button onClick = {index}>
          index
        </Button>
      </>
    )
}
