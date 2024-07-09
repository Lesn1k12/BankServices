"use client";
import Image from "next/image";
import Link from "next/link";
import { Button } from "@nextui-org/react";
import axios from "axios";
import { useState } from "react";

import CreateProduct from "@/components/products/createProduct";
import ProductsTable from "@/components/products/productsTable";
import Sidebar from "@/components/sidebar";

export default function Home() {
  return (
    <>
      <Button>
        <Link href="/auth">Login</Link>
      </Button>
      <Button>
        <Link href="/admin/products">Products</Link>
      </Button>
      <Button>index</Button>
    </>
  );
}
