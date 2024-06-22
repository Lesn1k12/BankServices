import Image from "next/image";
import Link from "next/link";
import { Button } from "@nextui-org/react";


export default function Home() {
    return (
      <Button>
        <Link href="/auth">
          Login
        </Link>
      </Button>
    )
}
