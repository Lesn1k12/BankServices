"use client";
import React from "react";
import { Button, Input } from "@nextui-org/react";
import Link from "next/link";

function Sidebar() {
  const services = [
    {
      name: "Products",
      link: "/admin/products",
    },
    {
      name: "Customers",
      link: "/admin/customers",
    },
    {
      name: "Orders",
      link: "/admin/orders",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
    {
      name: "Tasks",
      link: "/admin/tasks",
    },
  ];

  return (
    <div className="w-64 h-screen bg-gray-800 text-white p-6 flex flex-col">
      <div className="flex flex-col gap-y-0.5 mr-2 ml-2 mt-3 space-y-1">
        {services.map((service) => (
          <Link
            key={service.name}
            href={service.link}
            className="py-2 px-4 rounded hover:bg-gray-700"
            passHref
          >
            <Button color="primary" variant="light">{service.name}</Button>
          </Link>
        ))}
      </div>
    </div>
  );
}

export default Sidebar;
