import React from "react";
import { render } from "@testing-library/react-native";
import MapTab from "../app/(tabs)/index";

it("renders MapTab without crashing", () => {
  const tree = render(<MapTab />).toJSON();
  expect(tree).toBeTruthy();
});
