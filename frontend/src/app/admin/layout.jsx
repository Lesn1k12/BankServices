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

export default function ProductsLayout({ children }) {
  return (
    <div className="flex flex-col h-screen">
      <header className="h-[10vh]">
        <Navbar isBordered isBlurred={false}>
          <NavbarBrand>
            <p className="font-bold text-inherit">ACME</p>
          </NavbarBrand>
          <NavbarContent className="hidden sm:flex gap-4" justify="center">
            <NavbarItem>
              <Link color="foreground" href="#">
                Features
              </Link>
            </NavbarItem>
            <NavbarItem isActive>
              <Link href="#" aria-current="page">
                Customers
              </Link>
            </NavbarItem>
            <NavbarItem>
              <Link color="foreground" href="#">
                Integrations
              </Link>
            </NavbarItem>
          </NavbarContent>
          <NavbarContent justify="end">
            <NavbarItem className="hidden lg:flex">
              <Link href="#">Login</Link>
            </NavbarItem>
            <NavbarItem>
              <Button as={Link} color="primary" href="#" variant="flat">
                Sign Up
              </Button>
            </NavbarItem>
          </NavbarContent>
        </Navbar>
      </header>
      <main className="flex flex-row h-[90vh]">
        <div className="flex-none w-64 mt-2">
          <Sidebar />
        </div>
        <Divider orientation="vertical" className="flex-none" />
        <div className="flex-grow p-4 overflow-auto"> 
          {children}
        </div>
      </main>
    </div>
  );
}
