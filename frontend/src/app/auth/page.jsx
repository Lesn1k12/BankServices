"use client";

import React from "react";
import { Tabs, Tab, Card, CardBody, Link } from "@nextui-org/react";

import Login from "../../components/login";
import Register from "../../components/register";

function authentication() {
  const [selected, setSelected] = React.useState("login");

  return (
    <div className="flex items-center justify-center w-full h-screen">
      <Card className="max-w-full w-[340px] h-[400px]">
        <CardBody className="overflow-hidden">
          <Tabs
            fullWidth
            size="md"
            aria-label="Tabs form"
            selectedKey={selected}
            onSelectionChange={setSelected}
          >
            <Tab key="login" title="Login">
              <Login />
              <p className="text-center text-small">
                Need to create an account?{" "}
                <Link size="sm" onPress={() => setSelected("sign-up")}>
                  Sign up
                </Link>
              </p>
            </Tab>
            <Tab key="sign-up" title="Sign up">
              <Register />
              <p className="text-center text-small">
                Already have an account?{" "}
                <Link size="sm" onPress={() => setSelected("login")}>
                  Login
                </Link>
              </p>
            </Tab>
          </Tabs>
        </CardBody>
      </Card>
    </div>
  );
}

export default authentication;
