import React from "react";
import CreateOrder from "@/components/orders/CreateOrder";
import GetAllOrders from "@/components/orders/GetAllOrders";

function ClientPage() {
  const products = [
    {
      id: 1,
      name: "Product 1",
      description: "Description for Product 1",
      price: "$10",
    },
    {
      id: 2,
      name: "Product 2",
      description: "Description for Product 2",
      price: "$20",
    },
    {
      id: 3,
      name: "Product 3",
      description: "Description for Product 3",
      price: "$30",
    },
  ];

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <header className="bg-white shadow p-4 mb-8">
        <Navbar isBordered variant="floating">
          <Navbar.Brand>
            <Text b color="inherit" hideIn="xs">
              Product Company
            </Text>
          </Navbar.Brand>
          <Navbar.Content hideIn="xs">
            <Navbar.Link href="#">Home</Navbar.Link>
            <Navbar.Link href="#">Products</Navbar.Link>
            <Navbar.Link href="#">About Us</Navbar.Link>
            <Navbar.Link href="#">Contact</Navbar.Link>
          </Navbar.Content>
          <Navbar.Content>
            <Navbar.Link color="inherit" href="#">
              Login
            </Navbar.Link>
            <Button auto flat as={Link} href="#">
              Sign Up
            </Button>
          </Navbar.Content>
        </Navbar>
      </header>

      <main className="container mx-auto">
        <Grid.Container gap={2} justify="center">
          {products.map((product) => (
            <Grid xs={12} sm={4} key={product.id}>
              <Card>
                <Card.Body>
                  <Text h3>{product.name}</Text>
                  <Text>{product.description}</Text>
                  <Text b>{product.price}</Text>
                </Card.Body>
                <Card.Footer>
                  <Button shadow color="primary" auto>
                    Buy Now
                  </Button>
                </Card.Footer>
              </Card>
            </Grid>
          ))}
        </Grid.Container>
      </main>

      <footer className="bg-white shadow p-4 mt-8">
        <div className="text-center text-gray-600">
          © 2024 Product Company. All rights reserved.
        </div>
      </footer>
    </div>
  );
}

export default ClientPage;
