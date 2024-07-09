import Sidebar from "@/components/sidebar";
import { Divider } from "@nextui-org/react";
import {
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Link,
  Button,
} from "@nextui-org/react";
import { SignUpButton } from "@/components/signUpButton";

export default function ProductsLayout({ children }) {
  return (
    <div className="flex flex-col h-screen">
      <header className="h-[10vh]">
        <Navbar isBordered>
          <NavbarBrand>
            <p className="font-bold text-inherit">Logistics service</p>
          </NavbarBrand>
          <NavbarContent justify="end">
            <NavbarItem className="hidden lg:flex">
              <SignUpButton />
            </NavbarItem>
          </NavbarContent>
        </Navbar>
      </header>
      <main className="flex flex-row h-[90vh]">
        <div className="flex-none w-64 mt-2">
          <Sidebar />
        </div>
        <Divider orientation="vertical" className="flex-none" />
        <div className="flex-grow p-4 overflow-auto">{children}</div>
      </main>
    </div>
  );
}
