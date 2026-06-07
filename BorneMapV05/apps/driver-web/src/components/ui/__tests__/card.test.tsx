import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { Card, CardHeader, CardContent, CardFooter } from "../card";

describe("Card", () => {
  it("renders children", () => {
    render(<Card><p>Content</p></Card>);
    expect(screen.getByText("Content")).toBeInTheDocument();
  });

  it("renders Card.Header", () => {
    render(<Card><CardHeader>Header</CardHeader></Card>);
    expect(screen.getByText("Header")).toBeInTheDocument();
  });

  it("renders Card.Content", () => {
    render(<Card><CardContent>Body</CardContent></Card>);
    expect(screen.getByText("Body")).toBeInTheDocument();
  });

  it("renders Card.Footer", () => {
    render(<Card><CardFooter>Footer</CardFooter></Card>);
    expect(screen.getByText("Footer")).toBeInTheDocument();
  });

  it("renders all subcomponents together", () => {
    render(
      <Card>
        <CardHeader>Title</CardHeader>
        <CardContent>Details</CardContent>
        <CardFooter>Actions</CardFooter>
      </Card>,
    );
    expect(screen.getByText("Title")).toBeInTheDocument();
    expect(screen.getByText("Details")).toBeInTheDocument();
    expect(screen.getByText("Actions")).toBeInTheDocument();
  });
});
